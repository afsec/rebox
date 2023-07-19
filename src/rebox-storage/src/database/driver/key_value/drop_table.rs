use anyhow::{bail, format_err};
use rebox_types::{
    schema::{name::TableName, Table},
    DbPrefix, ReboxResult,
};
use rkv::{backend::SafeModeEnvironment, Rkv, StoreOptions};

use super::{helpers::retrieve_schema, KeyValueDriver};

pub(super) struct DropTable<'a>(&'a KeyValueDriver);
impl<'a> DropTable<'a> {
    pub(super) fn connect(driver: &'a KeyValueDriver) -> ReboxResult<Self> {
        Ok(Self(driver))
    }

    pub(super) fn delete(self, table_name: &TableName) -> ReboxResult<()> {
        let tbl_schema = retrieve_schema(&self.0.connection(), self.0.metadata(), table_name)?;
        let store_name_prefix = format!("{}-{}", Table::prefix(), table_name);
        tbl_schema
            .get_columns()
            .iter()
            .try_for_each(|(col_name, _)| {
                Self::delete_store(&self, format!("{store_name_prefix}_{col_name}"))
            })?;
        Self::update_master(&self, table_name)?;
        Self::update_sequence(&self, table_name)?;
        Self::check_integrity(&self, table_name)?;
        Ok(())
    }

    fn delete_store<T: AsRef<str>>(&self, store_name: T) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let rkv_env = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let store_name_str = store_name.as_ref();
        match rkv_env.open_single(store_name_str, StoreOptions::default()) {
            Ok(store) => {
                let mut writer = rkv_env.write()?;
                store.clear(&mut writer)?;
                writer.commit()?;
                Ok(())
            }
            Err(_) => {
                bail!("KvStore {store_name_str} already exists!");
            }
        }
    }

    fn update_master(&self, table_name: &TableName) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let rebox_master = self.0.metadata().rebox_master().table_name().as_ref();

        let rkv_env = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let store_name_str = table_name.as_ref();
        let master_store = rkv_env.open_single(rebox_master, StoreOptions::default())?;
        let mut writer = rkv_env.write()?;
        master_store.delete(&mut writer, store_name_str)?;
        writer.commit()?;

        Ok(())
    }

    fn update_sequence(&self, table_name: &TableName) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let rebox_sequence = self.0.metadata().rebox_sequence().table_name().as_ref();

        let rkv_env: std::sync::RwLockReadGuard<'_, Rkv<SafeModeEnvironment>> = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let store_name_str = table_name.as_ref();
        let master_store = rkv_env.open_single(rebox_sequence, StoreOptions::default())?;
        let mut writer = rkv_env.write()?;
        master_store.delete(&mut writer, store_name_str)?;
        writer.commit()?;
        Ok(())
    }

    fn check_integrity(&self, table_name: &TableName) -> ReboxResult<()> {
        let created_arc = self.0.connection();

        let rkv_env = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let table_name_str = table_name.as_ref();

        {
            let rebox_master = self.0.metadata().rebox_master().table_name().as_ref();
            let master_store = rkv_env.open_single(rebox_master, StoreOptions::default())?;
            let reader = rkv_env.read()?;
            if let Some(_) = master_store.get(&reader, table_name_str)? {
                bail!("Health check alert:  Table [{table_name_str}] is corrupted in [{rebox_master}]")
            }
        }
        {
            let rebox_sequence = self.0.metadata().rebox_sequence().table_name().as_ref();
            let sequence_store = rkv_env.open_single(rebox_sequence, StoreOptions::default())?;
            let reader = rkv_env.read()?;
            if let Some(_) = sequence_store.get(&reader, table_name_str)? {
                bail!("Health check alert:  Table [{table_name_str}] is corrupted in [{rebox_sequence}]")
            }
        }
        Ok(())
    }
}
