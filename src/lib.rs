use indicatif::{HumanBytes, ProgressBar, ProgressStyle, ProgressDrawTarget};
use reqwest::{Client, header::RANGE};
use tokio::{fs::File, io::{AsyncWriteExt, AsyncSeekExt, BufWriter}, task};
use std::sync::Arc;
use std::time::Duration;
use futures::stream::StreamExt;
use futures::future::join_all;
use tokio::sync::Mutex;


// Default chunk size (1 MiB)
const CHUNK_SIZE: u64 = 1024 * 1024;

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = if quiet_mode {
        ProgressBar::hidden()
    } else {
        match length {
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        }
    };

    bar.set_message(msg.to_string());
    bar.set_draw_target(ProgressDrawTarget::stderr_with_hz(10)); // Ensure it updates in place
    if length.is_some() {
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .unwrap()
                .progress_chars("=>"),
        );
    } else {
        bar.set_style(ProgressStyle::default_spinner());
    }

    bar
}

/// Download file in parallel with a progress bar
pub async fn parallel_download(
    url: &str,
    output: &str,
    num_connections: usize,
    quiet_mode: bool
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(60))
        .pool_max_idle_per_host(2)
        .build()?;

    // Get content length
    let start = 0;
    let end = 1023;
    let range_header = format!("bytes={}-{}", start, end);
    let response = client.get(url)
        .header(RANGE, range_header)
        .send()
        .await?;

    println!("Response status: {:?}", response.status());
    
    let content_length = response.headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or("Failed to get content length")?;

    println!("File size: {} bytes", content_length);

    // Create progress bar
    let progress_bar = Arc::new(create_progress_bar(quiet_mode, "Downloading", Some(content_length)));

    // Determine chunk size per connection
    let chunk_size = (content_length + num_connections as u64 - 1) / num_connections as u64;

    // Create an empty file with the full size
    let file = File::create(output).await?;
    file.set_len(content_length).await?;
    let file = Arc::new(Mutex::new(BufWriter::new(file)));

    let mut tasks = vec![];

    for i in 0..num_connections {
        let start = i as u64 * chunk_size;
        let end = (start + chunk_size - 1).min(content_length - 1);

        let client = client.clone();
        let url = url.to_string();
        let file = file.clone();
        let progress_bar = progress_bar.clone();

        let task = task::spawn(async move {
            match download_chunk(&client, &url, start, end, &file, &progress_bar).await {
                Ok(_) => println!("Chunk {} - {} downloaded", start, end),
                Err(e) => eprintln!("Error downloading chunk {}: {}", i, e),
            }
        });

        tasks.push(task);
    }

    join_all(tasks).await;

    progress_bar.finish_with_message("Download complete!");
    println!("File saved to: {}", output);
    Ok(())
}

/// Download a specific chunk of the file and update progress
async fn download_chunk(
    client: &Client,
    url: &str,
    start: u64,
    end: u64,
    file: &Arc<Mutex<BufWriter<File>>>,
    progress_bar: &Arc<ProgressBar>,
) -> Result<(), Box<dyn std::error::Error>> {
    let range_header = format!("bytes={}-{}", start, end);

    let response = client.get(url)
        .header(RANGE, range_header)
        .send()
        .await?;

    let mut stream = response.bytes_stream();

    let mut file = file.lock().await;
    file.seek(tokio::io::SeekFrom::Start(start)).await?;

    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        file.write_all(&bytes).await?;
        progress_bar.inc(bytes.len() as u64);
    }

    Ok(())
}