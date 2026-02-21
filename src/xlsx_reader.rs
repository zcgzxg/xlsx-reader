use calamine::{Reader, Xlsx, open_workbook_from_rs};
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

#[wasm_bindgen]
pub fn read_xlsx(data: &[u8]) -> Result<XlsxInfo, JsError> {
    let cursor = Cursor::new(data);
    let reader: Xlsx<_> =
        open_workbook_from_rs(cursor)?;
    let table_names = reader.sheet_names();

    Ok(XlsxInfo::new(table_names))
}
