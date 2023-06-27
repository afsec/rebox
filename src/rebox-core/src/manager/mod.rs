use rebox_storage::Driver;
use rebox_types::database::Database;

mod builder;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Manager<D: Driver> {
    driver: D,
    database: Database,
}

impl<D: Driver> Manager<D> {
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
