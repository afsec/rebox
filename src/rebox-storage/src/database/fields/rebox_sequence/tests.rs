use rebox_types::{
    schema::{ColumnKind, SchemaColumn, Table},
    ReboxResult,
};

use crate::{
    database::fields::{ReboxMaster, ReboxSchema},
    KeyValueStorage,
};

use super::ReboxSequence;

#[test]
fn open_table_rebox_sequence() -> ReboxResult<()> {
    // TODO: Implement new/open session
    let rebox_sequence = ReboxSequence::default();
    KeyValueStorage::open_metadata(rebox_sequence.table_name())?;
    Ok(())
}

#[test]
fn open_table_rebox_master() -> ReboxResult<()> {
    // TODO: Implement new/open session
    let rebox_master = ReboxMaster::default();
    KeyValueStorage::open_metadata(rebox_master.table_name())?;
    Ok(())
}

#[test]
fn open_table_rebox_schema() -> ReboxResult<()> {
    // TODO: Implement new/open session
    let rebox_schema = ReboxSchema::default();
    KeyValueStorage::open_metadata(rebox_schema.table_name())?;
    Ok(())
}
#[test]
fn open_table_requests() -> ReboxResult<()> {
    let c1 = SchemaColumn::new()
        .set_name("c1")?
        .set_kind(ColumnKind::Text)
        .is_nullable(false)
        .build();
    let columns = vec![c1];

    // TODO: Implement new/open session
    let table = Table::new()
        .set_name("requests")?
        .set_schema(columns)?
        .build()?;

    // TODO: Create Table Schema
    // TODO: Open Table to Insert into
    // TODO: Validate Table Schema against Database
    KeyValueStorage::open_table(table.name())?;
    Ok(())
}
#[test]
fn open_table_responses() -> ReboxResult<()> {
    let c1 = SchemaColumn::new()
        .set_name("c1")?
        .set_kind(ColumnKind::Text)
        .is_nullable(false)
        .build();
    let columns = vec![c1];

    // TODO: Implement new/open session
    let table = Table::new()
        .set_name("responses")?
        .set_schema(columns)?
        .build()?;

    // TODO: Create Table Schema
    // TODO: Open Table to Insert into
    // TODO: Validate Table Schema against Database
    KeyValueStorage::open_table(table.name())?;
    Ok(())
}
