use reqwest::Client;
use std::time::Instant;
use tokio::runtime::Runtime;
use clap::{Command};

const TEST_URL: &str = "http://ipv4.download.thinkbroadband.com/100MB.zip";

async fn measure_download_speed(url: &str) -> Result<f64, reqwest::Error> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let mut response = client.get(url).send().await?;
    let content_length = response.content_length().unwrap_or(0);

    let _buffer = vec![0u8; 1024 * 1024];

    // start the timer
    let start = Instant::now();

    // read the response in chunks
    while let Some(_chunk) = response.chunk().await? {
        // process the chunk or measure progress if needed
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
        .version("1.1")
        .about("Measures internet speed")
        .get_matches();


    let runtime = Runtime::new().unwrap();

    println!("Running Internet Speed Test...");

    match runtime.block_on(measure_download_speed(TEST_URL)) {
        Ok(speed) => println!("Download Speed: {:.2} Mbps", speed),
        Err(err) => eprintln!("Error measuring download speed: {}", err),
    }
}
