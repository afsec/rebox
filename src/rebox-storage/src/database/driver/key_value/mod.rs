pub(super) mod builder;

// use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use anyhow::bail;
use anyhow::format_err;
use rebox_types::schema::table::TableSchema;
use rebox_types::schema::CurrentRowId;
use rkv::Value;
use rkv::{backend::SafeModeEnvironment, Rkv, StoreOptions};

use rebox_types::schema::name::TableName;
use rebox_types::schema::Table;
use rebox_types::ReboxResult;

use crate::database::DatabaseMetadata;

use super::DataStorage;
use super::Driver;

use self::builder::KeyValueDriverBuilder;

pub(crate) type KvConnection = Arc<RwLock<Rkv<SafeModeEnvironment>>>;

#[derive(Debug)]
pub(crate) struct KeyValueDriver {
    metadata: DatabaseMetadata,
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
        CreateTable::connect(&self)?.create(table)?;
        Ok(())
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

struct CreateTable<'a>(&'a KeyValueDriver);
impl<'a> CreateTable<'a> {
    fn connect(driver: &'a KeyValueDriver) -> ReboxResult<Self> {
        Ok(Self(driver))
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
        Self::check_integrity(&self, table)?;
        Ok(())
    }
    fn create_store<T: AsRef<str>>(&self, store_name: T) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let k = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let store_name_str = store_name.as_ref();
        if k.open_single(store_name_str, StoreOptions::default())
            .is_ok()
        {
            bail!("Table {store_name_str} already exists!");
        } else {
            let created_store = k.open_single(store_name_str, StoreOptions::create());

            let mut writer = k.write()?;
            // created_store?.put(&mut writer, "some_key", &Value::Str("some_value"))?;
            writer.commit().map_err(|err| format_err!("{err}"))?;
        }

        Ok(())
    }
    fn update_master(&self, table: &Table) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let rebox_master = self.0.metadata().rebox_master().table_name().as_ref();

        let k = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let store_name_str = table.name().as_ref();
        let master_store = k.open_single(rebox_master, StoreOptions::default())?;
        let mut writer = k.write()?;
        let schema_blob = bincode::encode_to_vec(&table.schema(), bincode::config::standard())?;
        master_store.put(&mut writer, store_name_str, &Value::Blob(&schema_blob))?;
        writer.commit().map_err(|err| format_err!("{err}"))?;

        Ok(())
    }
    fn update_sequence(&self, table: &Table) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let rebox_sequence = self.0.metadata().rebox_sequence().table_name().as_ref();

        let k: std::sync::RwLockReadGuard<'_, Rkv<SafeModeEnvironment>> = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let store_name_str = table.name().as_ref();
        let master_store = k.open_single(rebox_sequence, StoreOptions::default())?;
        let mut writer = k.write()?;
        master_store.put(&mut writer, store_name_str, &Value::U64(0))?;
        writer.commit().map_err(|err| format_err!("{err}"))?;
        Ok(())
    }
    fn check_integrity(&self, table: &Table) -> ReboxResult<()> {
        use bincode::config::Configuration;

        let created_arc = self.0.connection();

        let k = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let table_name_str = table.name().as_ref();

        {
            let rebox_master = self.0.metadata().rebox_master().table_name().as_ref();
            let master_store = k.open_single(rebox_master, StoreOptions::default())?;
            let mut reader = k.read()?;
            let maybe_value: Option<Value> = master_store.get(&mut reader, table_name_str)?;

            let blob = match maybe_value {
                Some(Value::Blob(inner_blob)) => inner_blob,
                other => bail!(
                "Health check alert: Table [{table_name_str}] type mismatch in [{rebox_master}]. Reason: {other:?}"
            ),
            };
            let (retrieved_table_schema, _) = bincode::decode_from_slice::<
                TableSchema,
                Configuration,
            >(blob, bincode::config::standard())?;
            
            if &retrieved_table_schema != table.schema() {
                bail!("Health check alert:  Table [{table_name_str}] is corrupted in [{rebox_master}]")
            }
        }
        {
            let rebox_sequence = self.0.metadata().rebox_sequence().table_name().as_ref();
            let master_store = k.open_single(rebox_sequence, StoreOptions::default())?;
            let mut reader = k.read()?;
            let maybe_value: Option<Value> = master_store.get(&mut reader, table_name_str)?;

            let current_row_id = match maybe_value {
                Some(Value::U64(id)) => CurrentRowId::try_from(id)?,
                other => bail!(
                    "Health check alert: Table [{table_name_str}] type mismatch in [{rebox_sequence}]. Reason: {other:?}"
    
            ),
            };

            if *current_row_id != 0 {
                bail!("Health check alert:  Table [{table_name_str}] is corrupted in [{rebox_sequence}]")
            }
        }

        Ok(())
    }
}
