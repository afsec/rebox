use self::{column::SchemaColumn, name::TableName, schema::TableSchema};
use crate::{helpers::check_valid_entity_name, DbPrefix, ReboxResult};
use anyhow::bail;
use rebox_derive::DbEntity;
use serde::Serialize;
use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

pub mod column;
pub mod name;
pub mod schema;

#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct RowId(u32);

impl Deref for RowId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u32> for RowId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<RowId> for u64 {
    fn from(row_id: RowId) -> u64 {
        u64::from(row_id.0)
    }
}

impl TryFrom<u64> for RowId {
    type Error = anyhow::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < (u32::MAX as u64) {
            Ok(Self(value as u32))
        } else {
            bail!("Value is out of bounds. Reason: (value > u32::MAX).");
        }
    }
}

impl Display for RowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl RowId {
    pub fn inc(&mut self) -> ReboxResult<()> {
        if self.is_full() {
            bail!("Max limit reached for RowId");
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
    pub const MAX_TABLE_INPUT_COLS: u16 = 255;
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
    pub fn name<T: AsRef<str>>(self, name: T) -> ReboxResult<TableBuilderS1> {
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
    pub fn schema(self, columns: Vec<SchemaColumn>) -> ReboxResult<TableBuilderS2> {
        if columns.is_empty() {
            bail!("A table must have at least one SchemaColumn")
        }

        let max_tbl_columns = Table::MAX_TABLE_INPUT_COLS.into();
        if columns.len() > max_tbl_columns {
            bail!("A table cannot have more than `{max_tbl_columns}` columns")
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
