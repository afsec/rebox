use std::{collections::BTreeMap, io::Bytes};

use anyhow::bail;
use bytes::{Buf, BufMut, BytesMut};

use rebox_types::ReboxResult;

const COLUMN_MAX_CAPACITY: usize = 1024 * 1024 * 1024 * 50; // 50 MBytes

#[derive(Debug)]
struct MaxSize(usize);
impl Default for MaxSize {
    /// 4 GBytes
    fn default() -> Self {
        // Supporting from 32 bit architectures
        Self(u32::MAX as usize)
    }
}

#[derive(Debug, Default)]
pub struct DriverMemory {
    master_table: ReboxMaster,
    tables: Vec<Table>,
    max_size: MaxSize,
}

impl DriverMemory {
    pub fn current_size(&self) -> usize {
        // TODO
        std::mem::size_of::<Self>()
    }
    pub fn run(self) -> ReboxResult<()> {
        println!("Hello from: {} at line {}.", file!(), line!());
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Database {
    master_table: ReboxMaster,
    tables: Vec<Table>,
}

#[derive(Debug, Default)]
pub struct ReboxMaster {
    table_name: TableName,
    table_metadata_filename: TableFileName,
    tables: BTreeMap<TableName, CurrentRowId>,
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

#[derive(Debug, Default)]
pub struct TableFileName(String);

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
