// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Write;

fn model_exists(model_path: &str) -> bool {
  std::path::Path::new(&format!("{}/model.gguf", model_path)).exists()
}

async fn download_model(model_path: &str, model_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Model not found. Downloading from {}...", model_url);
    
    // Create directory if it doesn't exist
    fs::create_dir_all(model_path)?;
    
    // Download the model
    let response = reqwest::get(model_url).await?;
    let bytes = response.bytes().await?;
    
    // Write to file
    let mut file = fs::File::create(format!("{}/model.gguf", model_path))?;
    file.write_all(&bytes)?;
    
    println!("Model downloaded successfully!");
    Ok(())
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() > 1 {
    if !model_exists("models/example_model") {
      println!("No Model found. Downloading...");
    } else {
      println!("Model found. Starting application...");
    }
  } else {
    app_lib::run();
  }
}
