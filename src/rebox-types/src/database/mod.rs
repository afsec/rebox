use anyhow::bail;

use crate::{
    schema::{
        column::{ColumnKind, ColumnName, SchemaColumn},
        name::TableName,
        CurrentRowId, Table,
    },
    ReboxResult,
};

use std::{collections::BTreeMap, fmt::Debug};

pub use self::fields::{name::DatabaseName, rebox_sequence::ReboxSequence, tables::DatabaseTables};
use self::{builder::DatabaseBuilder, row::TableRow};

pub mod builder;
mod fields;
pub mod row;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Database {
    name: DatabaseName,
    rebox_sequence: ReboxSequence,
    tables: DatabaseTables,
}

impl Database {
    pub fn new() -> DatabaseBuilder {
        DatabaseBuilder
    }

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
