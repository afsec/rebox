use anyhow::{bail, format_err};
use rebox_types::{
    schema::{name::TableName, table::TableSchema},
    ReboxResult,
};
use rkv::{StoreOptions, Value};

use super::KeyValueDriver;

pub(super) struct ListTables<'a>(&'a KeyValueDriver);
impl<'a> ListTables<'a> {
    pub(super) fn connect(driver: &'a KeyValueDriver) -> ReboxResult<Self> {
        Ok(Self(driver))
    }
    pub(super) fn list(self) -> ReboxResult<Vec<TableName>> {
        let created_arc = self.0.connection();

        let k = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        use bincode::config::Configuration;

        let created_arc = self.0.connection();

        let k = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;

        let rebox_master = self.0.metadata().rebox_master().table_name().as_ref();
        let master_store = k.open_single(rebox_master, StoreOptions::default())?;
        let reader = k.read()?;
        let tables = master_store
            .iter_start(&reader)?
            .map(|result_retrieved| {
                let (key_raw, value) = result_retrieved?;
                let table_name = TableName::new(String::from_utf8_lossy(key_raw));
                // let blob = match value {
                //     Value::Blob(inner_blob) => inner_blob,
                //     other => bail!(
                //     "Health check alert: Table [{table_name}] type mismatch in [{rebox_master}]. Reason: {other:?}"
                // ),
                // };
                // let (retrieved_table_schema, _) = bincode::decode_from_slice::<
                //     TableSchema,
                //     Configuration,
                // >(blob, bincode::config::standard())?;

                // if &retrieved_table_schema != table.schema() {
                //     bail!("Health check alert:  Table [{table_name}] is corrupted in [{rebox_master}]")
                // }
                Ok(table_name)
            })
            .collect::<ReboxResult<Vec<TableName>>>();

        // let tables = k
        //     .get_dbs()?
        //     .into_iter()
        //     .filter_map(|maybe_item| maybe_item.map(|tbl_name| TableName::new(tbl_name)))
        //     .collect();

        Ok(tables?)
    }
}
