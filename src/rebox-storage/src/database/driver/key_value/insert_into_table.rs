use crate::database::{driver::key_value::helpers::retrieve_last_row_id, row::TableRow};

use super::{helpers::check_row_against_schema, KeyValueDriver};
use anyhow::{bail, format_err};
use rebox_types::{
    schema::{name::TableName, RowId, Table},
    DbPrefix, ReboxResult,
};
use rkv::{backend::SafeModeEnvironment, OwnedValue, Rkv, StoreOptions, Value};

pub(super) struct InsertIntoTable<'a>(&'a KeyValueDriver);
impl<'a> InsertIntoTable<'a> {
    pub(super) fn connect(driver: &'a KeyValueDriver) -> ReboxResult<Self> {
        Ok(Self(driver))
    }

    pub(super) fn insert(self, table_name: TableName, table_row: TableRow) -> ReboxResult<RowId> {
        let store_name_prefix = format!("{}-{}", Table::prefix(), table_name);
        check_row_against_schema(
            self.0.connection(),
            self.0.metadata(),
            &table_name,
            &table_row,
        )?;
        let current_row_id = {
            let mut inner_row_id =
                retrieve_last_row_id(self.0.connection(), self.0.metadata(), &table_name)?;
            inner_row_id.inc()?;
            inner_row_id
        };
        table_row
            .get()
            .iter()
            .try_for_each(|(col_name, tbl_column)| {
                let column_value = tbl_column.value().ok_or(format_err!(
                    "Impossible State at {} {}",
                    file!(),
                    line!()
                ))?;
                let value: OwnedValue = column_value.to_owned().into();
                let store_name_str = format!("{store_name_prefix}_{col_name}");
                self.put_into_store(store_name_str, value, &current_row_id)
            })?;

        self.update_rowid_index(&table_name, &current_row_id)?;
        self.update_sequence(&table_name, &current_row_id)?;
        Ok(current_row_id)
    }

    fn put_into_store<T: AsRef<str>>(
        &self,
        store_name: T,
        value: OwnedValue,
        current_row_id: &RowId,
    ) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let rkv_env = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let store_name_str = store_name.as_ref();

        let store = match rkv_env.open_single(store_name_str, StoreOptions::default()) {
            Ok(inner) => inner,
            Err(err) => bail!("KvStore {store_name_str} not found. Reason:{err}"),
        };

        let mut writer = rkv_env.write()?;
        let key = current_row_id.to_be_bytes();
        store.put(&mut writer, &key, &Value::from(&value))?;
        writer.commit()?;
        Ok(())
    }

    fn update_rowid_index(
        &self,
        table_name: &TableName,
        current_row_id: &RowId,
    ) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let table_rowid_store_str = format!("rebox-{table_name}_rowid");
        let rkv_env: std::sync::RwLockReadGuard<'_, Rkv<SafeModeEnvironment>> = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let key = current_row_id.to_be_bytes();
        let sequence_store =
            rkv_env.open_single(table_rowid_store_str.as_ref(), StoreOptions::default())?;
        let mut writer = rkv_env.write()?;
        sequence_store.put(&mut writer, key, &Value::Bool(true))?;
        writer.commit()?;
        Ok(())
    }
    fn update_sequence(&self, table_name: &TableName, current_row_id: &RowId) -> ReboxResult<()> {
        let created_arc = self.0.connection();
        let rebox_sequence = self.0.metadata().rebox_sequence().table_name().as_ref();

        let rkv_env: std::sync::RwLockReadGuard<'_, Rkv<SafeModeEnvironment>> = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let table_name_str = table_name.as_ref();
        let sequence_store = rkv_env.open_single(rebox_sequence, StoreOptions::default())?;
        let mut writer = rkv_env.write()?;
        sequence_store.put(
            &mut writer,
            table_name_str,
            &Value::U64(u64::from(**current_row_id)),
        )?;
        writer.commit()?;
        Ok(())
    }
}
