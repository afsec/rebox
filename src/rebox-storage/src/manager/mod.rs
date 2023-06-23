use std::marker::PhantomData;

use rebox_types::{database::Database, table::TableName, ReboxResult};

use crate::drivers::Driver;

#[derive(Debug)]
pub struct Manager<D: Driver> {
    driver: D,
    database: Database,
}

impl<D: Driver> Manager<D> {
    pub fn new() -> ManagerBuilder<D> {
        ManagerBuilder {
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

    pub fn database(&self) -> &Database {
        &self.database
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
    pub fn database_name<S: AsRef<str>>(
        self,
        session_name: S,
    ) -> ReboxResult<BuilderWithParams<D>> {
        let Self { driver } = self;
        // TODO
        Ok(BuilderWithParams {
            driver,
            database_name: session_name.as_ref().to_string(),
        })
    }
}

#[derive(Debug, Default)]
pub struct BuilderWithParams<D: Driver> {
    driver: D,
    database_name: String,
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
            database: Database::new().database_name(database_name)?.build()?,
        })
    }
}
