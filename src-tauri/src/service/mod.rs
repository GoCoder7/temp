use std::{fs::File, io::{self, BufReader, Read, Write}};

use encoding_rs::EUC_KR;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;
use crate::interfaces::types::{Paths, PdfTable};

pub async fn get_pdf_data(app: &AppHandle, paths: &Paths) -> Vec<Vec<String>> {
    let path: &String = &paths.pdf_path;
    let tbl: PdfTable = get_pdf_table_from_file(&app, &path).await;
    get_vec2_from_pdf_table(&tbl)
}

pub fn get_csv_data(paths: &Paths) -> Vec<Vec<String>> {
    // EUC-KR 파일이면 UTF-8로 변환
    let path = &paths.csv_path;
    convert_to_utf8_if_euckr(path).expect("failed to convert to utf-8");
    get_vec2_from_csv(path)
}

pub fn make_xl(paths: &Paths) {
}

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
        "kg".to_owned()
      ];
    // 필터링
    for page_table in &pdf_table.page_tables {
        for row in &page_table.tables {
            // 빈 행과 헤더 행
            let all_empty_or_header = row.iter().all(|x| x.is_empty() || header.contains(x));
            // 타겟이 아닌 행
            let target_material = vec!["AH32", "DH32", "EH32","AH36", "DH36", "EH36"];
            let is_not_target = !(row[5].starts_with("pl.") && target_material.contains(&row[6].as_ref()));

            if all_empty_or_header || is_not_target {
                continue;
            } else {
                data.push(row.clone());
            }
        }
    }
    data
}

pub fn get_vec2_from_csv(path: &str) -> Vec<Vec<String>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path)
        .expect("failed to read csv");
    let mut data: Vec<Vec<String>> = Vec::new();
    for res in rdr.records() {
        let rec = res.expect("failed to read record");
        let row: Vec<String> = rec.iter().map(|x| x.to_string()).collect();
        data.push(row);
    }
    data
}

pub fn convert_to_utf8_if_euckr(path: &str) -> io::Result<()>  {
    // EUC-KR 파일을 읽기 위한 파일 핸들
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);

    let mut euc_kr_content = Vec::new();
    buf_reader.read_to_end(&mut euc_kr_content)?;

    // EUC-KR에서 UTF-8로 변환
    let (utf8_content, _, had_errors) = EUC_KR.decode(&euc_kr_content);
    if !had_errors {
        // 변환된 UTF-8 내용을 같은 파일에 저장
        let mut output_file = File::create(path)?; // 같은 파일 경로로 저장
        output_file.write_all(utf8_content.as_bytes())?;
    };
    Ok(())
}