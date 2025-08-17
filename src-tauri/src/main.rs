// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Write;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

fn model_exists(model_path: &str) -> bool {
  std::path::Path::new(&format!("{}/model.gguf", model_path)).exists()
}

async fn download_model(model_path: &str, model_url: &str) -> Result<(), Box<dyn std::error::Error>> {
  println!("Model not found. Downloading from {}...", model_url);

  // Create directory if it doesn't exist
  fs::create_dir_all(model_path)?;

  // Start the download
  let response = reqwest::get(model_url).await?;
  let total_size = response.content_length().unwrap_or(0);

  // Create progress bar
  let pb = ProgressBar::new(total_size);
  pb.set_style(ProgressStyle::default_bar()
      .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
      .progress_chars("#>-"));

  // Create file
  let mut file = fs::File::create(format!("{}/model.gguf", model_path))?;
  let mut downloaded: u64 = 0;
  let mut stream = response.bytes_stream();

  // Download with progress
  while let Some(item) = stream.next().await {
      let chunk = item?;
      file.write_all(&chunk)?;
      let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
      downloaded = new;
      pb.set_position(new);
  }

  pb.finish_with_message("Download completed!");
  println!("Model downloaded successfully!");
  Ok(())
}

#[tokio::main]
async fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() > 1 {
    if !model_exists("models/example_model") {
      if let Err(e) = download_model("models/example_model", "https://huggingface.co/Qwen/Qwen2-1.5B-Instruct-GGUF/resolve/main/qwen2-1_5b-instruct-q4_0.gguf?download=true").await {
        eprintln!("Error downloading model: {}", e);
        std::process::exit(1);
      }
    }
    println!("Hello World");
  } else {
    app_lib::run();
  }
}
