use reqwest::Client;
use std::time::Instant;
use tokio::runtime::Runtime;
use clap::{Command};

const TEST_URL: &str = "http://ipv4.download.thinkbroadband.com/200MB.zip";

async fn measure_download_speed(url: &str) -> Result<f64, reqwest::Error> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let mut response = client.get(url).send().await?;
    let content_length = response.content_length().unwrap_or(0);

    let _buffer = vec![0u8; 1024 * 1024];

    // start the timer
    let start = Instant::now();

    // variable to track the downloaded bytes
    let mut downloaded_bytes = 0;

    // read the response in chunks
    while let Some(chunk) = response.chunk().await? {
        downloaded_bytes += chunk.len();
        if content_length > 0 {
            let progress = (downloaded_bytes as f64 / content_length as f64) * 100.0;
            println!("Download Progress: {:.2}%", progress);
        }
    }

    // calculate elapsed time
    let elapsed = start.elapsed().as_secs_f64();

    // convert bytes to megabits and calculate speed in mbps
    let speed_mbps = (content_length as f64 * 8.0) / (elapsed * 1_000_000.0);

    Ok(speed_mbps)
}

// main function
fn main() {
    let _matches = Command::new("speedtest")
        .version("1.2.0")
        .about("Measures internet speed")
        .get_matches();

    let runtime = Runtime::new().unwrap();

    println!("Running Internet Speed Test...");

    match runtime.block_on(measure_download_speed(TEST_URL)) {
        Ok(speed) => println!("Download Speed: {:.2} Mbps", speed),
        Err(err) => eprintln!("Error measuring download speed: {}", err),
    }
}
