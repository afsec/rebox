use crate::{Driver, DatabaseTables, DatabaseName};

use super::fields::{ReboxSequence, ReboxMaster, ReboxSchema};

mod builder;


#[derive(Debug)]
pub struct DatabaseConnection<D: Driver> {
    driver: D,
    name: DatabaseName,
    rebox_sequence: ReboxSequence,
    rebox_schema: ReboxSchema,
    rebox_master: ReboxMaster,
    tables: DatabaseTables,
}

// impl<D: Driver> DatabaseConnection<D> {
//     pub fn new(driver: D) -> Self {
//         Self { driver }
//     }

//     pub fn list_tables(&self) -> Vec<&TableName> {
//         self.tables.list_tables()
//     }
//     pub fn create_table(&mut self, table: Table) -> ReboxResult<TableName> {
//         let table_name = self.tables.create_table(table)?;

//         Ok(table_name)
//     }
//     pub fn insert_into_table<T: AsRef<str>>(
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

//     pub fn name(&self) -> &str {
//         self.name.as_ref()
//     }
// }
