use crate::ReboxResult;

use crate::schema::column::{ColumnKind, TableColumn};
use crate::schema::Table;

#[test]
fn error_on_define_new_table_with_no_column() -> ReboxResult<()> {
    use crate::database::Database;
    let db = Database::new().set_name("rebox-123123123")?.build()?;
    let res = Table::new().set_name("tbl1")?.build();

    assert!(res.is_err());

    Ok(())
}

#[test]
fn ok_on_create_table() -> ReboxResult<()> {
    use crate::database::Database;
    let mut db = Database::new().set_name("rebox-123123123")?.build()?;
    let c1 = TableColumn::new()
        .set_name("c1")?
        .set_kind(ColumnKind::Natural)
        .is_nullable(false)
        .build();
    let c1 = TableColumn::new()
        .set_name("c2")?
        .set_kind(ColumnKind::Text)
        .is_nullable(false)
        .build();
    let table = Table::new().set_name("tbl1")?.set_column(c1)?.build()?;
    let res = db.create_table(table);

    dbg!(&db);

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn error_on_create_two_table_with_same_name() -> ReboxResult<()> {
    use crate::database::Database;
    let mut db = Database::new().set_name("rebox-123123123")?.build()?;
    {
        let c1 = TableColumn::new()
            .set_name("c1")?
            .set_kind(ColumnKind::Text)
            .is_nullable(false)
            .build();
        let table = Table::new().set_name("tbl1")?.set_column(c1)?.build()?;
        let _ = db.create_table(table)?;
    }

    let c1 = TableColumn::new()
        .set_name("c1")?
        .set_kind(ColumnKind::Text)
        .is_nullable(false)
        .build();
    let table = Table::new().set_name("tbl1")?.set_column(c1)?.build()?;

    let res = db.create_table(table);

    assert!(res.is_err());

    Ok(())
}

#[test]
fn ok_on_create_two_table_with_diferent_name() -> ReboxResult<()> {
    use crate::database::Database;
    let mut db = Database::new().set_name("rebox-123123123")?.build()?;
    {
        let c1 = TableColumn::new()
            .set_name("c1")?
            .set_kind(ColumnKind::Text)
            .is_nullable(false)
            .build();
        let table = Table::new().set_name("tbl1")?.set_column(c1)?.build()?;
        let _ = db.create_table(table)?;
    }

    let c1 = TableColumn::new()
        .set_name("c1")?
        .set_kind(ColumnKind::Text)
        .is_nullable(false)
        .build();
    let table = Table::new().set_name("tbl2")?.set_column(c1)?.build()?;

    let res = db.create_table(table);

    assert!(res.is_ok());

    Ok(())
}
