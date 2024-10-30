use super::types::Paths;
use crate::service::{get_csv_data, get_pdf_data, make_xl};
use tauri::AppHandle;

#[specta::specta]
#[tauri::command]
pub async fn execute(app: AppHandle, paths: Paths) -> bool {
    // 1. pdf 파일에서 데이터 획득(필터링하여, 헤더없이 데이터만 반환)
    let pdf_data = get_pdf_data(&app, &paths).await;

    // 2. csv 파일에서 데이터 획득(제목 및 헤더 포함)
    let csv_data: Vec<Vec<String>> = get_csv_data(&paths);

    // 3. 비교 및 엑셀 파일 생성
    make_xl(&paths, pdf_data, csv_data);
    println!("execute done");

    true
}
