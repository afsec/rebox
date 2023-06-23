use std::collections::BTreeMap;

use crate::table::{CurrentRowId, TableFileName, TableName};

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
