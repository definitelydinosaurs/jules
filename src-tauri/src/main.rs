// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Write;
use std::process::Command;
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

async fn invoke_llama_cli(prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
  /** reference prompt
  ./llama-cli -m qwen2-1_5b-instruct-q5_k_m.gguf \
  -n 512 -co -i -if -f prompts/chat-with-qwen.txt \
  --in-prefix "<|im_start|>user\n" \
  --in-suffix "<|im_end|>\n<|im_start|>assistant\n" \
  -ngl 28 -fa
  */

  // this path needs to be fixed to be relevant to a built tauri app
  let output = Command::new("./binaries/llama-cli-aarch64-apple-darwin")
    .args(&[
      "-m", "models/model.gguf",
      "-p", &format!("<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n", prompt),
      "-n", "512",
      "--reverse-prompt", "<|im_end|>",
      "-ngl", "28",  // GPU acceleration if available
      "-fa",         // flash attention optimization
      "-e"           // end-of-text handling
    ])
    .output()?;

  if output.status.success() {
    println!("Process output: {}", String::from_utf8_lossy(&output.stdout));
  } else {
    eprintln!("Process failed: {}", String::from_utf8_lossy(&output.stderr));
  }

  Ok(())
}

#[tokio::main]
async fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() > 1 {
    if !model_exists("models") {
      if let Err(e) = download_model("models", "https://huggingface.co/Qwen/Qwen2-1.5B-Instruct-GGUF/resolve/main/qwen2-1_5b-instruct-q4_0.gguf?download=true").await {
        eprintln!("Error downloading model: {}", e);
        std::process::exit(1);
      }
    }

    // Check if --stream flag is present
    let stream = args.contains(&"--stream".to_string());

    // Find the prompt (first non-flag argument)
    let prompt = args.iter()
      .skip(1)
      .find(|arg| !arg.starts_with("--"))
      .map(|s| s.as_str())
      .unwrap_or("");

    // pass arg as query to invoke_llama_cli
    match invoke_llama_cli(prompt).await {
      Ok(_) => println!("External process executed successfully"),
      Err(e) => eprintln!("Error executing external process: {}", e),
    }

    println!("Hello World");
  } else {
    app_lib::run();
  }
}
