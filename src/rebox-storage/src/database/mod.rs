use std::fmt::Debug;

use crate::drivers::key_value::KeyValueDriver;
use rebox_types::{
    schema::{name::TableName, CurrentRowId, Table},
    ReboxResult,
};

use self::{
    fields::{
        name::DatabaseName, rebox_master::ReboxMaster, rebox_schema::ReboxSchema,
        rebox_sequence::ReboxSequence,
    },
    row::TableRow,
};

pub mod builder;
pub mod fields;
pub mod row;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Database {
    name: DatabaseName,
    driver: KeyValueDriver,
    metadata: DatabaseMetadata,
}

impl Database {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn list_tables(&self) -> ReboxResult<Vec<&TableName>> {
        // self.driver.list_tables()
        todo!();
        Ok(vec![])
    }
    pub fn create_table(&mut self, table: Table) -> ReboxResult<TableName> {
        // self.driver.open_table(&table, true)?;
        todo!();
        Ok(table.name().to_owned())
    }
    pub fn insert_into_table(
        &mut self,
        table_name: TableName,
        table_row: TableRow,
    ) -> ReboxResult<CurrentRowId> {
        todo!();
        Ok(CurrentRowId::default())
    }
}

#[derive(Debug, Default)]
pub struct DatabaseMetadata {
    rebox_sequence: ReboxSequence,
    rebox_schema: ReboxSchema,
    rebox_master: ReboxMaster,
}
