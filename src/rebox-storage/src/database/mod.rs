use crate::drivers::Driver;
use anyhow::bail;
use bytes::{Buf, BufMut, BytesMut};
use rebox_types::{
    table::{ReboxSequence, Table, TableFileName, TableName},
    ReboxResult,
};
use std::{fmt::Debug, marker::PhantomData};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Database<D: Driver> {
    driver: D,
    database_name: String,
    rebox_sequence: ReboxSequence,
    tables: Vec<Table>,
}

impl<D: Driver> Database<D> {
    pub fn new() -> DatabaseBuilder<D> {
        DatabaseBuilder(PhantomData)
    }

    pub fn driver(&self) -> &D {
        &self.driver
    }
    pub fn connect(&mut self) -> ReboxResult<()> {
        // TODO
        Ok(())
    }
}

pub struct DatabaseBuilder<D: Driver>(PhantomData<D>);
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
    pub fn set_session_name<S: AsRef<str>>(
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
