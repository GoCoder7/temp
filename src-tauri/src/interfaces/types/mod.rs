use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct Paths {
    pub csv_path: String,
    pub pdf_path: String,
    pub xl_path: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct PdfTable {
    page_tables: Vec<PageTable>,
    num_pages: u32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageTable {
    page: u32,
    tables: Vec<Vec<String>>,
}