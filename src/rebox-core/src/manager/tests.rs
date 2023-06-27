use rebox_storage::KeyValueDriver;
use rebox_types::{
    database::{
        row::{ColumnValue, TableColumn, TableRow},
        Database,
    },
    schema::{
        column::{ColumnKind, SchemaColumn},
        Table,
    },
    ReboxResult,
};
use test_case::test_case;

use crate::manager::Manager;

#[test_case(vec!["db-name1","db-name2"] ; "when creating one table for earch of them")]
fn digging_manager(database_names: Vec<&str>) -> ReboxResult<()> {
    let driver = KeyValueDriver;
    let rebox_instances = database_names
        .iter()
        .map(|db_name| {
            Manager::new()
                .set_driver(driver.clone())?
                .set_database_name(db_name)?
                .build()
        })
        .collect::<ReboxResult<Vec<Manager<KeyValueDriver>>>>()?;
    dbg!(rebox_instances);

    Ok(())
}
