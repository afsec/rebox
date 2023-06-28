use std::fmt::Debug;

use crate::drivers::Driver;

pub(crate) use self::row::TableRow;

pub(crate) use self::fields::{
    DatabaseName, DatabaseTables, ReboxMaster, ReboxSchema, ReboxSequence,
};

pub(crate) use connection::DatabaseConnection;

pub(crate) mod builder;
mod connection;
mod fields;
pub(crate) mod row;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub(crate) struct Database<D: Driver> {
    name: DatabaseName,
    connection: DatabaseConnection<D>,
}

impl<D: Driver> Database<D> {
    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }
}
