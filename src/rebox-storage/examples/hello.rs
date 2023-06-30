use rebox_types::{
    schema::{
        column::{model::ColumnKind, SchemaColumn},
        Table,
    },
    ReboxResult,
};

use rebox_storage::{
    database::{
        row::{
            column::{ColumnValue, TableColumn},
            TableRow,
        },
        Database,
    },
    drivers::key_value::KeyValueDriver,
};
fn main() -> ReboxResult<()> {
    let driver = KeyValueDriver;
    let mut db = Database::new()
        .set_name("example-database")?
        .set_driver(driver)
        .build()?;

    let c1 = SchemaColumn::new()
        .set_name("c1")?
        .set_kind(ColumnKind::Text)
        .set_nullable(false)
        .build();
    let c2 = SchemaColumn::new()
        .set_name("c2")?
        .set_kind(ColumnKind::Text)
        .set_nullable(false)
        .build();

    let schema: Vec<SchemaColumn> = vec![c1, c2];
    dbg!(&schema);
    let table = Table::new().set_name("tbl2")?.set_schema(schema)?.build()?;
    dbg!(&table);
    let table_name = db.create_table(table)?;

    dbg!(&table_name);

    Ok(())
}
