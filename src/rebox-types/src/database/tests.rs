use crate::{database::Database, ReboxResult};

const DB_NAME: &str = "rebox-123123123";

#[test]
fn ok_on_new_database_with_valid_name() -> ReboxResult<()> {
    let res = Database::new().database_name(DB_NAME);

    assert!(res.is_ok());
    Ok(())
}

#[test]
fn erro_on_new_database_with_invalid_name() -> ReboxResult<()> {
    let res = Database::new().database_name(DB_NAME);

    assert!(res.is_err());
    Ok(())
}
