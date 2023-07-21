use super::{helpers::retrieve_schema, KeyValueDriver};
use crate::database::row::column::data::{ColumnData, RowData};
use anyhow::{bail, format_err};
use rebox_types::{
    schema::{column::model::ColumnValue, name::TableName, RowId, Table},
    DbPrefix, ReboxResult,
};
use rkv::{OwnedValue, StoreOptions};

pub(super) struct GetTableSingleRow<'a>(&'a KeyValueDriver);
impl<'a> GetTableSingleRow<'a> {
    pub(super) fn connect(driver: &'a KeyValueDriver) -> ReboxResult<Self> {
        Ok(Self(driver))
    }

    pub(super) fn get(
        self,
        table_name: &TableName,
        row_id: &RowId,
    ) -> ReboxResult<Option<RowData>> {
        let store_name_prefix = format!("{}-{}", Table::prefix(), table_name);
        let tbl_schema = retrieve_schema(self.0.connection(), self.0.metadata(), &table_name)?;
        let schema_cols = tbl_schema.get_columns();

        let outcome = schema_cols
            .iter()
            .map(|(col_name, tbl_column)| {
                let store_name_str = format!("{store_name_prefix}_{col_name}");
                let kind = tbl_column.kind();
                let value: ColumnValue = self.get_from_store(store_name_str, row_id)?.try_into()?;
                if value != *kind {
                    bail!("Incompatiple types between ColumnKind and ColumnValue");
                }
                let data = ColumnData::new()
                    .set_row_id(row_id.clone())
                    .set_col_name(col_name)
                    .set_value(value)
                    .build();
                Ok(data)
            })
            .collect::<ReboxResult<Vec<ColumnData>>>()?;

        if outcome.is_empty() {
            Ok(None)
        } else {
            let table_row = RowData::from(outcome);
            Ok(Some(table_row))
        }
    }

    fn get_from_store<T: AsRef<str>>(
        &self,
        store_name: T,
        row_id: &RowId,
    ) -> ReboxResult<OwnedValue> {
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
        let key = row_id.to_be_bytes();
        let value = store
            .get(&mut writer, &key)?
            .ok_or(format_err!("Table corrupted. Column data not found."))?;

        Ok((&value).into())
    }
}
