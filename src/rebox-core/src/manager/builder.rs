use std::marker::PhantomData;

use rebox_storage::Driver;
use rebox_types::{
    database::{Database, DatabaseName},
    helpers::check_valid_entity_name,
    ReboxResult,
};

use super::Manager;

impl<D: Driver> Manager<D> {
    pub fn new() -> ManagerBuilder<D> {
        ManagerBuilder {
            driver: PhantomData,
        }
    }
}

pub struct ManagerBuilder<D: Driver> {
    driver: PhantomData<D>,
}
impl<D: Driver> ManagerBuilder<D> {
    pub fn set_driver(self, driver: D) -> ReboxResult<BuilderWithDriver<D>> {
        Ok(BuilderWithDriver { driver })
    }
}

#[derive(Debug, Default)]
pub struct BuilderWithDriver<D: Driver> {
    driver: D,
}

impl<D: Driver> BuilderWithDriver<D> {
    pub fn set_database_name<S: AsRef<str>>(self, name: S) -> ReboxResult<BuilderWithParams<D>> {
        check_valid_entity_name(&name)?;
        let Self { driver } = self;
        // TODO
        Ok(BuilderWithParams {
            driver,
            database_name: DatabaseName::new(name),
        })
    }
}

#[derive(Debug, Default)]
pub struct BuilderWithParams<D: Driver> {
    driver: D,
    database_name: DatabaseName,
}
impl<D: Driver> BuilderWithParams<D> {
    pub fn build(self) -> ReboxResult<Manager<D>> {
        let Self {
            driver,
            database_name,
        } = self;
        // TODO
        Ok(Manager {
            driver,
            database: Database::new().set_name(database_name)?.build()?,
        })
    }
}
