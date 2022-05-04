
use clap::Parser;



#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[clap(short, long, default_value = "0.1.1")]
    version: String,
}

use std::cmp::min;
use std::fs::File;
use std::io::Write;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {

    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;


    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{ProgressStyle::default_spinner} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("☭>-"));
    pb.set_message(format!("Downloading {}", url));


    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", url, path));
    return Ok(());
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let version = args.version;
    let name = "/soviet-grub-".to_owned() + &version + ".iso";
    let url = "https://sovietlinux.ml/iso/".to_owned() + &version + "/" + &name;
    let path = "../".to_owned() + &name;

    

    download_file(
        &Client::new(),
        &url,
        &path,
    )
    .await
    .unwrap();
}