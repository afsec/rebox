use std::collections::BTreeMap;

use anyhow::{bail, Ok};
use rebox_types::schema::{
    column::model::{ColumnName, ColumnValue},
    RowId,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RowData {
    row_id: RowId,
    #[serde(flatten)]
    data: BTreeMap<ColumnName, ColumnValue>,
}

impl RowData {
    // TODO: Refactor RowId;
    pub fn new_rowid(row_id: RowId) -> Self {
        Self {
            row_id,
            data: Default::default(),
        }
    }
    pub fn row_id(&self) -> &RowId {
        &self.row_id
    }
    pub fn col_names(&self) -> Vec<&ColumnName> {
        self.data.keys().collect()
    }
    pub fn col_values(&self) -> Vec<&ColumnValue> {
        self.data.values().collect()
    }
}
impl TryFrom<Vec<ColumnData>> for RowData {
    type Error = anyhow::Error;

    fn try_from(vec: Vec<ColumnData>) -> Result<Self, Self::Error> {
        if vec.is_empty() {
            bail!("Empty Vec.")
        } else {
            let mut data = BTreeMap::new();
            let mut maybe_first_row_id: Option<RowId> = None;
            vec.into_iter().enumerate().for_each(|(idx, column_data)| {
                let ColumnData {
                    row_id,
                    col_name,
                    value,
                } = column_data;
                if idx == 0 {
                    maybe_first_row_id = Some(row_id)
                }
                data.insert(col_name, value);
            });
            match maybe_first_row_id {
                Some(row_id) => Ok(Self { row_id, data }),
                None => bail!("Empty Vec."),
            }
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
