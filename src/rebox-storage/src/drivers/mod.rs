use crate::database::fields::name::DatabaseName;
use rebox_types::ReboxResult;

pub mod key_value;
pub mod memory;

pub trait Driver {
    type Storage: DataStorage;
    fn connect(&mut self, db_name: &DatabaseName) -> ReboxResult<Self::Storage>;
}

pub trait DataStorage {
    const MAX_SIZE_DB: usize;
    fn max_dbsize(&self) -> usize;
}
