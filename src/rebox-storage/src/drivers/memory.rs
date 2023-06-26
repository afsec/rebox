use std::collections::BTreeMap;

use rebox_types::database::row::TableRow;

use super::{DataStorage, Driver};

impl Driver for InMemoryDriver {
    type Storage<DS> = InMemoryStorage;
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct InMemoryDriver;

impl DataStorage for InMemoryStorage {
    const MAX_SIZE_DB: usize = u32::MAX as usize; // 4 GBytes
    fn max_dbsize(&self) -> usize {
        Self::MAX_SIZE_DB
    }
}

#[derive(Debug, Default)]
pub struct InMemoryStorage(BTreeMap<u32, TableRow>);
