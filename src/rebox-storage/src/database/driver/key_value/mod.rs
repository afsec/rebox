pub(super) mod builder;
mod create_table;
mod drop_table;
mod helpers;
mod insert_into_table;
mod list_tables;
mod number_of_stores;
mod table_exists;

// TODO: Implement another modude to perform (Defrag / Compact) Database

use self::{
    builder::KeyValueDriverBuilder, create_table::CreateTable, drop_table::DropTable,
    list_tables::ListTables, number_of_stores::NumberOfStores, table_exists::TableExists, insert_into_table::InsertIntoTable,
};
use super::DataStorage;
use crate::database::{driver::Driver, row::TableRow, DatabaseMetadata};
use anyhow::bail;
use rebox_derive::DbDriver;
use rebox_types::{
    schema::{name::TableName, RowId, Table},
    ReboxResult,
};

use rkv::{backend::SafeModeEnvironment, Rkv};
use std::sync::{Arc, RwLock};

pub(crate) type KvConnection = Arc<RwLock<Rkv<SafeModeEnvironment>>>;

#[derive(Debug, DbDriver)]
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
    pub(crate) fn table_exists(&self, table_name: &TableName) -> ReboxResult<bool> {
        TableExists::connect(self)?.exists(table_name)
    }
    pub(crate) fn number_of_stores(&self) -> ReboxResult<u16> {
        NumberOfStores::connect(self)?.len()
    }
    pub(crate) fn create_table(&self, table: &Table) -> ReboxResult<()> {
        if self.table_exists(table.name())? {
            bail!("Table [{}] already exists", table.name());
        } else {
            CreateTable::connect(self)?.create(table)
        }
    }
    pub(crate) fn insert_into_table(
        &self,
        table_name: TableName,
        table_row: TableRow,
    ) -> ReboxResult<RowId> {
        if self.table_exists(&table_name)? {
            InsertIntoTable::connect(self)?.insert(table_name, table_row)
        } else {
            bail!("Table [{}] already exists", &table_name);
        }
    }
    pub(crate) fn drop(&self, table_name: &TableName) -> ReboxResult<()> {
        if self.table_exists(table_name)? {
            DropTable::connect(self)?.delete(table_name)
        } else {
            bail!("Table [{}] not exists", table_name);
        }
    }

    pub(crate) fn list_tables(&self) -> ReboxResult<Vec<TableName>> {
        ListTables::connect(self)?.list()
    }
    pub(crate) fn metadata(&self) -> &DatabaseMetadata {
        &self.metadata
    }
}

impl DataStorage for KeyValueDriver {
    const MAX_SIZE_DB: usize = 1024 * 1024 * 1024 * 20; // 20 GBytes
    fn max_dbsize(&self) -> usize {
        Self::MAX_SIZE_DB
    }
}
