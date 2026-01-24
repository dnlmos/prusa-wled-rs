mod models;

use crate::models::{JobResponse, Progress, endpoints};
use anyhow::Result;
use reqwest::Client;
use std::{env, time::Duration};
use tokio::{
    signal,
    time::{MissedTickBehavior, interval},
};

#[derive(Debug, Clone)]
struct Config {
    printer_ip: String,
    wled_ip: String,
    api_key: String,
    interval: Duration,
    client: Client,
}

/// Core body of one iteration.
async fn run_once(cfg: &Config) -> Result<()> {
    let url = format!("{}{}", cfg.printer_ip, endpoints::JOB);

    let resp: JobResponse = cfg
        .client
        .get(url)
        .header("X-Api-Key", &cfg.api_key)
        .send()
        .await?
        .json()
        .await?;

    // println!("Parsed Version:\n{:?}", resp);
    if let Progress::Mk3(progress) = resp.progress
        && let Some(completion) = progress.common.completion
    {
        // compute ix (intensity for wled effect)
        let ix = 200.0 - (0.35 * completion * 100.0);
        println!("Progress: {}, setting WLED intensity to {}", completion, ix);

        let ix = ix.round().clamp(0.0, 255.0) as u8;

        // fx = effect ID (98), ix = intensity.
        // 200 → off, 198 → first 3 LEDs, 196 → 6 LEDs, …, 165 → all LEDs.
        let payload = format!(
            r#"{{
  "on": true,
  "seg": [{{
    "fx": 98,
    "ix": {}
  }}]
}}"#,
            ix
        );

        let resp = cfg
            .client
            .post(format!("{}/json/state", cfg.wled_ip))
            .header("Content-Type", "application/json")
            .body(payload)
            .send()
            .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let cfg = Config {
        printer_ip: env::var("PRINTER_IP")?,
        wled_ip: env::var("WLED_IP")?,
        api_key: env::var("PRINTER_API_KEY")?,
        interval: Duration::from_secs(20),
        client: Client::new(),
    };

    println!("{:?}", cfg);

    let mut timer = interval(cfg.interval);
    timer.set_missed_tick_behavior(MissedTickBehavior::Delay);

    println!("Service started. Press Ctrl+C to stop.");

    loop {
        tokio::select! {
            _ = timer.tick() => {
                // Run the job concurrently with the shutdown signal.
                tokio::select! {
                    // The actual work.
                    result = run_once(&cfg) => {
                        if let Err(e) = result {
                            eprintln!("iteration failed: {:#}", e);
                            // optional: break; // stop the service on error
                        }
                    }

                    // A shutdown while the job is still running.
                    _ = signal::ctrl_c() => {
                        println!("Shutting down (while job running)");
                        break;
                    }
                }
            }

            // Immediate shutdown (no job in progress)
            _ = signal::ctrl_c() => {
                println!("Shutting down");
                break;
            }
        }
    }

    Ok(())
}
