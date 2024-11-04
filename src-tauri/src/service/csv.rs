use std::{fs::File, io::{self, BufReader, Read, Write}};

use encoding_rs::EUC_KR;

use crate::interfaces::types::Paths;

pub fn get_csv_data(paths: &Paths) -> Vec<Vec<String>> {
    // EUC-KR 파일이면 UTF-8로 변환
    let path = &paths.csv_path;
    convert_to_utf8_if_euckr(path).expect("failed to convert to utf-8");
    get_vec2_from_csv(path)
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

pub fn convert_to_utf8_if_euckr(path: &str) -> io::Result<()> {
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