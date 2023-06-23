use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::vec;

use crate::ReboxResult;
use anyhow::bail;

pub use self::columns::Column;
use self::columns::ColumnDesc;
pub use self::file_name::TableFileName;
pub use self::name::TableName;

mod columns;
mod file_name;
mod name;

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
pub struct TableRow(BTreeSet<Column>);
impl TableRow {}

#[derive(Debug, Default)]
pub struct Table {
    name: TableName,
    filename: TableFileName,
    schema: Vec<ColumnDesc>,
}

impl Table {
    pub fn name(&self) -> &TableName {
        &self.name
    }
}

impl Table {
    pub fn new() -> TableBuilder {
        TableBuilder
    }
}
pub struct TableBuilder;

impl TableBuilder {
    pub fn table_name<T: AsRef<str>>(self, name: T) -> TableBuilderWtbName {
        let tb_name = name.as_ref();
        TableBuilderWtbName {
            name: TableName::from(tb_name.to_string()),
            filename: TableFileName::from(tb_name.to_string()),
            schema: vec![],
        }
    }
}

pub struct TableBuilderWtbName {
    name: TableName,
    filename: TableFileName,
    schema: Vec<ColumnDesc>,
}

impl TableBuilderWtbName {
    pub fn set_column(self, column: ColumnDesc) -> Self {
        let Self {
            name,
            filename,
            mut schema,
        } = self;
        schema.push(column);
        Self {
            name,
            filename,
            schema,
        }
    }
    pub fn build(self) -> Table {
        let Self {
            name,
            filename,
            schema,
        } = self;

        Table {
            name,
            filename,
            schema,
        }
    }
}
