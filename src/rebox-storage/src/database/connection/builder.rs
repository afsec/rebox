use std::marker::PhantomData;

use rebox_types::{helpers::check_valid_entity_name, ReboxResult};

use crate::{database::DatabaseName, drivers::Driver};

use super::DatabaseConnection;

impl<D: Driver> DatabaseConnection<D> {
    pub fn new() -> DatabaseConnectionBuilder<D> {
        DatabaseConnectionBuilder(PhantomData)
    }
}

pub struct DatabaseConnectionBuilder<D: Driver>(PhantomData<D>);

impl<D: Driver> DatabaseConnectionBuilder<D> {
    pub fn set_driver(self, driver: D) -> DatabaseConnectionBuilderS1<D> {
        DatabaseConnectionBuilderS1 { driver }
    }
}

pub struct DatabaseConnectionBuilderS1<D: Driver> {
    driver: D,
}
impl<D: Driver> DatabaseConnectionBuilderS1<D> {
    pub fn set_name(self, name: DatabaseName) -> ReboxResult<DatabaseConnectionBuilderS2<D>> {
        check_valid_entity_name(&name)?;
        let Self { driver } = self;
        Ok(DatabaseConnectionBuilderS2 { driver, name })
    }
}

pub struct DatabaseConnectionBuilderS2<D: Driver> {
    driver: D,
    name: DatabaseName,
}
impl<D: Driver> DatabaseConnectionBuilderS2<D> {
    pub fn build(self) -> DatabaseConnectionToConnect<D> {
        let Self { driver, name } = self;
        DatabaseConnectionToConnect { driver, name }
    }
}

pub struct DatabaseConnectionToConnect<D: Driver> {
    driver: D,
    name: DatabaseName,
}
impl<D: Driver> DatabaseConnectionToConnect<D> {
    pub fn connect(self) -> ReboxResult<DatabaseConnection<D>> {
        let Self { driver, name } = self;
        // TODO: Start storage controller
        Ok(DatabaseConnection {
            driver,
            name,
            rebox_sequence: Default::default(),
            rebox_schema: Default::default(),
            rebox_master: Default::default(),
            // tables: Default::default(),
        })
    }
}
