use super::KeyValueDriver;
use anyhow::{bail, format_err};
use bincode::config::Configuration;
use rebox_types::{
    schema::{schema::TableSchema, RowId, Table},
    DbPrefix, ReboxResult,
};
use rkv::{backend::SafeModeEnvironment, Rkv, StoreOptions, Value};

pub(super) struct CreateTable<'a>(&'a KeyValueDriver);
impl<'a> CreateTable<'a> {
    pub(super) fn connect(driver: &'a KeyValueDriver) -> ReboxResult<Self> {
        Ok(Self(driver))
    }

    pub(super) fn create(self, table: &Table) -> ReboxResult<()> {
        let tbl_name = table.name();
        let tbl_schema = table.schema();
        let store_name_prefix = format!("{}-{}", Table::prefix(), tbl_name);
        tbl_schema
            .get_columns()
            .iter()
            .try_for_each(|(col_name, _)| {
                self.create_store(format!("{store_name_prefix}_{col_name}"))
            })?;
        self.create_store(format!("{store_name_prefix}_rowid"))?;
        self.update_master(table)?;
        self.update_sequence(table)?;
        self.check_integrity(table)?;
        Ok(())
    }

    fn create_store<T: AsRef<str>>(&self, store_name: T) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let rkv_env = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let store_name_str = store_name.as_ref();
        if rkv_env
            .open_single(store_name_str, StoreOptions::default())
            .is_ok()
        {
            bail!("KvStore {store_name_str} already exists!");
        } else {
            let _ = rkv_env.open_single(store_name_str, StoreOptions::create())?;
            Ok(())
        }
    }

    fn update_master(&self, table: &Table) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let rebox_master = self.0.metadata().rebox_master().table_name().as_ref();

        let rkv_env = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let store_name_str = table.name().as_ref();
        let master_store = rkv_env.open_single(rebox_master, StoreOptions::default())?;
        let mut writer = rkv_env.write()?;
        let schema_blob = bincode::encode_to_vec(table.schema(), bincode::config::standard())?;
        master_store.put(&mut writer, store_name_str, &Value::Blob(&schema_blob))?;
        writer.commit()?;

        Ok(())
    }

    fn update_sequence(&self, table: &Table) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let rebox_sequence = self.0.metadata().rebox_sequence().table_name().as_ref();

        let rkv_env: std::sync::RwLockReadGuard<'_, Rkv<SafeModeEnvironment>> = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let store_name_str = table.name().as_ref();
        let master_store = rkv_env.open_single(rebox_sequence, StoreOptions::default())?;
        let mut writer = rkv_env.write()?;
        master_store.put(&mut writer, store_name_str, &Value::U64(0))?;
        writer.commit()?;
        Ok(())
    }

    fn check_integrity(&self, table: &Table) -> ReboxResult<()> {
        let created_arc = self.0.connection();

        let rkv_env = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let table_name_str = table.name().as_ref();

        {
            let rebox_master = self.0.metadata().rebox_master().table_name().as_ref();
            let master_store = rkv_env.open_single(rebox_master, StoreOptions::default())?;
            let reader = rkv_env.read()?;
            let maybe_value: Option<Value> = master_store.get(&reader, table_name_str)?;

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
            let sequence_store = rkv_env.open_single(rebox_sequence, StoreOptions::default())?;
            let reader = rkv_env.read()?;
            let maybe_value: Option<Value> = sequence_store.get(&reader, table_name_str)?;

            let current_row_id = match maybe_value {
                Some(Value::U64(id)) => RowId::try_from(id)?,
                other => bail!("Health check alert: Table [{table_name_str}] type mismatch in [{rebox_sequence}]. Reason: {other:?}"            ),
            };

            if *current_row_id != 0 {
                bail!("Health check alert:  Table [{table_name_str}] is corrupted in [{rebox_sequence}]")
            }
        }

        Ok(())
    }
}
