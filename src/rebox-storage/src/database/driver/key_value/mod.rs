pub(super) mod builder;

// use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use rkv::{backend::SafeModeEnvironment, Rkv};

// use crate::database::name::DatabaseName;

use self::builder::KeyValueDriverBuilder;

use super::DataStorage;
use super::Driver;

pub(crate) type KvConnection = Arc<RwLock<Rkv<SafeModeEnvironment>>>;

#[derive(Debug)]
pub(crate) struct KeyValueDriver {
    // db_name: DatabaseName,
    // base_path: PathBuf,
    // create_mode: bool,
    connection: KvConnection,
}
impl KeyValueDriver {
    pub(crate) fn new() -> KeyValueDriverBuilder {
        KeyValueDriverBuilder::default()
    }

    pub(crate) fn connection(&self) -> &KvConnection {
        &self.connection
    }
}

impl Driver for KeyValueDriver {}

impl DataStorage for KeyValueDriver {
    const MAX_SIZE_DB: usize = 1024 * 1024 * 1024 * 20; // 20 GBytes
    fn max_dbsize(&self) -> usize {
        Self::MAX_SIZE_DB
    }
}
