use rebox_types::{
    schema::{CurrentRowId, Table, TableName},
    ReboxResult,
};

use std::fmt::Debug;

use crate::Driver;

pub use self::fields::{DatabaseName, DatabaseTables};

use self::{
    builder::DatabaseBuilder,
    fields::{ReboxMaster, ReboxSchema, ReboxSequence},
    row::TableRow,
};

pub use connection::DatabaseConnection;

pub mod builder;
mod connection;
mod fields;
pub mod row;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Database<D: Driver> {
    name: DatabaseName,
    connection: DatabaseConnection<D>,
    rebox_sequence: ReboxSequence,
    rebox_schema: ReboxSchema,
    rebox_master: ReboxMaster,
    tables: DatabaseTables,
}

impl<D: Driver> Database<D> {
    pub fn list_tables(&self) -> Vec<&TableName> {
        self.tables.list_tables()
    }
    pub fn create_table(&mut self, table: Table) -> ReboxResult<TableName> {
        let table_name = self.tables.create_table(table)?;

        Ok(table_name)
    }
    pub fn insert_into_table<T: AsRef<str>>(
        &mut self,
        table: T,
        row: TableRow,
    ) -> ReboxResult<CurrentRowId> {
        let table_name = TableName::new(table.as_ref());
        self.rebox_sequence.check_can_inc_rowid(&table_name)?;
        let cur_row_id = self.tables.insert_into_table(table_name.to_owned(), row)?;
        self.rebox_sequence.bump_table_cur_rowid(&table_name)?;
        Ok(cur_row_id)
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
