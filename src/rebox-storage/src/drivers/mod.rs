mod key_value;
mod memory;

pub trait Driver {
    type Storage<DS>: DataStorage;
}

pub trait DataStorage {
    const MAX_SIZE_DB: usize;
    fn max_dbsize(&self) -> usize;
}

pub use key_value::{KeyValue, KeyValueStorage};
pub use memory::{InMemoryDriver, InMemoryStorage};
