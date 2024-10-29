use tauri::AppHandle;
use crate::service::get_pdf_table_from_file;
use super::types::{Paths, PdfTable};


#[specta::specta]
#[tauri::command]
pub async fn execute(app: AppHandle, paths: Paths) -> bool {
    let pdf_table: PdfTable = get_pdf_table_from_file(&app, &paths.pdf_path).await;
    println!("{:#?}",pdf_table);

    true
}