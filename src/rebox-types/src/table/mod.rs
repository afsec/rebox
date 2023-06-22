use crate::ReboxResult;
use anyhow::bail;
use bytes::{Buf, BufMut, BytesMut};
use std::collections::BTreeMap;
use std::fmt::Debug;

const COLUMN_MAX_CAPACITY: usize = 1024 * 1024 * 1024 * 50; // 50 MBytes

////////////////////

#[derive(Debug)]
pub struct ReboxSequence {
    table_name: TableName,
    table_filename: TableFileName,
    inner_data: BTreeMap<TableName, CurrentRowId>,
}
impl Default for ReboxSequence {
    fn default() -> Self {
        Self {
            table_name: TableName::new("rebox_sequence"),
            table_filename: TableFileName::new("rebox_sequence"),
            inner_data: Default::default(),
        }
    }
}
#[derive(Debug, Default)]
pub struct CurrentRowId(u32);

#[derive(Debug, Default)]
pub struct Table {
    table_name: TableName,
    table_metadata_filename: TableFileName,
    columns: Vec<Column>,
}

#[derive(Debug, Default)]
pub struct TableName(String);

impl TableName {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self(name.as_ref().to_string())
    }
}

#[derive(Debug, Default)]
pub struct TableFileName(String);
impl TableFileName {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self(name.as_ref().to_string())
    }
}

#[derive(Debug, Default)]
pub struct Column {
    column_name: ColumnName,
    column_storage: ColumnStorage,
}

#[derive(Debug, Default)]
pub struct ColumnName(String);

#[derive(Debug, Default)]
pub struct ColumnStorage(BTreeMap<u32, ColumnContent>);

#[derive(Debug, Default)]
pub struct ColumnContent(BytesMut);

impl ColumnContent {
    pub fn store(mut self, payload: Box<dyn Buf>) -> ReboxResult<()> {
        if self.0.capacity() >= COLUMN_MAX_CAPACITY {
            bail!("Out ot space inside a column")
        }
        self.0.put(payload);
        Ok(())
    }
}
