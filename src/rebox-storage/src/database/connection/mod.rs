use crate::drivers::Driver;

use super::{
    fields::{ReboxMaster, ReboxSchema, ReboxSequence},
    DatabaseName, DatabaseTables,
};

mod builder;

#[derive(Debug)]
pub(crate) struct DatabaseConnection<D: Driver> {
    driver: D,
    name: DatabaseName,
    rebox_sequence: ReboxSequence,
    rebox_schema: ReboxSchema,
    rebox_master: ReboxMaster,
    tables: DatabaseTables,
}

// impl<D: Driver> DatabaseConnection<D> {
//     pub (crate) fn new(driver: D) -> Self {
//         Self { driver }  'q12e4r5t6y7890-
//     }

//     pub (crate) fn list_tables(&self) -> Vec<&TableName> {
//         self.tables.list_tables()
//     }
//     pub (crate) fn create_table(&mut self, table: Table) -> ReboxResult<TableName> {
//         let table_name = self.tables.create_table(table)?;

//         Ok(table_name)
//     }
//     pub (crate) fn insert_into_table<T: AsRef<str>>(
//         &mut self,
//         table: T,
//         row: TableRow,
//     ) -> ReboxResult<CurrentRowId> {
//         let table_name = TableName::new(table.as_ref());
//         self.rebox_sequence.check_can_inc_rowid(&table_name)?;
//         let cur_row_id = self.tables.insert_into_table(table_name.to_owned(), row)?;
//         self.rebox_sequence.bump_table_cur_rowid(&table_name)?;
//         Ok(cur_row_id)
//     }

//     pub (crate) fn name(&self) -> &str {
//         self.name.as_ref()
//     }
// }
