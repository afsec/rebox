use std::fmt::Debug;

use crate::drivers::Driver;

use connection::DatabaseConnection;

use self::fields::name::DatabaseName;

pub(crate) mod builder;
pub(crate) mod connection;
pub(crate) mod fields;
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
