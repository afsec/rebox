use rebox_types::ReboxResult;

use crate::{drivers::InMemoryDriver, table::Table};

#[test]
fn error_on_create_two_table_with_same_name() -> ReboxResult<()> {
    use crate::database::Database;
    let mut db = Database::new()
        .set_driver(InMemoryDriver)?
        .database_name("rebox-123123123")?
        .build()?;
    let table = Table::new().table_name("tbl1").build();
    let _ = db.create_table(table)?;

    let table = Table::new().table_name("tbl1").build();
    let res = db.create_table(table);

    assert!(res.is_err());

    Ok(())
}

#[test]
fn ok_on_create_two_table_with_diferent_name() -> ReboxResult<()> {
    use crate::database::Database;
    let mut db = Database::new()
        .set_driver(InMemoryDriver)?
        .database_name("rebox-123123123")?
        .build()?;
    let table: Table = Table::new().table_name("tbl1").build();
    let _ = db.create_table(table)?;
    dbg!(&db);
    let table: Table = Table::new().table_name("tbl2").build();
    let res = db.create_table(table);
    dbg!(&db);
    assert!(res.is_ok());

    Ok(())
}
