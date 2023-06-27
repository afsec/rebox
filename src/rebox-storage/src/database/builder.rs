use std::marker::PhantomData;

use rebox_types::{helpers::check_valid_entity_name, ReboxResult};

use crate::Driver;

use super::{
    fields::{DatabaseName, ReboxMaster, ReboxSchema, ReboxSequence},
    Database, DatabaseConnection,
};

impl<D: Driver> Database<D> {
    pub fn new() -> DatabaseBuilder<D> {
        DatabaseBuilder(PhantomData)
    }
}

pub struct DatabaseBuilder<D: Driver>(PhantomData<D>);

impl<D: Driver> DatabaseBuilder<D> {
    pub fn set_name<S: AsRef<str>>(self, name: S) -> ReboxResult<DatabaseBuilderS1<D>> {
        check_valid_entity_name(&name)?;

        Ok(DatabaseBuilderS1 {
            driver: PhantomData,
            name: DatabaseName::new(name),
        })
    }
}
#[derive(Debug, Default)]
pub struct DatabaseBuilderS1<D: Driver> {
    driver: PhantomData<D>,
    name: DatabaseName,
}
impl<D: Driver> DatabaseBuilderS1<D> {
    pub fn set_driver(self, driver: D) -> DatabaseBuilderS2<D> {
        let Self { name, .. } = self;

        DatabaseBuilderS2 { name, driver }
    }
}

#[derive(Debug, Default)]
pub struct DatabaseBuilderS2<D: Driver> {
    name: DatabaseName,
    driver: D,
}
impl<D: Driver> DatabaseBuilderS2<D> {
    pub fn build(self) -> ReboxResult<Database<D>> {
        let Self { name, driver } = self;
        // TODO
        let connection = DatabaseConnection::new()
            .set_driver(driver)
            .set_name(name.clone())?
            .build()
            .connect()?;
        Ok(Database { name, connection })
    }
}
