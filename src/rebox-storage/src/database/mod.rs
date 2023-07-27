pub(crate) mod builder;
mod driver;
mod metadata;
mod name;
pub mod row;

use self::{
    driver::key_value::KeyValueDriver,
    metadata::{rebox_master::ReboxMaster, rebox_sequence::ReboxSequence, rebox_stat::ReboxStat},
    name::DatabaseName,
    row::{column::data::RowData, TableRow},
};
use anyhow::{bail, format_err};
use rebox_types::{
    query::{columns_filter::ColumnsFilter, values_to_match::ValuesToMatch},
    schema::{name::TableName, RowId, Table},
    ReboxResult,
};
use rkv::StoreOptions;
use std::fmt::Debug;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Database {
    name: DatabaseName,
    driver: KeyValueDriver,
    metadata: DatabaseMetadata,
}

impl Database {
    pub const MAX_DB_INPUT_COLS: u16 = 10_000 - Table::MAX_TABLE_INPUT_COLS;
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn list_tables(&self) -> ReboxResult<Vec<TableName>> {
        self.driver.list_tables()
    }

    pub fn create_table(&self, table: Table) -> ReboxResult<TableName> {
        if self.driver.number_of_stores()? <= Self::MAX_DB_INPUT_COLS {
            self.driver.create_table(&table)?;
            Ok(table.name().clone())
        } else {
            bail!("Can't create the table [{}]. Reason: The Database is reaching the MAX_DB_INPUT_COLS limit and add that table would cause overflow.",table.name());
        }
    }

    pub fn drop_table(&self, table_name: &TableName) -> ReboxResult<TableName> {
        self.driver.drop(&table_name)?;
        Ok(table_name.clone())
    }

    pub fn insert_into_table(
        &self,
        table_name: TableName,
        table_row: TableRow,
    ) -> ReboxResult<RowId> {
        table_row.check_verified()?;
        let cur_id = self.driver.insert_into_table(table_name, table_row)?;

        Ok(cur_id)
    }

    pub fn get_table_rows(
        &self,
        table_name: &TableName,
        columns_filter: &ColumnsFilter,
        values_to_match: &ValuesToMatch,
    ) -> ReboxResult<Vec<RowData>> {
        self.driver
            .get_table_rows(table_name, columns_filter, values_to_match)
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
            let _ = k.open_single(store_name, StoreOptions::create())?;
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
