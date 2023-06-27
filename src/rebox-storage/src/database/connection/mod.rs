use crate::Driver;

#[derive(Debug)]
pub struct DatabaseConnection<D: Driver> {
    driver: D,
}

impl<D: Driver> DatabaseConnection<D> {
    pub fn new(driver: D) -> Self {
        Self { driver }
    }
}
