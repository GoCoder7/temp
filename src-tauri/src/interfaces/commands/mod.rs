use tauri::AppHandle;
use crate::service::get_pdf_table_from_file;
use super::types::{Paths, PdfTable};


#[specta::specta]
#[tauri::command]
pub async fn execute(app: AppHandle, paths: Paths) -> bool {
    let pdf_table: PdfTable = get_pdf_table_from_file(&app, &paths.pdf_path).await;
    let data: Vec<Vec<String>> = get_pdf_table_data(&pdf_table);
    // println!("{:#?}",pdf_table);
    println!("{:#?}",data);

    true
}


// PdfTable의 각 PageTable의 tables를 2차원 벡터로 변환
fn get_pdf_table_data(pdf_table: &PdfTable) -> Vec<Vec<String>> {
    let mut data: Vec<Vec<String>> = Vec::new();
    let header: Vec<String> = vec![
        "pos".to_owned(),
        "item".to_owned(),
        "qty".to_owned(),
        "description".to_owned(),
        "draw-no".to_owned(),
        "dimension".to_owned(),
        "material".to_owned(),
        "certi.".to_owned(),
        "kg".to_owned()
      ];
    // 데이터 작성
    data.push(header.clone());
    for page_table in &pdf_table.page_tables {
        for row in &page_table.tables {
            // 빈 행 제거
            let all_empty = row.iter().all(|x| x.is_empty());
            let is_header = row.iter().all(|x| header.contains(x));
            if all_empty || is_header {
                continue;
            } else {
                data.push(row.clone());
            }
        }
    }
    data
}