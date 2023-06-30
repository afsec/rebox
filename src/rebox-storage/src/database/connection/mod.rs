use rebox_types::{
    schema::{name::TableName, CurrentRowId, Table},
    ReboxResult,
};

use crate::drivers::Driver;

use super::{
    fields::{
        name::DatabaseName,
        rebox_master::ReboxMaster,
        rebox_schema::ReboxSchema,
        rebox_sequence::ReboxSequence, // , tables::DatabaseTables,
    },
    row::TableRow,
};

mod builder;

// pub use builder::{
//     DatabaseConnectionBuilder, DatabaseConnectionBuilderS1, DatabaseConnectionBuilderS2,
// };

#[derive(Debug)]
pub struct DatabaseConnection<D: Driver> {
    driver: D,
    name: DatabaseName,
    rebox_sequence: ReboxSequence,
    rebox_schema: ReboxSchema,
    rebox_master: ReboxMaster,
}

impl<D: Driver> DatabaseConnection<D> {
    pub(crate) fn list_tables(&self) -> ReboxResult<Vec<&TableName>> {
        todo!();
        Ok(vec![])
    }
    pub(crate) fn create_table(&mut self, table: Table) -> ReboxResult<TableName> {
        todo!();
        Ok(TableName::default())
    }
    pub(crate) fn insert_into_table<T: AsRef<str>>(
        &mut self,
        table: T,
        row: TableRow,
    ) -> ReboxResult<CurrentRowId> {
        todo!();
        Ok(CurrentRowId::default())
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }
}
