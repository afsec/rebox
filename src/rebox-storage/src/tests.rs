use rebox_types::ReboxResult;

use crate::drivers::{InMemory, KeyValue};

#[test]
fn new_in_memory_database() -> ReboxResult<()> {
    use crate::database::Database;
    let db = Database::new()
        .set_driver(InMemory)?
        .set_session_name("remora-123123123")?
        .connect()?;

    // assert_eq!(result, 4);
    Ok(())
}

#[test]
fn new_kv_database() -> ReboxResult<()> {
    use crate::database::Database;
    let db = Database::new()
        .set_driver(KeyValue)?
        .set_session_name("remora-123123123")?
        .connect()?;

    // assert_eq!(result, 4);
    Ok(())
}
