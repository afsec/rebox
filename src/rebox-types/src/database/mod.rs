use anyhow::bail;

use crate::{
    helpers::check_valid_name,
    schema::{name::TableName, CurrentRowId, Table, TableRow},
    ReboxResult,
};

use std::{cell::RefCell, fmt::Debug};

pub use self::fields::{name::DatabaseName, rebox_sequence::ReboxSequence, tables::DatabaseTables};

mod fields;

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
    pub fn insert_into_table(
        &mut self,
        table_name: TableName,
        table_row: TableRow,
    ) -> ReboxResult<CurrentRowId> {
        self.rebox_sequence.check_can_inc_rowid(&table_name)?;
        let cur_row_id = self
            .tables
            .insert_into_table(table_name.to_owned(), table_row)?;
        self.rebox_sequence.bump_table_cur_rowid(&table_name)?;
        Ok(cur_row_id)
    }

    pub fn name(&self) -> &str {
        self.name().as_ref()
    }
}

pub struct DatabaseBuilder;

impl DatabaseBuilder {
    pub fn set_name<S: AsRef<str>>(self, name: S) -> ReboxResult<DatabaseBuilderS1> {
        check_valid_name(&name)?;

        Ok(DatabaseBuilderS1 {
            name: DatabaseName::new(name),
        })
    }
}

#[derive(Debug, Default)]
pub struct DatabaseBuilderS1 {
    name: DatabaseName,
}
impl DatabaseBuilderS1 {
    pub fn build(self) -> ReboxResult<Database> {
        let Self { name } = self;
        // TODO
        Ok(Database {
            name,
            rebox_sequence: ReboxSequence::default(),
            tables: Default::default(),
        })
    }
}
