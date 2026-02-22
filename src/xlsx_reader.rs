use calamine::{Data, Range, Reader, Xlsx, open_workbook_from_rs};
use serde::{
    Serialize, Serializer,
    ser::{SerializeMap, SerializeStruct},
};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// 工作表信息输出类型
    #[wasm_bindgen(typescript_type = r#"Map<string, {
        headers: string[] | null,
        rows: (string | number | boolean | undefined)[][]
    }>"#)]
    pub type WorkSheetsOutPut;
}

/// 单元格数据引用
#[derive(Serialize)]
#[serde(untagged)]
pub enum CellDataRef<'a> {
    String(String),
    SharedString(&'a str),
    Number(f64),
    Boolean(bool),
    Undefined,
}

impl<'a> From<&'a Data> for CellDataRef<'a> {
    fn from(data: &'a Data) -> Self {
        match data {
            Data::String(s) => CellDataRef::SharedString(s),
            Data::Bool(b) => CellDataRef::Boolean(*b),
            Data::Int(i) => CellDataRef::Number(*i as f64),
            Data::Float(f) => CellDataRef::Number(*f),
            Data::Empty => CellDataRef::Undefined,
            Data::Error(_) => CellDataRef::Undefined,
            Data::DateTime(d) => CellDataRef::String(format!("{:?}", d)),
            Data::DateTimeIso(d) => CellDataRef::SharedString(d),
            Data::DurationIso(d) => CellDataRef::SharedString(d),
        }
    }
}

/// 包含所有工作表的名称和数据
pub struct WorkSheets {
    sheets: Vec<(String, Range<Data>)>,
}

impl WorkSheets {
    /// 创建工作区
    pub fn new(sheets: Vec<(String, Range<Data>)>) -> WorkSheets {
        WorkSheets { sheets }
    }

    /// 获取工作表的行数据
    pub fn rows(range: &Range<Data>) -> impl Iterator<Item = &[Data]> {
        let rows = range.rows();
        rows.skip(1)
    }

    /// 获取工作表的表头
    pub fn headers(range: &Range<Data>) -> Option<Vec<String>> {
        range.headers()
    }
}

impl Serialize for WorkSheets {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        struct WorkSheetInfo<'a> {
            headers: Option<Vec<String>>,
            rows: Vec<Vec<CellDataRef<'a>>>,
        }
        impl<'a> Serialize for WorkSheetInfo<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut struct_data = serializer.serialize_struct("WorkSheetInfo", 2)?;
                struct_data.serialize_field("headers", &self.headers)?;
                struct_data.serialize_field("rows", &self.rows)?;
                struct_data.end()
            }
        }

        let mut sheet_map = serializer.serialize_map(Some(self.sheets.len()))?;

        for (name, range) in &self.sheets {
            let rows: Vec<Vec<CellDataRef>> = Self::rows(range)
                .map(|row| row.iter().map(|cell| CellDataRef::from(cell)).collect())
                .collect();
            let headers = Self::headers(range);

            sheet_map.serialize_entry(name, &WorkSheetInfo { headers, rows })?;
        }

        sheet_map.end()
    }
}

/// 读取 xlsx 文件，返回包含所有工作表的名称和数据的对象
#[wasm_bindgen]
pub fn read_xlsx(data: &[u8]) -> Result<WorkSheetsOutPut, JsError> {
    let cursor = Cursor::new(data);
    let mut reader: Xlsx<_> = open_workbook_from_rs(cursor)?;

    Ok(serde_wasm_bindgen::to_value(&WorkSheets::new(
        reader.worksheets(),
    ))?.unchecked_into())
}
