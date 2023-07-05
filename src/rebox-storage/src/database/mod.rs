pub(crate) mod builder;
mod driver;
mod metadata;
mod name;
mod row;
use std::{fmt::Debug, marker::PhantomData, pin::Pin};

use anyhow::format_err;

use rebox_types::{
    schema::{name::TableName, CurrentRowId, Table},
    ReboxResult,
};

use self::metadata::{
    rebox_schema::ReboxSchema, rebox_sequence::ReboxSequence, rebox_stat::ReboxStat,
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
        self.metadata
            .into_iter()
            .try_for_each(|table| self.create_metadata_table(table))?;

        Ok(())
    }
    fn create_metadata_table<T: MetadataTable + ?Sized>(
        &self,
        metadata_table: Box<&T>,
    ) -> ReboxResult<()> {
        use rkv::{StoreOptions, Value};
        let created_arc = self.driver.connection();
        let k = created_arc.read().unwrap();
        let store_name = format!("{}-{}", self.metadata.prefix, metadata_table.table_name());

        if k.open_single(&*store_name, StoreOptions::default())
            .is_err()
        {
            let created_store = k.open_single(&*store_name, StoreOptions::create());

            let mut writer = k.write()?;
            // created_store?.put(&mut writer, "some_key", &Value::Str("some_value"))?;
            writer.commit().map_err(|err| format_err!("{err}"))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct DatabaseMetadata {
    prefix: &'static str,
    rebox_schema: ReboxSchema,
    rebox_sequence: ReboxSequence,
    rebox_stat: ReboxStat,
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
            0 => Box::new(&self.inner.rebox_schema as &dyn MetadataTable),
            1 => Box::new(&self.inner.rebox_sequence as &dyn MetadataTable),
            2 => Box::new(&self.inner.rebox_stat as &dyn MetadataTable),
            _ => return None,
        };
        self.index += 1;
        Some(outcome)
    }
}

impl Default for DatabaseMetadata {
    fn default() -> Self {
        Self {
            prefix: "rebox",
            rebox_stat: Default::default(),
            rebox_schema: Default::default(),
            rebox_sequence: Default::default(),
            // iter_idx: 0,
            // iter_cur: None
        }
    }
}

pub(super) trait MetadataTable: Debug {
    fn table_name(&self) -> &TableName;
}
