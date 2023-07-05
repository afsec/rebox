pub mod builder;
mod moz_rkv;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use rkv::{backend::SafeModeEnvironment, Rkv};

use crate::database::fields::name::DatabaseName;
use crate::drivers::Driver;

use self::builder::KeyValueDriverBuilder;

use super::DataStorage;

pub type KvConnection = Arc<RwLock<Rkv<SafeModeEnvironment>>>;

#[derive(Debug)]
pub struct KeyValueDriver {
    db_name: DatabaseName,
    base_path: PathBuf,
    create_mode: bool,
    connection: KvConnection,
}
impl KeyValueDriver {
    pub fn new() -> KeyValueDriverBuilder {
        KeyValueDriverBuilder::default()
    }

    pub fn connection(&self) -> &KvConnection {
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
