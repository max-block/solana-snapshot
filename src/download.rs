use std::time::Duration;
use std::{cmp::min, fs::File, io::Write};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

use crate::model::Node;

async fn download_file_async(url: &str, path: &str) -> Result<(), String> {
    let res = Client::new().get(url).send().await.or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res.content_length().ok_or(format!("Failed to get content length from '{}'", &url))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url));

    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk).or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", url, path));
    Ok(())
}

pub fn download_file(url: &str, path: &str) -> Result<(), String> {
    tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(download_file_async(url, path))
}

async fn measure_download_speed_async(node: &mut Node, measure_time: u64) -> Result<(), String> {
    let client = Client::builder().timeout(Duration::from_secs(measure_time)).build().unwrap();
    let res = client
        .get(format!("http://{}/snapshot.tar.bz2", node.rpc))
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", node.rpc)))?;

    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        node.measure_download += chunk.len() as u64;
    }
    Ok(())
}

pub fn measure_download_speed(node: &mut Node, measure_time: u64) -> Result<(), String> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(measure_download_speed_async(node, measure_time))
}
