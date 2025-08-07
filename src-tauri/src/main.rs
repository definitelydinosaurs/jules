// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() > 1 {
    println!("Hello World");
  } else {
    app_lib::run();
  }
}
