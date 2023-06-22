use rebox_types::ReboxResult;

use crate::drivers::{InMemoryDriver, InMemoryStorage, KeyValue, KeyValueStorage};

#[test]
fn new_in_memory() -> ReboxResult<()> {
    use crate::database::Database;
    let mut db = Database::new()
        .set_driver(InMemoryDriver, InMemoryStorage::default())?
        .set_session_name("remora-123123123")?
        .build()?;
    db.connect()?;

    assert_eq!(db.driver(), &InMemoryDriver);
    Ok(())
}

#[test]
fn new_kv() -> ReboxResult<()> {
    use crate::database::Database;
    let mut db = Database::new()
        .set_driver(KeyValue, KeyValueStorage::default())?
        .set_session_name("remora-123123123")?
        .build()?;

    db.connect()?;

    assert_eq!(db.driver(), &KeyValue);
    Ok(())
}
