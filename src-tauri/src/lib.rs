mod pdf_generator;

use pdf_generator::{generate_invoice_pdf, InvoiceData};
use std::path::PathBuf;

#[tauri::command]
fn generate_invoice(invoice_data: InvoiceData, file_path: String) -> Result<String, String> {
    let path = PathBuf::from(&file_path);

    // Ensure the directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Generate the PDF
    generate_invoice_pdf(&invoice_data, &path)?;

    Ok(file_path)
}

#[tauri::command]
fn open_pdf(file_path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| format!("Failed to open PDF: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &file_path])
            .spawn()
            .map_err(|e| format!("Failed to open PDF: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| format!("Failed to open PDF: {}", e))?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![generate_invoice, open_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
