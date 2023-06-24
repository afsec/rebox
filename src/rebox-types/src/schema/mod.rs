use anyhow::bail;
use std::{collections::BTreeSet, fmt::Debug};

use crate::{helpers::check_valid_name, ReboxResult};

use self::{column::TableColumn, name::TableName, schema::TableSchema};

// pub use self::{column::TableColumn, name::TableName};

pub mod column;
pub mod name;
pub mod schema;

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

#[derive(Debug, Default)]
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
    pub fn set_name<T: AsRef<str>>(self, name: T) -> ReboxResult<TableBuilderWtbName> {
        check_valid_name(&name)?;
        Ok(TableBuilderWtbName {
            name: TableName::new(name),
            schema: Default::default(),
        })
    }
}

pub struct TableBuilderWtbName {
    name: TableName,
    schema: TableSchema,
}

impl TableBuilderWtbName {
    pub fn set_column(self, column: TableColumn) -> ReboxResult<Self> {
        let Self { name, mut schema } = self;
        schema.add_column(column)?;

        Ok(Self { name, schema })
    }
    pub fn build(self) -> ReboxResult<Table> {
        let Self { name, schema } = self;
        if schema.count_columns() > 0 {
            Ok(Table { name, schema })
        } else {
            bail!("Can't build a table without column")
        }
    }
}

#[derive(Debug, Default)]
pub struct TableRow(TableSchema);
impl TableRow {}
