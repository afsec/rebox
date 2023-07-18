use super::KeyValueDriver;
use anyhow::{bail, format_err};
use bincode::config::Configuration;
use rebox_types::{
    schema::{table::TableSchema, Table},
    ReboxResult,
};
use rkv::{StoreOptions, Value};

pub(super) struct TableExists<'a>(&'a KeyValueDriver);
impl<'a> TableExists<'a> {
    pub(super) fn connect(driver: &'a KeyValueDriver) -> ReboxResult<Self> {
        Ok(Self(driver))
    }
    pub(super) fn exists(self, table: &Table) -> ReboxResult<bool> {
        let created_arc = self.0.connection();
        let rkv_env = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let table_name_str = table.name().as_ref();
        let rebox_master = self.0.metadata().rebox_master().table_name().as_ref();
        let master_store = rkv_env.open_single(rebox_master, StoreOptions::default())?;
        let reader = rkv_env.read()?;
        let maybe_value: Option<Value> = match master_store.get(&reader, table_name_str) {
            Ok(Some(inner_value)) => Some(inner_value),
            _ => return Ok(false),
        };

        let blob = match maybe_value {
                Some(Value::Blob(inner_blob)) => inner_blob,
                other => bail!(
                "Health check alert: Table [{table_name_str}] type mismatch in [{rebox_master}]. Reason: {other:?}"
            ),
            };
        let (retrieved_table_schema, _) = bincode::decode_from_slice::<TableSchema, Configuration>(
            blob,
            bincode::config::standard(),
        )?;
        if &retrieved_table_schema == table.schema() {
            Ok(true)
        } else {
            bail!("Health check alert:  Table [{table_name_str}] is corrupted in [{rebox_master}]")
        }
    }
}
