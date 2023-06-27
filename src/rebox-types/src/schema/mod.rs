use anyhow::bail;
use std::fmt::Debug;

use crate::{helpers::check_valid_entity_name, ReboxResult};

pub use self::{
    column::{ColumnKind, ColumnName, SchemaColumn},
    name::TableName,
    table::TableSchema,
};

// pub use self::{column::SchemaColumn, name::TableName};

mod column;
mod name;
mod table;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentRowId(u32);

impl From<u32> for CurrentRowId {
    fn from(value: u32) -> Self {
        Self(value)
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
        self.0 >= u32::MAX
    }
}

#[derive(Debug, Default, Clone)]
pub struct Table {
    name: TableName,
    schema: TableSchema,
}

impl Table {
    pub fn name(&self) -> &TableName {
        &self.name
    }
    pub fn take(self) -> (TableName, TableSchema) {
        let Self { name, schema } = self;
        (name, schema)
    }
}

impl Table {
    pub fn new() -> TableBuilder {
        TableBuilder
    }
}
pub struct TableBuilder;

impl TableBuilder {
    pub fn set_name<T: AsRef<str>>(self, name: T) -> ReboxResult<TableBuilderS1> {
        check_valid_entity_name(&name)?;
        Ok(TableBuilderS1 {
            name: TableName::new(name),
        })
    }
}

pub struct TableBuilderS1 {
    name: TableName,
}

impl TableBuilderS1 {
    pub fn set_schema(self, columns: Vec<SchemaColumn>) -> ReboxResult<TableBuilderS2> {
        if columns.len() < 1 {
            bail!("Sorry, you must to define at least one SchemaColumn")
        }
        let mut schema = TableSchema::default();
        let Self { name } = self;
        columns
            .into_iter()
            .map(|column| schema.add_column(column))
            .collect::<ReboxResult<()>>()?;

        Ok(TableBuilderS2 { name, schema })
    }
}

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
