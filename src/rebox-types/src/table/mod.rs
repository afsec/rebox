use std::{collections::BTreeMap, io::Bytes, marker::PhantomData, vec};

use anyhow::bail;
use bytes::{Buf, BufMut, BytesMut};

const COLUMN_MAX_CAPACITY: usize = 1024 * 1024 * 1024 * 50; // 50 MBytes
use crate::{Memory, ReboxResult};
use std::fmt::Debug;

pub trait Driver {}

#[derive(Debug)]
pub struct Database<D: Driver> {
    driver: D,
    database_name: String,
    rebox_sequence: ReboxSequence,
    tables: Vec<Table>,
}

impl<D: Driver> Database<D> {
    pub fn new() -> DatabaseBuilder<D> {
        DatabaseBuilder(PhantomData)
    }
}

pub struct DatabaseBuilder<D: Driver>(PhantomData<D>);
impl<D: Driver> DatabaseBuilder<D> {
    pub fn set_driver(&mut self, driver: D) -> ReboxResult<DatabaseWithDriver<D>> {
        Ok(DatabaseWithDriver { driver })
    }
}

#[derive(Debug, Default)]
pub struct DatabaseWithDriver<D: Driver> {
    driver: D,
}

impl<D: Driver> DatabaseWithDriver<D> {
    pub fn set_session_name<S: AsRef<str>>(
        self,
        session_name: S,
    ) -> ReboxResult<DatabaseWithParams<D>> {
        let Self { driver } = self;
        // TODO
        Ok(DatabaseWithParams {
            driver,
            database_name: session_name.as_ref().to_string(),
        })
    }
}

#[derive(Debug, Default)]
pub struct DatabaseWithParams<D: Driver> {
    driver: D,
    database_name: String,
}
impl<D: Driver> DatabaseWithParams<D> {
    pub fn connect(self) -> ReboxResult<Database<D>> {
        let Self {
            driver,
            database_name,
        } = self;
        // TODO
        Ok(Database {
            driver,
            database_name,
            rebox_sequence: ReboxSequence {
                table_name: TableName::new("rebox_sequence"),
                table_filename: TableFileName::new("rebox_sequence"),
                inner_data: Default::default(),
            },
            tables: vec![],
        })
    }
}

////////////////////

#[derive(Debug, Default)]
pub struct ReboxSequence {
    table_name: TableName,
    table_filename: TableFileName,
    inner_data: BTreeMap<TableName, CurrentRowId>,
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
