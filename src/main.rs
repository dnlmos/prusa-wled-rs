mod colors;
mod models;

use crate::{
    colors::Color,
    models::{ConnectionResponse, JobResponse, PrinterResponse, Progress, endpoints},
};
use anyhow::Result;
use reqwest::{Client, ClientBuilder};
use std::{env, time::Duration};
use tokio::signal;

#[derive(Debug, Clone)]
struct Config {
    printer_ip: String,
    wled_ip: String,
    api_key: String,
    connection_check_interval: Duration,
    job_check_interval: Duration,
    progress_update_interval: Duration,
    heating_update_interval: Duration,
    client: Client,
}

#[derive(Debug, PartialEq)]
enum State {
    NoConnection,
    Connected,
    JobActive,
    Heating,
}

async fn update_wled(completion: f64, color: Color, cfg: &Config) -> Result<()> {
    // percentage effect intensity (200 is 0, every two down will light up next segment)
    let ix = 200.0 - (0.35 * completion * 100.0);
    println!(
        "Progress: {:.1}%, setting WLED intensity to {:.0} | Code {:?}",
        completion * 100.0,
        ix,
        color
    );

    let ix = ix.round().clamp(0.0, 255.0) as u8;

    // percentage effect, pursa orange color
    let payload = format!(
        r#"{{
        "on": true,
        "seg": [{{
            "ix":{},
            "fx":98,
            "col":[{}]
        }}]
    }}"#,
        ix,
        color.rgb()
    );

    cfg.client
        .post(format!("{}/json/state", cfg.wled_ip))
        .header("Content-Type", "application/json")
        .body(payload)
        .send()
        .await?;

    Ok(())
}

#[derive(Debug, Clone)]
struct WledState {
    raw_json: String,
}

async fn fetch_wled_state(cfg: &Config) -> Result<WledState> {
    let resp = cfg
        .client
        .get(format!("{}/json/state", cfg.wled_ip))
        .send()
        .await?
        .text()
        .await?;

    Ok(WledState { raw_json: resp })
}

async fn restore_wled_state(cfg: &Config, state: WledState) -> Result<()> {
    println!("Restoring state");
    let _resp = cfg
        .client
        .post(format!("{}/json/state", cfg.wled_ip))
        .body(state.raw_json)
        .send()
        .await?;

    Ok(())
}

async fn check_connection(cfg: &Config) -> Result<bool> {
    let _connection: ConnectionResponse = cfg
        .client
        .get(format!("{}{}", cfg.printer_ip, endpoints::CONNECTION))
        .header("X-Api-Key", &cfg.api_key)
        .send()
        .await?
        .json()
        .await?;

    Ok(true)
}

async fn get_job_progress(cfg: &Config) -> Result<Option<f64>> {
    let resp: JobResponse = cfg
        .client
        .get(format!("{}{}", cfg.printer_ip, endpoints::JOB))
        .header("X-Api-Key", &cfg.api_key)
        .send()
        .await?
        .json()
        .await?;

    if let Progress::Mk3(progress) = resp.progress {
        Ok(progress.common.completion)
    } else {
        Ok(None)
    }
}

async fn get_printer_status(cfg: &Config) -> Result<PrinterResponse> {
    let resp: PrinterResponse = cfg
        .client
        .get(format!("{}{}", cfg.printer_ip, endpoints::PRINTER))
        .header("X-Api-Key", &cfg.api_key)
        .send()
        .await?
        .json()
        .await?;
    Ok(resp)
}

async fn get_heating_progress(cfg: &Config) -> Result<f64> {
    let status = get_printer_status(cfg).await?;

    let temps = &status.temperature;

    let actual = temps.tool0.actual + temps.bed.actual;
    let target = temps.tool0.target + temps.bed.target;

    if target <= 0.0 {
        return Ok(0.0);
    }

    Ok(actual / target)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let cfg = Config {
        printer_ip: env::var("PRINTER_IP")?,
        wled_ip: env::var("WLED_IP")?,
        api_key: env::var("PRINTER_API_KEY")?,
        connection_check_interval: Duration::from_secs(5),
        job_check_interval: Duration::from_secs(10),
        progress_update_interval: Duration::from_secs(10),
        heating_update_interval: Duration::from_secs(5),
        client: ClientBuilder::new()
            .timeout(Duration::from_secs(5))
            .build()?,
    };

    println!("Service started. Press Ctrl+C to stop.");

    let mut state = State::NoConnection;
    let mut saved_wled_state: Option<WledState> = None;

    loop {
        // Determine sleep duration based on current state
        let sleep_duration = match state {
            State::NoConnection => cfg.connection_check_interval,
            State::Connected => cfg.job_check_interval,
            State::JobActive => cfg.progress_update_interval,
            State::Heating => cfg.heating_update_interval,
        };

        // Wait for next tick or Ctrl+C
        tokio::select! {
            _ = tokio::time::sleep(sleep_duration) => {
                match state {
                    State::NoConnection => {
                        print!("Checking connection... ");
                        match check_connection(&cfg).await {
                            Ok(true) => {
                                println!("✓ Connected");
                                state = State::Connected;

                                // Save WLED state on job start
                                if saved_wled_state.is_none() {
                                    saved_wled_state = fetch_wled_state(&cfg).await.ok();
                                    println!("WLED state saved");
                                }

                                update_wled(1.0, Color::Operational, &cfg).await?;
                            }
                            Ok(false) | Err(_) => {
                                println!("✗ No connection");
                                if let Some(wled_state) = saved_wled_state.clone(){
                                    restore_wled_state(&cfg, wled_state).await.ok();
                                    saved_wled_state=None;
                                }
                            }
                        }
                    }

                    State::Connected => {
                        print!("Checking for job... ");

                        match get_job_progress(&cfg).await {
                            Ok(Some(completion)) => {
                                println!("✓ Job found at {:.1}%", completion * 100.0);
                                state = match get_heating_progress(&cfg).await {
                                    Ok(p) if p < 90.0 => State::Heating,
                                    _ => State::JobActive,
                                };
                            }
                            Ok(None) => {
                                println!("✗ No active job");
                            }
                            Err(e) => {
                                eprintln!("✗ Connection lost: {:#}", e);
                                state = State::NoConnection;
                            }
                        }
                    }

                    State::Heating=> {
                        let heating_progress = get_heating_progress(&cfg).await?;
                        println!("Heating progress: {}", heating_progress);
                        update_wled(heating_progress, Color::Heating, &cfg).await?;
                        if heating_progress>0.90{
                            state=State::JobActive
                        }
                    }

                    State::JobActive => {
                        println!("job active");
                        match get_job_progress(&cfg).await {
                            Ok(Some(completion)) => {
                                if let Err(e) = update_wled(completion,Color::Printing, &cfg).await {
                                    eprintln!("Failed to update WLED: {:#}", e);
                                }
                            }
                            Ok(None) => {
                                println!("Job completed!");
                                update_wled(1.0,Color::Finished,&cfg).await?;
                                state = State::Connected;
                            }
                            Err(e) => {
                                eprintln!("Connection lost: {:#}", e);

                                // Restore WLED state
                                if let Some(wled_state) = saved_wled_state.clone()
                                    && let Err(e) = restore_wled_state(&cfg, wled_state).await {
                                        eprintln!("Failed to restore WLED: {:#}", e);
                                    }

                                state = State::NoConnection;
                            }
                        }
                    }
                }
            }

            _ = signal::ctrl_c() => {
                println!("\nShutting down...");

                // Restore WLED state if a job was active
                if let Some(wled_state) = saved_wled_state {
                    restore_wled_state(&cfg, wled_state).await.ok();
                }

                break;
            }
        }
    }

    Ok(())
}
