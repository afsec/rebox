use std::{collections::BTreeMap, io::Bytes};

use anyhow::bail;
use bytes::{Buf, BufMut, BytesMut};

const COLUMN_MAX_CAPACITY: usize = 1024 * 1024 * 1024 * 50; // 50 MBytes
use crate::ReboxResult;

#[derive(Debug, Default)]
pub struct Database<D: Driver>(DatabaseState<D>);

impl<D: Driver> Database<D> {
    pub fn run(self) -> ReboxResult<()> {
        println!("Hello from: {} at line {}.", file!(), line!());
        Ok(())
    }
}

pub trait Driver {}

#[derive(Debug, Default)]
pub struct DatabaseOps<D: Driver> {
    driver: D,
}

#[derive(Debug, Default)]
pub enum DatabaseState<D: Driver> {
    #[default]
    NotStarted,
    Opening(D),
    Ready(DatabaseReady<D>),
}

#[derive(Debug, Default)]
pub struct DatabaseReady<D: Driver> {
    driver: D,
    master_table: ReboxSequence,
    tables: Vec<Table>,
}

////////////////////

#[derive(Debug, Default)]
pub struct ReboxSequence {
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
