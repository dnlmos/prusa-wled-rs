mod models;

use crate::models::{ConnectionResponse, JobResponse, Progress, endpoints};
use anyhow::Result;
use reqwest::Client;
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
    client: Client,
}

#[derive(Debug, PartialEq)]
enum State {
    NoConnection,
    Connected,
    JobActive,
}

async fn update_wled(completion: f64, cfg: &Config) -> Result<()> {
    // percentage effect intensity (200 is 0, every two down will light up next segment)
    let ix = 200.0 - (0.35 * completion * 100.0);
    println!(
        "Progress: {:.1}%, setting WLED intensity to {:.0}",
        completion * 100.0,
        ix
    );

    let ix = ix.round().clamp(0.0, 255.0) as u8;

    // percentage effect, pursa orange color
    let payload = format!(
        r#"{{
        "on": true,
        "seg": [{{
            "ix":{},
            "fx":98,
            "col":[[234,94,26]]
        }}]
    }}"#,
        ix
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
    let resp = cfg
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
        client: Client::new(),
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
        };

        // Wait for next tick or Ctrl+C
        tokio::select! {
            _ = tokio::time::sleep(sleep_duration) => {
                // State machine: determine next state and action

                match state {
                    State::NoConnection => {
                        print!("Checking connection... ");
                        match check_connection(&cfg).await {
                            Ok(true) => {
                                println!("✓ Connected");
                                state = State::Connected;
                            }
                            Ok(false) | Err(_) => {
                                println!("✗ No connection");
                            }
                        }
                    }

                    State::Connected => {
                        print!("Checking for job... ");
                        match get_job_progress(&cfg).await {
                            Ok(Some(completion)) => {
                                println!("✓ Job found at {:.1}%", completion * 100.0);

                                // Save WLED state on job start
                                if saved_wled_state.is_none() {
                                    saved_wled_state = fetch_wled_state(&cfg).await.ok();
                                    println!("WLED state saved");
                                }

                                // Update WLED
                                if let Err(e) = update_wled(completion, &cfg).await {
                                    eprintln!("Failed to update WLED: {:#}", e);
                                }

                                state = State::JobActive;
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

                    State::JobActive => {
                        match get_job_progress(&cfg).await {
                            Ok(Some(completion)) => {
                                // Job still active, update WLED
                                if let Err(e) = update_wled(completion, &cfg).await {
                                    eprintln!("Failed to update WLED: {:#}", e);
                                }
                            }
                            Ok(None) => {
                                println!("Job completed!");

                                // Restore WLED state
                                if let Some(wled_state) = saved_wled_state.take()
                                    && let Err(e) = restore_wled_state(&cfg, wled_state).await {
                                        eprintln!("Failed to restore WLED: {:#}", e);
                                }

                                state = State::Connected;
                            }
                            Err(e) => {
                                eprintln!("Connection lost: {:#}", e);

                                // Restore WLED state
                                if let Some(wled_state) = saved_wled_state.take()
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
                if let Some(wled_state) = saved_wled_state.take() {
                    restore_wled_state(&cfg, wled_state).await.ok();
                }

                break;
            }
        }
    }

    Ok(())
}
