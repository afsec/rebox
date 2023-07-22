use rebox_storage::database::Database;
use rebox_types::{
    schema::{
        column::{model::ColumnKind, SchemaColumn},
        Table,
    },
    ReboxResult,
};

fn main() -> ReboxResult<()> {
    let db_name = "example_hello";

    let db = Database::new().name(db_name)?.build()?;

    let c1 = SchemaColumn::new()
        .name("c1")?
        .kind(ColumnKind::Text)
        .not_null(false)
        .unique(false)
        .primary_key(false)
        .auto_increment(false)
        .build();
    let c2 = SchemaColumn::new()
        .name("c2")?
        .kind(ColumnKind::Text)
        .not_null(false)
        .unique(false)
        .primary_key(false)
        .auto_increment(false)
        .build();

    let schema: Vec<SchemaColumn> = vec![c1, c2];
    dbg!(&schema);

    let table = Table::new().name("tbl2")?.schema(schema)?.build()?;
    dbg!(&table);

    // dbg!(&db);

    let table_name = db.create_table(table)?;

    dbg!(&table_name);

    let tables = db.list_tables()?;

    dbg!(&tables);
    Ok(())
}
