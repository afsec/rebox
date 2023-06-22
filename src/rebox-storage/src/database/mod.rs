use crate::{
    drivers::{DataStorage, Driver},
    table::{ReboxSequence, Table},
};
use anyhow::bail;
use bytes::{Buf, BufMut, BytesMut};
use rebox_types::ReboxResult;
use std::{fmt::Debug, marker::PhantomData};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Database<D: Driver, DS: DataStorage> {
    driver: D,
    database_name: String,
    rebox_sequence: ReboxSequence,
    tables: Vec<Table<DS>>,
}

impl<D: Driver, DS: DataStorage> Database<D, DS> {
    pub fn new() -> DatabaseBuilder<D, DS> {
        DatabaseBuilder {
            driver: PhantomData,
            storage: PhantomData,
        }
    }

    pub fn driver(&self) -> &D {
        &self.driver
    }
    pub fn connect(&mut self) -> ReboxResult<()> {
        // TODO
        Ok(())
    }
}

pub struct DatabaseBuilder<D: Driver, DS: DataStorage> {
    driver: PhantomData<D>,
    storage: PhantomData<DS>,
}
impl<D: Driver, DS: DataStorage> DatabaseBuilder<D, DS> {
    pub fn set_driver(self, driver: D, storage: DS) -> ReboxResult<DatabaseWithDriver<D, DS>> {
        Ok(DatabaseWithDriver { driver, storage })
    }
}

#[derive(Debug, Default)]
pub struct DatabaseWithDriver<D: Driver, DS: DataStorage> {
    driver: D,
    storage: DS,
}

impl<D: Driver, DS: DataStorage> DatabaseWithDriver<D, DS> {
    pub fn set_session_name<S: AsRef<str>>(
        self,
        session_name: S,
    ) -> ReboxResult<DatabaseWithParams<D, DS>> {
        let Self { driver, storage } = self;
        // TODO
        Ok(DatabaseWithParams {
            driver,
            database_name: session_name.as_ref().to_string(),
            storage,
        })
    }
}

#[derive(Debug, Default)]
pub struct DatabaseWithParams<D: Driver, DS: DataStorage> {
    driver: D,
    storage: DS,
    database_name: String,
}
impl<D: Driver, DS: DataStorage> DatabaseWithParams<D, DS> {
    pub fn build(self) -> ReboxResult<Database<D, DS>> {
        let Self {
            driver,
            database_name,
            storage,
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
