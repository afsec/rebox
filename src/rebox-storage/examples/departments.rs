use rebox_types::{
    schema::{
        column::{model::ColumnKind, SchemaColumn},
        Table,
    },
    ReboxResult,
};

use rebox_storage::Database;
fn main() -> ReboxResult<()> {
    let db_name = "example-database";

    let db = Database::new().set_name(db_name)?.build()?;

    let c1 = SchemaColumn::new()
        .set_name("id")?
        .set_kind(ColumnKind::Natural)
        .set_nullable(false)
        .build();
    let c2 = SchemaColumn::new()
        .set_name("name")?
        .set_kind(ColumnKind::Text)
        .set_nullable(false)
        .build();

    let schema: Vec<SchemaColumn> = vec![c1, c2];
    dbg!(&schema);

    let table = Table::new().set_name("departments")?.set_schema(schema)?.build()?;
    dbg!(&table);

    // dbg!(&db);

    let table_name = db.create_table(table)?;

    dbg!(&table_name);

    let tables = db.list_tables()?;

    dbg!(&tables);
    Ok(())
}
