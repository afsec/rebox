use rebox_types::ReboxResult;

pub use self::rebox_sequence::ReboxSequence;

mod rebox_sequence;

use crate::{
    drivers::DataStorage,
    table::{Table, TableName},
};

#[derive(Debug)]
pub struct DatabaseTables<DS: DataStorage>(Vec<Table<DS>>);
impl<DS: DataStorage> Default for DatabaseTables<DS> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<DS: DataStorage> DatabaseTables<DS> {
    pub fn list(&self) -> Vec<&TableName> {
        let outcome: Vec<&TableName> = self.0.iter().map(|table| table.name()).collect();
        outcome
    }
    pub fn create_table<'a>(&mut self, table_name: &'a TableName) -> ReboxResult<&'a TableName> {
        Ok(table_name)
    }
}
