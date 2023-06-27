mod moz_rkv;

use crate::drivers::Driver;

impl Driver for KeyValueDriver {}

// impl Driver for KeyValueDriver {
//     type Storage<DS> = KeyValueStorage;
// }

// impl DataStorage for KeyValueStorage {
//     const MAX_SIZE_DB: usize = 1024 * 1024 * 1024 * 20; // 20 GBytes
//     fn max_dbsize(&self) -> usize {
//         Self::MAX_SIZE_DB
//     }
// }

#[derive(Debug, Default, PartialEq, Eq)]
pub struct KeyValueStorage;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KeyValueDriver;
