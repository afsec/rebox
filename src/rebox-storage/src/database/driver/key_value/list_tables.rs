use super::KeyValueDriver;
use anyhow::format_err;
use rebox_types::{schema::name::TableName, ReboxResult};
use rkv::StoreOptions;

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

        let rebox_master = self.0.metadata().rebox_master().table_name().as_ref();
        let master_store = k.open_single(rebox_master, StoreOptions::default())?;
        let reader = k.read()?;
        let tables = master_store
            .iter_start(&reader)?
            .map(|result_retrieved| {
                let (key_raw, _) = result_retrieved?;
                let table_name = TableName::new(String::from_utf8_lossy(key_raw));
                Ok(table_name)
            })
            .collect::<ReboxResult<Vec<TableName>>>();

        Ok(tables?)
    }
}
