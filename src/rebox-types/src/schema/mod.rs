use self::{column::SchemaColumn, name::TableName, table::TableSchema};
use crate::{helpers::check_valid_entity_name, DbPrefix, ReboxResult};
use anyhow::bail;
use rebox_derive::DbEntity;
use std::{fmt::Debug, ops::Deref};

pub mod column;
pub mod name;
pub mod table;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentRowId(u32);

impl Deref for CurrentRowId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u32> for CurrentRowId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<u64> for CurrentRowId {
    type Error = anyhow::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < (u32::MAX as u64) {
            Ok(Self(value as u32))
        } else {
            bail!("Value is out of bounds. Reason: (value > u32::MAX).");
        }
    }
}

impl CurrentRowId {
    pub fn inc(&mut self) -> ReboxResult<()> {
        if self.is_full() {
            bail!("Max limit reached for CurrentRowId");
        } else {
            self.0 += 1;
            Ok(())
        }
    }
    pub fn is_full(&self) -> bool {
        self.0 == u32::MAX
    }
}

#[derive(Debug, Default, Clone, DbEntity)]
pub struct Table {
    name: TableName,
    schema: TableSchema,
}

impl Table {
    pub fn new() -> TableBuilder {
        TableBuilder
    }
    pub fn name(&self) -> &TableName {
        &self.name
    }
    pub fn take(self) -> (TableName, TableSchema) {
        let Self { name, schema } = self;
        (name, schema)
    }

    pub fn schema(&self) -> &TableSchema {
        &self.schema
    }
}

#[derive(Debug)]
pub struct TableBuilder;

impl TableBuilder {
    pub fn set_name<T: AsRef<str>>(self, name: T) -> ReboxResult<TableBuilderS1> {
        check_valid_entity_name(&name)?;
        Ok(TableBuilderS1 {
            name: TableName::new(name),
        })
    }
}

#[derive(Debug)]
pub struct TableBuilderS1 {
    name: TableName,
}

impl TableBuilderS1 {
    pub fn set_schema(self, columns: Vec<SchemaColumn>) -> ReboxResult<TableBuilderS2> {
        if columns.is_empty() {
            bail!("Sorry, you must to define at least one SchemaColumn")
        }
        let mut schema = TableSchema::default();
        let Self { name } = self;
        columns
            .into_iter()
            .try_for_each(|column| schema.add_column(column))?;

        Ok(TableBuilderS2 { name, schema })
    }
}

#[derive(Debug)]
pub struct TableBuilderS2 {
    name: TableName,
    schema: TableSchema,
}

impl TableBuilderS2 {
    pub fn build(self) -> ReboxResult<Table> {
        let Self { name, schema } = self;
        if schema.count_columns() > 0 {
            Ok(Table { name, schema })
        } else {
            bail!("Can't build a table without column")
        }
    }
}
