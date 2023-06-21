use std::collections::BTreeMap;

use rebox_types::ReboxResult;

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
    master_table: MasterTable,
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
    master_table: MasterTable,
    tables: Vec<Table>,
}

#[derive(Debug, Default)]
pub struct MasterTable(Table);

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
pub struct ColumnStorage(BTreeMap<u32, String>);
