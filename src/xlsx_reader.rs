use calamine::{Xlsx, XlsxError, open_workbook_from_rs};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct XlsxInfo {
    table_names: Vec<String>,
}

#[wasm_bindgen]
impl XlsxInfo {

    #[wasm_bindgen(constructor)]
    pub fn new(table_names: Vec<String>) -> XlsxInfo {
        XlsxInfo { table_names }
    }

    #[wasm_bindgen(getter)]
    pub fn table_names(&self) -> Vec<String> {
        self.table_names.clone()
    }
}

pub fn read_xlsx(data: &[u8]) -> Result<XlsxInfo, JsValue> {
    let cursor = Cursor::new(data);
    let xlsx: Xlsx<Cursor<&[u8]>> =
        open_workbook_from_rs(cursor).map_err(|e: XlsxError| JsValue::from(e.to_string()))?;
    let table_names = xlsx.table_names().iter().map(|s| s.to_string()).collect();
    Ok(XlsxInfo::new(table_names))
}
