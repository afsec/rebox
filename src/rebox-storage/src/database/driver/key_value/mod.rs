pub(super) mod builder;

// use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use anyhow::format_err;
use rkv::{backend::SafeModeEnvironment, Rkv};

use rebox_types::schema::name::TableName;
use rebox_types::schema::Table;
use rebox_types::ReboxResult;

use super::DataStorage;
use super::Driver;

use self::builder::KeyValueDriverBuilder;

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
    pub(crate) fn create_table(&self, table: &Table) -> ReboxResult<()> {
        CreateTable::connect(self.connection())?.create(table)?;
        Ok(())
    }
}

impl Driver for KeyValueDriver {}

impl DataStorage for KeyValueDriver {
    const MAX_SIZE_DB: usize = 1024 * 1024 * 1024 * 20; // 20 GBytes
    fn max_dbsize(&self) -> usize {
        Self::MAX_SIZE_DB
    }
}

struct CreateTable<'a>(&'a KvConnection);
impl<'a> CreateTable<'a> {
    fn connect(conn: &'a KvConnection) -> ReboxResult<Self> {
        Ok(Self(conn))
    }
    fn create(self, table: &Table) -> ReboxResult<()> {
        let tbl_name = table.name();
        let tbl_schema = table.schema();
        let store_name_prefix = format!("{}_{}", "rebox", table.name());
        // TODO
        tbl_schema
            .get_columns()
            .iter()
            .try_for_each(|(col_name, column)| {
                Self::create_store(&self, format!("{store_name_prefix}_{col_name}"))
            })?;
        // TODO
        Self::update_master(&self, table)?;
        Self::update_sequence(&self, table)?;
        Self::health_check(&self, table.name())?;
        Ok(())
    }
    fn create_store<T: AsRef<str>>(&self, store_name: T) -> ReboxResult<()> {
        use rkv::{StoreOptions, Value};
        let created_arc = self.0;
        let k = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;

        let created_store = k.open_single(store_name.as_ref(), StoreOptions::create());

        let mut writer = k.write()?;
        // created_store?.put(&mut writer, "some_key", &Value::Str("some_value"))?;
        writer.commit().map_err(|err| format_err!("{err}"))?;

        Ok(())
    }
    fn update_master(&self, table: &Table) -> ReboxResult<()> {
        Ok(())
    }
    fn update_sequence(&self, table: &Table) -> ReboxResult<()> {
        Ok(())
    }
    fn health_check(&self, table: &TableName) -> ReboxResult<()> {
        Ok(())
    }
}
// fn step_1(table: &Table) -> ReboxResult<()> {
// TODO: Implement rebox_schema
// TODO: Implement rebox_sequence
// Ok(())
// }
