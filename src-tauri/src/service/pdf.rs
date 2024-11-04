use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

use crate::interfaces::types::{Paths, PdfTable};

pub async fn get_pdf_data(app: &AppHandle, paths: &Paths) -> Vec<Vec<String>> {
    let path: &String = &paths.pdf_path;
    let tbl: PdfTable = get_pdf_table_from_file(&app, &path).await;
    get_vec2_from_pdf_table(&tbl)
}
pub fn get_pdf_idx(pdf_data: &Vec<Vec<String>>, item: &String) -> Option<usize> {
    for (i, row) in pdf_data.iter().enumerate() {
        // println!("row[1]: {}, *item: {}", row[0], *item);
        if &row[0] == item {
            return Some(i);
        }
    }
    None
}
pub async fn get_pdf_table_from_file(app: &AppHandle, path: &str) -> PdfTable {
    let sidecar_command = app.shell().sidecar("app").unwrap().arg(path);
    let output = sidecar_command.output().await.unwrap();
    let res_str = String::from_utf8(output.stdout).unwrap();
    serde_json::from_str(&res_str).expect("failed to parse json")
}
// PdfTable의 각 PageTable의 tables를 2차원 벡터로 변환(필터링 적용)
pub fn get_vec2_from_pdf_table(pdf_table: &PdfTable) -> Vec<Vec<String>> {
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
        "kg".to_owned(),
    ];
    // 필터링
    for page_table in &pdf_table.page_tables {
        for row in &page_table.tables {
            // 빈 행과 헤더 행
            let all_empty_or_header = row.iter().all(|x| x.is_empty() || header.contains(x));
            // 타겟이 아닌 행
            let target_material = vec!["AH32", "DH32", "EH32", "AH36", "DH36", "EH36"];
            let is_not_target =
                !(row[5].starts_with("pl.") && target_material.contains(&row[6].as_ref()));

            if all_empty_or_header || is_not_target {
                continue;
            } else {
                data.push(row.clone());
            }
        }
    }
    data
}