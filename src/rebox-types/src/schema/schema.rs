use std::collections::BTreeSet;

use anyhow::bail;

use crate::ReboxResult;

use super::column::TableColumn;

#[derive(Debug, Default)]
pub struct TableSchema(BTreeSet<TableColumn>);

impl TableSchema {
    pub fn add_column(&mut self, column: TableColumn) -> ReboxResult<()> {
        if self.0.contains(&column) {
            bail!("Column already defined");
        }

        self.0.insert(column);

        Ok(())
    }
    pub fn count_columns(&self) -> usize {
        self.0.len()
    }
}
