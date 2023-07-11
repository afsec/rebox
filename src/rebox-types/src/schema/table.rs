use std::collections::BTreeMap;

use anyhow::bail;

use crate::ReboxResult;

use super::column::{model::ColumnName, SchemaColumn};

#[derive(Debug, Default, Clone)]
pub struct TableSchema(BTreeMap<ColumnName, SchemaColumn>);

impl TableSchema {
    pub fn add_column(&mut self, column: SchemaColumn) -> ReboxResult<()> {
        let column_name = column.name();
        if self.0.contains_key(column_name) {
            bail!("Column already defined");
        }

        self.0.insert(column_name.to_owned(), column);

        Ok(())
    }
    pub fn count_columns(&self) -> usize {
        self.0.len()
    }
    pub fn get_columns(&self) -> &BTreeMap<ColumnName, SchemaColumn> {
        &self.0
    }
}
