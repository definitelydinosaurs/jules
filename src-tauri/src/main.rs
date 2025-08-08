// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn model_exists(model_path: &str) -> bool {
  std::path::Path::new(&format!("{}/model.gguf", model_path)).exists()
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
