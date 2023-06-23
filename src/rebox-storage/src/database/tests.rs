use rebox_types::ReboxResult;

use crate::drivers::{InMemoryDriver, InMemoryStorage, KeyValue, KeyValueStorage};

const SESSION_NAME: &str = "rebox-123123123";

#[test]
fn new_in_memory() -> ReboxResult<()> {
    use crate::database::Database;
    let db = Database::new()
        .set_driver(InMemoryDriver)?
        .database_name(SESSION_NAME)?
        .build()?;

    assert_eq!(db.driver(), &InMemoryDriver);
    Ok(())
}

#[test]
fn new_kv() -> ReboxResult<()> {
    use crate::database::Database;
    let db = Database::new()
        .set_driver(KeyValue)?
        .database_name(SESSION_NAME)?
        .build()?;

    assert_eq!(db.driver(), &KeyValue);
    Ok(())
}
