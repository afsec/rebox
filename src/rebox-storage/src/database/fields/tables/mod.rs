use std::collections::BTreeMap;

use anyhow::bail;
use rebox_types::{
    schema::{name::TableName, table::TableSchema, CurrentRowId, Table},
    ReboxResult,
};

use crate::database::row::TableRow;

#[cfg(test)]
mod tests;

// TODO
// #[derive(Debug, Default)]
// pub struct DatabaseTables(BTreeMap<TableName, TableSchema>);

// impl DatabaseTables {
//     pub fn list_tables(&self) -> Vec<&TableName> {
//         let outcome: Vec<&TableName> = self.0.keys().collect();
//         outcome
//     }
//     pub fn create_table(&mut self, table: Table) -> ReboxResult<TableName> {
//         let (name, schema) = table.take();
//         if self.0.contains_key(&name) {
//             bail!("Table {} is already in Database", name);
//         }
//         self.0.insert(name.clone(), schema);

//         Ok(name)
//     }
//     pub fn insert_into_table(
//         &mut self,
//         table_name: TableName,
//         table_row: TableRow,
//     ) -> ReboxResult<CurrentRowId> {
//         // TODO
//         todo!();
//         Ok(0.into())
//     }
// }
