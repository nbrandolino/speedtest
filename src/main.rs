use reqwest::Client;
use std::time::Instant;
use tokio::runtime::Runtime;

const testUrl: &str = "http://ipv4.download.thinkbroadband.com/100MB.zip";

async fn measureDownloadSpeed(url: &str) -> Result<f64, reqwest::Error> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let mut response = client.get(url).send().await?;
    let contentLength = response.content_length().unwrap_or(0);

    let _buffer = vec![0u8; 1024 * 1024];

    // start the timer
    let start = Instant::now();

    // read the response in chunks
    while let Some(_chunk) = response.chunk().await? {
        // process the chunk or measure progress if needed
    }

    // calculate elapsed time
    let elapsed = start.elapsed().as_secs_f64();

    // convert bytes to megabits and calculate speed in Mbps
    let speedMbps = (contentLength as f64 * 8.0) / (elapsed * 1_000_000.0);

    Ok(speedMbps)
}

// main function
fn main() {
    let runtime = Runtime::new().unwrap();

    println!("Running internet speed test...");

    match runtime.block_on(measureDownloadSpeed(testUrl)) {
        Ok(speed) => println!("Download Speed: {:.2} Mbps", speed),
        Err(err) => eprintln!("Error measuring download speed: {}", err),
    }
}
