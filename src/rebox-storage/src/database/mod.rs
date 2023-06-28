use std::fmt::Debug;

use crate::Driver;

pub use self::{
    fields::{DatabaseName, DatabaseTables},
    row::TableRow,
};

pub use self::fields::{ReboxMaster, ReboxSchema, ReboxSequence};

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
}

impl<D: Driver> Database<D> {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
