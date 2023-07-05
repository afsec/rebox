use rebox_types::{
    schema::{
        column::{model::ColumnKind, SchemaColumn},
        Table,
    },
    ReboxResult,
};

use rebox_storage::database::{fields::name::DatabaseName, Database};
fn main() -> ReboxResult<()> {
    use rebox_storage::drivers::key_value::KeyValueDriver;
    let db_name = "example-database";
    // let mut db = Database::new().set_name(db_name)?.build()?;

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
    // dbg!(&db);
    let schema: Vec<SchemaColumn> = vec![c1, c2];
    dbg!(&schema);
    let table = Table::new().set_name("tbl2")?.set_schema(schema)?.build()?;
    dbg!(&table);

    // let table_name = db.create_table(table)?;

    // dbg!(&table_name);

    let kv_driver = KeyValueDriver::new()
        .set_name(DatabaseName::new(db_name)?)?
        .create_mode(true)
        .build()?
        .connect()?;

    dbg!(&kv_driver);

    // kv_driver.create_table(table)?;
    Ok(())
}
