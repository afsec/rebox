pub(crate) mod builder;
mod driver;
mod metadata;
mod name;
mod row;
use std::fmt::Debug;

use anyhow::format_err;

use rkv::StoreOptions;

use rebox_types::{
    schema::{name::TableName, CurrentRowId, Table},
    ReboxResult,
};

use self::metadata::{
    rebox_master::ReboxMaster, rebox_sequence::ReboxSequence, rebox_stat::ReboxStat,
};
use self::name::DatabaseName;
use self::{driver::key_value::KeyValueDriver, row::TableRow};

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
    pub fn list_tables(&self) -> ReboxResult<Vec<TableName>> {
        self.driver.list_tables()
    }
    pub fn create_table(&self, table: Table) -> ReboxResult<TableName> {
        self.driver.create_table(&table)?;
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
        self.metadata
            .into_iter()
            .try_for_each(|table| self.create_metadata_table(table))?;

        Ok(())
    }
    fn create_metadata_table<T: MetadataTable + ?Sized>(
        &self,
        metadata_table: Box<&T>,
    ) -> ReboxResult<()> {
        let created_arc = self.driver.connection();
        let k = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let store_name = metadata_table.table_name().as_ref();

        if k.open_single(store_name, StoreOptions::default()).is_err() {
            let created_store = k.open_single(store_name, StoreOptions::create());

            let mut writer = k.write()?;
            // created_store?.put(&mut writer, "some_key", &Value::Str("some_value"))?;
            writer.commit().map_err(|err| format_err!("{err}"))?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub(super) struct DatabaseMetadata {
    rebox_master: ReboxMaster,
    rebox_sequence: ReboxSequence,
    rebox_stat: ReboxStat,
}

impl DatabaseMetadata {
    pub(super) fn rebox_master(&self) -> &ReboxMaster {
        &self.rebox_master
    }

    pub(super) fn rebox_sequence(&self) -> &ReboxSequence {
        &self.rebox_sequence
    }

    pub(super) fn rebox_stat(&self) -> &ReboxStat {
        &self.rebox_stat
    }
}

impl<'a> IntoIterator for &'a DatabaseMetadata {
    type Item = Box<&'a dyn MetadataTable>;

    type IntoIter = DatabaseMetadataIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DatabaseMetadataIntoIterator {
            inner: self,
            index: 0,
        }
    }
}

pub(super) struct DatabaseMetadataIntoIterator<'a> {
    inner: &'a DatabaseMetadata,
    index: usize,
}

impl<'a> Iterator for DatabaseMetadataIntoIterator<'a> {
    type Item = Box<&'a dyn MetadataTable>;
    fn next(&mut self) -> Option<Box<&'a dyn MetadataTable>> {
        let outcome = match self.index {
            0 => Box::new(&self.inner.rebox_master as &dyn MetadataTable),
            1 => Box::new(&self.inner.rebox_sequence as &dyn MetadataTable),
            2 => Box::new(&self.inner.rebox_stat as &dyn MetadataTable),
            _ => return None,
        };
        self.index += 1;
        Some(outcome)
    }
}

pub(super) trait MetadataTable: Debug {
    fn table_name(&self) -> &TableName;
}
