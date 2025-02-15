use indicatif::{HumanBytes, ProgressBar, ProgressStyle, ProgressDrawTarget};
use reqwest::Client;
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use url::Url;
use console::style;
use futures_util::StreamExt; // For async streaming

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

pub async fn download(target: &str, quiet_mode: bool, fname: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(target)?;
    let client = Client::new();
    let resp = client.get(url.clone()).send().await?; // Await here

    if !quiet_mode {
        println!("HTTP request sent.. {}", style(resp.status()).green());
    }

    if resp.status().is_success() {
        let headers = resp.headers();

        let ct_len = headers.get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        let ct_type = headers.get("content-type")
            .map(|v| v.to_str().unwrap_or("unknown"))
            .unwrap_or("unknown");

        if !quiet_mode {
            match ct_len {
                Some(len) => {
                    println!(
                        "Length: {} ({})",
                        style(len).green(),
                        style(HumanBytes(len)).red()
                    );
                }
                None => {
                    println!("Length: {}", style("unknown").red());
                }
            }
            println!("Type: {}", style(ct_type).green());
        }
        
        if !quiet_mode {
            println!("Saving to: {}", style(&fname).green());
        }

        let mut file = File::create(fname).await?;
        let bar = create_progress_bar(quiet_mode, &fname, ct_len);
        
        
        let mut stream = resp.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            bar.inc(chunk.len() as u64);
        }

        bar.finish();
        if !quiet_mode {
            println!("Download complete!");
        }
    }

    Ok(())
}