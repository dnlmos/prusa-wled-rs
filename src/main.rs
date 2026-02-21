mod colors;
mod logger;
mod models;
use log::{LevelFilter, debug, info, warn};
use logger::init;

use crate::{
    colors::Color,
    models::{ConnectionResponse, JobResponse, PrinterResponse, Progress, endpoints},
};
use anyhow::{Context, Result};
use reqwest::blocking::{Client, ClientBuilder};
use std::{env, time::Duration};

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

#[derive(Debug, Clone)]
struct WledState {
    raw_json: String,
}

#[derive(Debug, PartialEq)]
enum State {
    NoConnection,
    Connected,
    JobActive,
    Heating,
}

fn update_wled(completion: f64, color: Color, cfg: &Config) -> Result<()> {
    // percentage effect intensity (200 is 0, every two down will light up next segment)
    let ix = 200.0 - (0.35 * completion * 100.0);

    debug!(
        "Progress: {:.1}, setting WLED intensity to {:.0} | Code {:?}",
        completion, ix, color
    );

    let ix = ix.round().clamp(0.0, 255.0) as u8;

    // percentage effect (98), color assigned based on the state
    let payload = format!(
        r#"{{
        "on": true,
        "seg": [{{
            "ix":{},
            "fx":98,
            "pal":0,
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
        .send()?;

    Ok(())
}

fn fetch_wled_state(cfg: &Config) -> Result<WledState> {
    let resp = cfg
        .client
        .get(format!("{}/json/state", cfg.wled_ip))
        .send()?
        .text()?;

    Ok(WledState { raw_json: resp })
}

fn restore_wled_state(cfg: &Config, state: WledState) -> Result<()> {
    info!("Restoring WLED state");
    let _resp = cfg
        .client
        .post(format!("{}/json/state", cfg.wled_ip))
        .body(state.raw_json)
        .send()?;

    Ok(())
}

fn check_connection(cfg: &Config) -> Result<bool> {
    let _connection: ConnectionResponse = cfg
        .client
        .get(format!("{}{}", cfg.printer_ip, endpoints::CONNECTION))
        .header("X-Api-Key", &cfg.api_key)
        .send()?
        .json()?;
    Ok(true)
}

fn get_job_progress(cfg: &Config) -> Result<Option<f64>> {
    let resp: JobResponse = cfg
        .client
        .get(format!("{}{}", cfg.printer_ip, endpoints::JOB))
        .header("X-Api-Key", &cfg.api_key)
        .send()?
        .json()?;

    if let Progress::Mk3(progress) = resp.progress {
        Ok(progress.common.completion)
    } else {
        Ok(None)
    }
}

fn get_printer_status(cfg: &Config) -> Result<PrinterResponse> {
    let resp: PrinterResponse = cfg
        .client
        .get(format!("{}{}", cfg.printer_ip, endpoints::PRINTER))
        .header("X-Api-Key", &cfg.api_key)
        .send()?
        .json()?;
    Ok(resp)
}

fn get_heating_progress(cfg: &Config) -> Result<f64> {
    let status = get_printer_status(cfg).context("Failed to retrieve printer status")?;

    let temps = &status.temperature;

    let actual = temps.tool0.actual + temps.bed.actual;
    let target = temps.tool0.target + temps.bed.target;

    if target <= 0.0 {
        return Ok(0.0);
    }

    Ok(actual / target)
}

fn main() -> Result<()> {
    if let Err(e) = init(LevelFilter::Info) {
        eprintln!("Failed to initialize logger: {}", e);
    }
    let pid = std::process::id();
    info!("Process ID: {}", pid);

    dotenv::dotenv().ok();
    let cfg = Config {
        printer_ip: env::var("PRINTER_IP")
            .context("Variable 'PRINTER_IP' not found in .env file or is not unicode")?,
        wled_ip: env::var("WLED_IP")
            .context("Variable 'WLED_IP' not found in .env file or is not unicode")?,
        api_key: env::var("PRINTER_API_KEY")
            .context("Variable 'PRINTER_API_KEY' not found in .env file or is not unicode")?,
        connection_check_interval: Duration::from_secs(5),
        job_check_interval: Duration::from_secs(10),
        progress_update_interval: Duration::from_secs(10),
        heating_update_interval: Duration::from_secs(5),
        client: ClientBuilder::new()
            .timeout(Duration::from_secs(5))
            .build()?,
    };

    info!("Service started. Press Ctrl+C to stop.");

    let mut state = State::NoConnection;
    let mut saved_wled_state: Option<WledState> = None;

    // finite state automata
    loop {
        let sleep_duration = match state {
            State::NoConnection => cfg.connection_check_interval,
            State::Connected => cfg.job_check_interval,
            State::JobActive => cfg.progress_update_interval,
            State::Heating => cfg.heating_update_interval,
        };

        std::thread::sleep(sleep_duration);

        match state {
            State::NoConnection => {
                info!("Checking connection... ");
                match check_connection(&cfg) {
                    Ok(true) => {
                        info!("Connected");
                        state = State::Connected;

                        if saved_wled_state.is_none() {
                            match fetch_wled_state(&cfg) {
                                Ok(fetched_state) => {
                                    saved_wled_state = Some(fetched_state);
                                    info!("WLED state saved");
                                }
                                Err(e) => warn!("Failed to fetch WLED state: {:#}", e),
                            }
                        }

                        if let Err(e) = update_wled(1.0, Color::Operational, &cfg) {
                            warn!("Failed to update WLED: {:#}", e);
                        }
                    }
                    Ok(false) | Err(_) => {
                        info!("No connection");
                        if let Some(wled_state) = saved_wled_state.clone() {
                            match restore_wled_state(&cfg, wled_state) {
                                Ok(_) => {
                                    saved_wled_state = None;
                                    info!("WLED State restored")
                                }
                                Err(e) => warn!("Failed to restore WLED state: {:#}", e),
                            }
                        }
                    }
                }
            }

            State::Connected => {
                info!("Checking for job... ");
                match get_job_progress(&cfg) {
                    Ok(Some(completion)) => {
                        info!("Job found at {}", completion);
                        state = match get_heating_progress(&cfg) {
                            //
                            Ok(p) if p < 0.97 => {
                                info!("Heating mode detected");
                                State::Heating
                            }
                            _ => {
                                info!("Heating completed, print started");
                                State::JobActive
                            }
                        };
                    }
                    Ok(None) => {
                        info!("No active job");
                    }
                    Err(e) => {
                        warn!("Connection lost: {:#}", e);
                        state = State::NoConnection;
                    }
                }
            }

            State::Heating => match get_heating_progress(&cfg) {
                Ok(heating_progress) => {
                    info!("Heating progress: {}", heating_progress);
                    if let Err(e) = update_wled(heating_progress, Color::Heating, &cfg) {
                        warn!("Failed to update WLED: {:#}", e);
                    }
                    if heating_progress > 0.90 {
                        state = State::JobActive
                    }
                }
                Err(e) => {
                    warn!("Failed to fetch heating progress: {:#}", e);
                }
            },

            State::JobActive => match get_job_progress(&cfg) {
                Ok(Some(completion)) => {
                    if let Err(e) = update_wled(completion, Color::Printing, &cfg) {
                        warn!("Failed to update WLED: {:#}", e);
                    }
                }
                Ok(None) => {
                    info!("Job completed!");
                    if let Err(e) = update_wled(1.0, Color::Finished, &cfg) {
                        warn!("Failed to update WLED: {:#}", e);
                    }
                    state = State::Connected;
                }
                Err(e) => {
                    warn!("Connection lost: {:#}", e);

                    if let Some(wled_state) = saved_wled_state.clone()
                        && let Err(e) = restore_wled_state(&cfg, wled_state)
                    {
                        warn!("Failed to restore WLED: {:#}", e);
                    }

                    state = State::NoConnection;
                }
            },
        }
    }
}
