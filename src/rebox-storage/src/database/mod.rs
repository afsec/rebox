use std::fmt::Debug;

use crate::drivers::Driver;

use anyhow::bail;
use connection::DatabaseConnection;
use rebox_types::{
    schema::{name::TableName, CurrentRowId, Table},
    ReboxResult,
};

use self::{fields::name::DatabaseName, row::TableRow};

pub mod builder;
pub mod connection;
pub mod fields;
pub mod row;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Database<D: Driver> {
    name: DatabaseName,
    connection: DatabaseConnection<D>,
}

impl<D: Driver> Database<D> {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn list_tables(&self) -> ReboxResult<Vec<&TableName>> {
        self.connection.list_tables()
    }
    pub fn create_table(&mut self, table: Table) -> ReboxResult<TableName> {
        self.connection.create_table(table)
    }
    pub fn insert_into_table(
        &mut self,
        table_name: TableName,
        table_row: TableRow,
    ) -> ReboxResult<CurrentRowId> {
        self.connection.insert_into_table(table_name, table_row)
    }
}
