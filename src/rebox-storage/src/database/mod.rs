use crate::{
    drivers::{DataStorage, Driver},
    table::{CurrentRowId, Table, TableName, TableRow},
};

use rebox_types::ReboxResult;
use std::{fmt::Debug, marker::PhantomData};

use self::tables::{DatabaseTables, ReboxSequence};

mod tables;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Database<D: Driver> {
    driver: D,
    database_name: String,
    rebox_sequence: ReboxSequence,
    tables: DatabaseTables,
}

impl<D: Driver> Database<D> {
    pub fn new() -> DatabaseBuilder<D> {
        DatabaseBuilder {
            driver: PhantomData,
        }
    }

    pub fn driver(&self) -> &D {
        &self.driver
    }
    // pub fn connect(&mut self) -> ReboxResult<Arc<RwLock<&D>>> {
    //     // TODO
    //     Ok(Arc::new(RwLock::new(&self.driver)))
    // }

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
}

pub struct DatabaseBuilder<D: Driver> {
    driver: PhantomData<D>,
}
impl<D: Driver> DatabaseBuilder<D> {
    pub fn set_driver(self, driver: D) -> ReboxResult<DatabaseWithDriver<D>> {
        Ok(DatabaseWithDriver { driver })
    }
}

#[derive(Debug, Default)]
pub struct DatabaseWithDriver<D: Driver> {
    driver: D,
}

impl<D: Driver> DatabaseWithDriver<D> {
    pub fn database_name<S: AsRef<str>>(
        self,
        session_name: S,
    ) -> ReboxResult<DatabaseWithParams<D>> {
        let Self { driver } = self;
        // TODO
        Ok(DatabaseWithParams {
            driver,
            database_name: session_name.as_ref().to_string(),
        })
    }
}

#[derive(Debug, Default)]
pub struct DatabaseWithParams<D: Driver> {
    driver: D,
    database_name: String,
}
impl<D: Driver> DatabaseWithParams<D> {
    pub fn build(self) -> ReboxResult<Database<D>> {
        let Self {
            driver,
            database_name,
        } = self;
        // TODO
        Ok(Database {
            driver,
            database_name,
            rebox_sequence: ReboxSequence::default(),
            tables: Default::default(),
        })
    }
}
