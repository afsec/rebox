pub mod builder;
pub mod fields;
pub mod row;

use std::fmt::Debug;

use anyhow::format_err;

use rebox_types::{
    schema::{name::TableName, CurrentRowId, Table},
    ReboxResult,
};

use crate::drivers::key_value::KeyValueDriver;

use self::{
    fields::{
        name::DatabaseName, rebox_master::ReboxMaster, rebox_schema::ReboxSchema,
        rebox_sequence::ReboxSequence,
    },
    row::TableRow,
};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Database {
    name: DatabaseName,
    driver: KeyValueDriver,
    metadata: DatabaseMetadata,
}

impl Database {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn list_tables(&self) -> ReboxResult<Vec<&TableName>> {
        // self.driver.list_tables()
        todo!();
        Ok(vec![])
    }
    pub fn create_table(&self, table: Table) -> ReboxResult<TableName> {
        // self.driver.open_table(&table, true)?;
        todo!();
        Ok(table.name().to_owned())
    }
    pub fn insert_into_table(
        &self,
        table_name: TableName,
        table_row: TableRow,
    ) -> ReboxResult<CurrentRowId> {
        todo!();
        Ok(CurrentRowId::default())
    }
    fn bootstrap_metadata(&self) -> ReboxResult<()> {
        // TODO: Implement new/open session
        self.create_metadata_table(&self.metadata.rebox_schema)?;
        self.create_metadata_table(&self.metadata.rebox_master)?;
        self.create_metadata_table(&self.metadata.rebox_sequence)?;
        Ok(())
    }
    fn create_metadata_table<T: MetadataTable>(&self, metadata_table: &T) -> ReboxResult<()> {
        use rkv::{StoreOptions, Value};
        let table_name = metadata_table.table_name();
        let created_arc = self.driver.connection();
        let k = created_arc.read().unwrap();
        let store = k.open_single(table_name.to_string().as_str(), StoreOptions::create())?;
        let mut writer = k.write()?;
        // store.put(&mut writer, "some_key", &Value::Str("some_value"))?;
        writer.commit().map_err(|err| format_err!("{err}"))?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct DatabaseMetadata {
    rebox_sequence: ReboxSequence,
    rebox_schema: ReboxSchema,
    rebox_master: ReboxMaster,
}

pub trait MetadataTable {
    fn table_name(&self) -> &TableName;
}
