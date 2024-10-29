use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;
use crate::interfaces::types::PdfTable;

pub async fn get_pdf_table_from_file(app: &AppHandle, path: &str) -> PdfTable {
    let sidecar_command = app
        .shell()
        .sidecar("app")
        .unwrap()
        .arg(path);
    let output = sidecar_command.output().await.unwrap();
    let res_str = String::from_utf8(output.stdout).unwrap();
    serde_json::from_str(&res_str).expect("failed to parse json")
}