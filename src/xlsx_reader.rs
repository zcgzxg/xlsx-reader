use calamine::{Range, Data, Reader, Xlsx, open_workbook_from_rs};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

pub struct XlsxInfo {
    sheets: Vec<(String, Range<Data>)>
}

impl XlsxInfo {
    pub fn new(sheets: Vec<(String, Range<Data>)>) -> XlsxInfo {
        XlsxInfo { sheets }
    }
}

pub fn read_xlsx(data: &[u8]) -> Result<XlsxInfo, JsError> {
    let cursor = Cursor::new(data);
    let mut reader: Xlsx<_> =
        open_workbook_from_rs(cursor)?;

    Ok(XlsxInfo::new(reader.worksheets()))
}
