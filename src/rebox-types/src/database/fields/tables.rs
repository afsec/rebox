use std::collections::{BTreeMap, BTreeSet};

use crate::{
    schema::{column::TableColumn, name::TableName, schema::TableSchema},
    ReboxResult,
};
use anyhow::bail;

use crate::schema::{CurrentRowId, Table, TableRow};





// TODO
#[derive(Debug)]
pub struct DatabaseTables(BTreeMap<TableName, TableSchema>);
impl Default for DatabaseTables {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl DatabaseTables {
    pub fn list_tables(&self) -> Vec<&TableName> {
        let outcome: Vec<&TableName> = self.0.iter().map(|(table, _)| table).collect();
        outcome
    }
    pub fn create_table(&mut self, table: Table) -> ReboxResult<TableName> {
        let (name, schema) = table.take();
        if self.0.contains_key(&name) {
            bail!("Table {} is already in Database", name);
        }
        self.0.insert(name.clone(), schema);

        Ok(name)
    }
    pub fn insert_into_table(
        &mut self,
        table_name: TableName,
        table_row: TableRow,
    ) -> ReboxResult<CurrentRowId> {
        let x = table_row;

        // TODO
        todo!();
        Ok(0.into())
    }
}
