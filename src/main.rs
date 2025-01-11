use reqwest::Client;
use std::time::Instant;
use clap::{Command};
use serde::Deserialize;

// urls to data to download and ipinfo for isp information
const TEST_URL: &str = "http://ipv4.download.thinkbroadband.com/200MB.zip";
const ISP_INFO_URL: &str = "http://ipinfo.io/json";

#[derive(Deserialize)]
struct IpInfo {
    org: Option<String>,
}

async fn get_isp_info() -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get(ISP_INFO_URL).send().await?;
    let ip_info: IpInfo = response.json().await?;
    // return ISP or "Unknown"
    Ok(ip_info.org.unwrap_or_else(|| "Unknown".to_string()))
}

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
            println!("      Download Progress: {:.2}%", progress);
        }
    }

    // calculate elapsed time
    let elapsed = start.elapsed().as_secs_f64();

    // convert bytes to megabits and calculate speed in mbps
    let speed_mbps = (content_length as f64 * 8.0) / (elapsed * 1_000_000.0);

    Ok(speed_mbps)
}

// automatically set up the runtime
#[tokio::main]
async fn main() {
    let _matches = Command::new("speedtest")
        .version("1.3.0")
        .about("Measures internet speed")
        .get_matches();

    println!("");
    println!("   Speedtest by nbrandolino");
    println!("");

    // get isp information
    match get_isp_info().await {
        Ok(isp) => println!("      ISP: {}", isp),
        Err(err) => eprintln!("      Error fetching ISP information: {}", err),
    }

    // measure download speed
    match measure_download_speed(TEST_URL).await {
        Ok(speed) => println!("      Download Speed: {:.2} Mbps", speed),
        Err(err) => eprintln!("      Error measuring download speed: {}", err),
    }
}
