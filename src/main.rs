mod models;
// Note the change to the blocking module
use reqwest::blocking::Client;
use std::error::Error;

use crate::models::{PrinterResponse, endpoints};

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Create a client (optional, but good for connection pooling)
    let ip = "http://192.168.0.243";
    let client = Client::new();

    // curl -s -H "X-Api-Key: -uF7x1-Uq62XoA" http://192.168.0.243/api/printer | jq .
    let resp: PrinterResponse = client
        .get(format!("{}{}", ip, endpoints::PRINTER))
        .header("X-Api-Key", "-uF7x1-Uq62XoA")
        .send()?
        .json()?;

    println!("Parsed Version: {:?}", resp);

    Ok(())
}
