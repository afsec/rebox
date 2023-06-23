use std::collections::BTreeMap;

use crate::ReboxResult;
use anyhow::bail;

use crate::table::{CurrentRowId, Table, TableName, TableRow};

pub use self::rebox_sequence::ReboxSequence;

mod rebox_sequence;
#[cfg(test)]
mod tests;

// TODO
#[derive(Debug)]
pub struct DatabaseTables(BTreeMap<TableName, Table>);
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
        let table_name = table.name().to_owned();
        if self.0.contains_key(&table_name) {
            bail!("Table {} is already in Database", table.name());
        }
        self.0.insert(table_name.clone(), table);

        Ok(table_name)
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
