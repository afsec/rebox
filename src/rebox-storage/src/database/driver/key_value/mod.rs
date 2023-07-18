pub(super) mod builder;
mod create_table;
mod list_tables;
mod table_exists;
//TODO
mod drop_table;

use self::{
    builder::KeyValueDriverBuilder, create_table::CreateTable, list_tables::ListTables,
    table_exists::TableExists,
};
use super::{DataStorage, Driver};
use crate::database::DatabaseMetadata;
use anyhow::bail;
use rebox_types::{
    schema::{name::TableName, Table},
    ReboxResult,
};
use rkv::{backend::SafeModeEnvironment, Rkv};
use std::sync::{Arc, RwLock};

pub(crate) type KvConnection = Arc<RwLock<Rkv<SafeModeEnvironment>>>;

#[derive(Debug)]
pub(crate) struct KeyValueDriver {
    metadata: DatabaseMetadata,
    connection: KvConnection,
}
impl KeyValueDriver {
    pub(crate) fn new() -> KeyValueDriverBuilder {
        KeyValueDriverBuilder
    }

    pub(crate) fn connection(&self) -> &KvConnection {
        &self.connection
    }
    pub(crate) fn table_exists(&self, table: &Table) -> ReboxResult<bool> {
        TableExists::connect(self)?.exists(table)
    }
    pub(crate) fn create_table(&self, table: &Table) -> ReboxResult<()> {
        if self.table_exists(table)? {
            bail!("Table [{}] already exists", table.name());
        } else {
            CreateTable::connect(self)?.create(table)
        }
    }

    pub(crate) fn list_tables(&self) -> ReboxResult<Vec<TableName>> {
        ListTables::connect(self)?.list()
    }
    pub(crate) fn metadata(&self) -> &DatabaseMetadata {
        &self.metadata
    }
}

impl Driver for KeyValueDriver {}

impl DataStorage for KeyValueDriver {
    const MAX_SIZE_DB: usize = 1024 * 1024 * 1024 * 20; // 20 GBytes
    fn max_dbsize(&self) -> usize {
        Self::MAX_SIZE_DB
    }
}
