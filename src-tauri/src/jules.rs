use std::fs;
use std::io::{Write, BufRead, BufReader, Read};
use std::process::{Command, Stdio};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

pub fn model_exists(model_path: &str) -> bool {
  std::path::Path::new(&format!("{}/model.gguf", model_path)).exists()
}

pub async fn download_model(model_path: &str, model_url: &str) -> Result<(), Box<dyn std::error::Error>> {
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

pub async fn invoke_llama_cli(prompt: &str, stream: bool) -> Result<Option<BufReader<std::process::ChildStdout>>, Box<dyn std::error::Error>> {
  /** reference prompt
  ./llama-cli -m qwen2-1_5b-instruct-q5_k_m.gguf \
  -n 512 -co -i -if -f prompts/chat-with-qwen.txt \
  --in-prefix "<|im_start|>user\n" \
  --in-suffix "<|im_end|>\n<|im_start|>assistant\n" \
  -ngl 28 -fa
  */

  // this path needs to be fixed to be relevant to a built tauri app
  // this path needs to be fixed to be relevant to a built tauri app
  let mut child = Command::new("./binaries/llama-cli-aarch64-apple-darwin")
    .args(&[
      "-m", "models/model.gguf",
      "-p", &format!("<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n", prompt),
      "-n", "1000",
      "--reverse-prompt", "<|im_end|>",
      "-ngl", "28",  // GPU acceleration if available
      "-fa",         // flash attention optimization
      "-e"           // end-of-text handling
    ])
    .stdout(Stdio::piped())
    .stderr(Stdio::null())
    .stdin(Stdio::null())
    .spawn()?;

  let stdout = child.stdout.take().unwrap();

  let mut aggregated_output = String::new();

  if stream {
    // Stream mode: read character by character for real-time output
    // use std::io::Read;
    // let mut reader = stdout;
    // let mut buffer = [0; 1]; // Read one byte at a time

    // while let Ok(bytes_read) = reader.read(&mut buffer) {
    //   if bytes_read == 0 {
    //     break; // EOF
    //   }

    //   let ch = buffer[0] as char;
    //   print!("{}", ch); // Print each character immediately
    //   std::io::stdout().flush()?; // Force immediate output
    //   aggregated_output.push(ch);
    // }
    return Ok(Some(BufReader::new(stdout)));
  } else {
    // Non-stream mode: collect all output first
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
      let line = line?;
      aggregated_output.push_str(&line);
      aggregated_output.push('\n');
    }
  }

  let status = child.wait()?;

  if status.success() {
    if !stream {
      println!("Process output: {}", aggregated_output);
    }
  } else {
    eprintln!("Process failed");
  }

  Ok(None)
}
