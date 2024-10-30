use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
};

use crate::interfaces::types::{Paths, PdfTable};
use encoding_rs::EUC_KR;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;
use umya_spreadsheet::*;

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

pub fn make_xl(paths: &Paths, pdf_data: Vec<Vec<String>>, csv_data: Vec<Vec<String>>) {
    let path = &paths.xl_path;
    let mut book = new_file();
    book.remove_sheet(0).expect("failed to remove sheet1");
    let sht_name = "Differ Check";
    let sht = book
        .new_sheet(sht_name)
        .expect(format!("failed to create sheet: {}", sht_name).as_str());

    let mut xl_row_i = 1;

    // 제목
    sht.get_cell_mut((xl_row_i, 1))
        .set_value(csv_data[0][0].as_str())
        .get_style_mut()
        .get_font_mut()
        .set_bold(true);
    xl_row_i += 1;

    // 헤더
    let header = csv_data[1].clone();
    for col_i in 0..=10 {
        let xl_col_i = (col_i + 1) as u32;
        let cell = sht.get_cell_mut((xl_col_i, xl_row_i));
        let font = cell.get_style_mut().get_font_mut();
        font.set_bold(true);
        match xl_col_i {
            1..=9 => {
                cell.set_value(&header[col_i]);
            }
            10 | 11 => {
                font.get_color_mut().set_argb(Color::COLOR_RED);
                if xl_col_i == 10 {
                    cell.set_value("Differ Check");
                } else {
                    cell.set_value("Description");
                }
            }
            _ => (),
        };
    }
    xl_row_i += 1;

    // 데이터
    let mut pdf_idxs: Vec<usize> = (0..pdf_data.len()).collect();
    // 행(제목과 헤더인 위의 두줄은 제외)
    for csv_row in csv_data.iter().skip(2) {
        let mut absence = false;
        let mut err_msgs = Vec::new();
        let mut pdf_row = &vec![];
        // let mut absence_nums = Vec::new();

        // 1. csv의 부재번호에 해당하는 pdf의 행을 찾음(없으면 absense = true)

        if let Some(pdf_i) = get_pdf_idx(&pdf_data, &csv_row[1]) {
            pdf_row = &pdf_data[pdf_i];
            pdf_idxs.retain(|&x| x != pdf_i);
        } else {
            absence = true;
        }

        if absence {
            err_msgs.push("원본에 해당 부재 없음".to_owned());
        }

        // 열
        for csv_col_i in 0..=10 {
            let xl_col_i = (csv_col_i + 1) as u32;

            let cell = sht.get_cell_mut((xl_col_i, xl_row_i));

            // 1. 비교 후 에러가 있는 경우, 에러벡터에 추가
            if !absence {
                match xl_col_i {
                    // 두께와 dimension 비교
                    3 => {
                        let dimension = &pdf_row[5].split(".").last().unwrap_or("").to_owned();
                        let thickness = &csv_row[csv_col_i]
                            .split(".")
                            .next()
                            .unwrap_or("")
                            .to_owned();
                        if thickness != dimension {
                            err_msgs.push("두께 불일치".to_owned());
                            cell.get_style_mut()
                                .get_font_mut()
                                .get_color_mut()
                                .set_argb(Color::COLOR_RED);
                        }
                    }
                    // GRADE와 material 비교
                    4 => {
                        let material = &pdf_row[6];
                        if &csv_row[csv_col_i] != material {
                            err_msgs.push("GRADE 불일치".to_owned());
                            cell.get_style_mut()
                                .get_font_mut()
                                .get_color_mut()
                                .set_argb(Color::COLOR_RED);
                        }
                    }
                    // 수량과 qty 비교
                    6 => {
                        let qty = pdf_row[2]
                            .parse::<u32>()
                            .expect(format!("failed to parse qty {}", pdf_row[2]).as_str());
                        if csv_row[csv_col_i]
                            .parse::<u32>()
                            .expect(format!("failed to parse 수량{}", csv_row[csv_col_i]).as_str())
                            != qty
                        {
                            err_msgs.push("수량 불일치".to_owned());

                            cell.get_style_mut()
                                .get_font_mut()
                                .get_color_mut()
                                .set_argb(Color::COLOR_RED);
                        }
                    }
                    _ => (),
                }
            }
            // 값 작성
            match xl_col_i {
                1..=9 => {
                    cell.set_value(&csv_row[csv_col_i]);
                }
                10 => {
                    if err_msgs.is_empty() {
                        cell.set_value("OK");
                    } else {
                        cell.set_value("N.G");
                    }
                    cell.get_style_mut()
                        .get_font_mut()
                        .get_color_mut()
                        .set_argb(Color::COLOR_RED);
                }
                11 => {
                    if !err_msgs.is_empty() {
                        cell.set_value(err_msgs.join(", "));
                    }
                    cell.get_style_mut()
                        .get_font_mut()
                        .get_color_mut()
                        .set_argb(Color::COLOR_RED);
                }
                _ => (),
            }
        }
        xl_row_i += 1;
    }

    // pdf에만 있는 부재번호 표시
    if pdf_idxs.len() > 0 {
        for pdf_i in pdf_idxs {
            let pdf_row = &pdf_data[pdf_i];

            for xl_col_i in 10..=11 {
                let cell = sht.get_cell_mut((xl_col_i, xl_row_i));
                cell.get_style_mut()
                    .get_font_mut()
                    .get_color_mut()
                    .set_argb(Color::COLOR_RED);

                if xl_col_i == 10 {
                    cell.set_value("N.G");
                } else {
                    cell.set_value(&format!("부재번호 {} 누락", pdf_row[0]));
                }
            }

            xl_row_i += 1;
        }
    }

    // 추가설정
    for col in sht.get_column_dimensions_mut().iter_mut() {
        col.set_auto_width(true);
    }

    // 엑셀파일 생성
    writer::xlsx::write(&book, path).expect("failed to write xlsx");
}

fn get_pdf_idx(pdf_data: &Vec<Vec<String>>, item: &String) -> Option<usize> {
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
