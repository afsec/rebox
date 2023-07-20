use std::fmt::Display;

use rebox_types::schema::{
    column::model::{ColumnName, ColumnValue},
    RowId,
};

#[derive(Debug)]
pub struct RowData(Vec<ColumnData>);
impl From<Vec<ColumnData>> for RowData {
    fn from(value: Vec<ColumnData>) -> Self {
        Self(value)
    }
}

impl Display for RowData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maybe_rowid = self.0.first().map(|data| data.row_id());
        match maybe_rowid {
            Some(row_id) => {
                let mut header_str = format!("| row_id |");
                let mut data_str = format!("| {:a$} |", **row_id, a = 6);
                self.0.iter().for_each(|data| {
                    header_str.push_str(format!(" {} |", data.col_name()).as_str());
                    data_str.push_str(format!(" {} |", data.value()).as_str());
                });

                write!(f, "{header_str}\n{data_str}")
            }
            None => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub struct ColumnData {
    row_id: RowId,
    col_name: ColumnName,
    value: ColumnValue,
}

impl ColumnData {
    pub fn new() -> ColumnDataBuilder {
        ColumnDataBuilder
    }
    pub fn row_id(&self) -> &RowId {
        &self.row_id
    }

    pub fn col_name(&self) -> &String {
        &*(self.col_name)
    }

    pub fn value(&self) -> &ColumnValue {
        &self.value
    }
}

#[derive(Debug)]
pub struct ColumnDataBuilder;

impl ColumnDataBuilder {
    pub fn set_row_id(self, row_id: RowId) -> ColumnDataBuilderS1 {
        ColumnDataBuilderS1 { row_id }
    }
}

#[derive(Debug)]
pub struct ColumnDataBuilderS1 {
    row_id: RowId,
}

impl ColumnDataBuilderS1 {
    pub fn set_col_name<T: AsRef<str>>(self, name: T) -> ColumnDataBuilderS2 {
        let Self { row_id } = self;
        let col_name = ColumnName::from(name.as_ref());
        ColumnDataBuilderS2 { row_id, col_name }
    }
}

#[derive(Debug)]
pub struct ColumnDataBuilderS2 {
    row_id: RowId,
    col_name: ColumnName,
}

impl ColumnDataBuilderS2 {
    pub fn set_value(self, value: ColumnValue) -> ColumnDataBuilderS3 {
        let Self { row_id, col_name } = self;
        ColumnDataBuilderS3 {
            row_id,
            col_name,
            value,
        }
    }
}

#[derive(Debug)]
pub struct ColumnDataBuilderS3 {
    row_id: RowId,
    col_name: ColumnName,
    value: ColumnValue,
}
impl ColumnDataBuilderS3 {
    pub fn build(self) -> ColumnData {
        let Self {
            row_id,
            col_name,
            value,
        } = self;
        ColumnData {
            row_id,
            col_name,
            value,
        }
    }
}
