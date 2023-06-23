use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::drivers::DataStorage;

pub use self::columns::{Column, ColumnContent};
pub use self::file_name::TableFileName;
pub use self::name::TableName;

mod columns;
mod file_name;
mod name;


#[derive(Debug, Default)]
pub struct CurrentRowId(u32);

#[derive(Debug, Default)]
pub struct Table<DS: DataStorage> {
    name: TableName,
    metadata_filename: TableFileName,
    columns: Vec<Column<DS>>,
}

impl<DS: DataStorage> Table<DS> {
    pub fn name(&self) -> &TableName {
        &self.name
    }
}
