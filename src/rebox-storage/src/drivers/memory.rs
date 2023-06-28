use std::collections::BTreeMap;

use crate::database::row::TableRow;

// use super::{DataStorage, Driver};

// impl Driver for InMemoryDriver {
//     type Storage<DS> = InMemoryStorage;
// }

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct InMemoryDriver;

// impl DataStorage for InMemoryStorage {
//     const MAX_SIZE_DB: usize = u32::MAX as usize; // 4 GBytes
//     fn max_dbsize(&self) -> usize {
//         Self::MAX_SIZE_DB
//     }
// }

#[derive(Debug, Default)]
pub(crate) struct InMemoryStorage(BTreeMap<u32, TableRow>);
