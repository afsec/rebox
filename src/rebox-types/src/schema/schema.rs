use super::column::SchemaColumn;
use crate::ReboxResult;
use anyhow::bail;
use bincode::{Decode, Encode};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone, Decode, Encode, PartialEq, Eq)]
pub struct TableSchema(BTreeMap<String, SchemaColumn>);

impl TableSchema {
    pub fn add_column(&mut self, column: SchemaColumn) -> ReboxResult<()> {
        let column_name = &**(column.name());
        if self.0.contains_key(column_name) {
            bail!("Column already defined");
        }

        self.0.insert(column_name.to_owned(), column);

        Ok(())
    }
    pub fn count_columns(&self) -> usize {
        self.0.len()
    }
    pub fn get_columns(&self) -> &BTreeMap<String, SchemaColumn> {
        &self.0
    }
}
