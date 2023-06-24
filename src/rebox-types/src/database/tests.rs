use crate::{database::Database, ReboxResult};

#[test]
fn ok_on_new_database_with_valid_name() -> ReboxResult<()> {
    let res = Database::new().set_name("rebox-123123123");

    assert!(res.is_ok());

    let res = res?.build();

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn error_on_new_database_with_invalid_name() -> ReboxResult<()> {
    let res = Database::new().set_name("rebox_123123123");

    assert!(res.is_err());

    Ok(())
}

#[test]
fn ok_on_new_database_with_columns() -> ReboxResult<()> {
    let res = Database::new().set_name("rebox-123123123")?.build();

    assert!(res.is_ok());

    Ok(())
}
