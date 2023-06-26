use std::collections::{ BTreeMap};

use anyhow::bail;

use crate::ReboxResult;

use super::{column::{TableColumn, ColumnName}};

#[derive(Debug, Default)]
pub struct TableSchema(BTreeMap<ColumnName,TableColumn>);

impl TableSchema {
    pub fn add_column(&mut self, column: TableColumn) -> ReboxResult<()> {
        let column_name = column.name();
        if self.0.contains_key(column_name) {
            bail!("Column already defined");
        }

        self.0.insert(column_name.to_owned(),column);

        Ok(())
    }
    pub fn count_columns(&self) -> usize {
        self.0.len()
    }
}
