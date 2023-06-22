mod key_value;
mod memory;

// use std::str::FromStr;

// use anyhow::bail;
// use rebox_types::ReboxResult;

pub trait Driver {
    type Storage<DS>: DataStorage;
}

pub trait DataStorage {
    fn max_dbsize(&self) -> usize;
}

pub use key_value::{KeyValue, KeyValueStorage};
pub use memory::{InMemoryDriver, InMemoryStorage};
