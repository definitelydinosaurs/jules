mod jules;

use jules::{model_exists, download_model, invoke_llama_cli};

#[tauri::command]
async fn prompt(prompt: String) {
  match invoke_llama_cli(&prompt, true).await {
    Ok(_) => println!("External process executed successfully"),
    Err(e) => eprintln!("Error executing external process: {}", e),
  };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
