use rebox_types::{
    schema::{
        column::{model::ColumnKind, SchemaColumn},
        Table,
    },
    ReboxResult,
};

use rebox_storage::Database;
fn main() -> ReboxResult<()> {
    let db_name = "example-crud";

    let db = Database::new().set_name(db_name)?.build()?;
    create_table_departments(&db)?;
    create_table_users(&db)?;
    show_tables(&db)?;
    Ok(())
}

fn show_tables(db: &Database) -> ReboxResult<()> {
    println!("Tables");
    println!("======\n");
    db.list_tables()?
        .iter()
        .for_each(|tbl_name| println!(" - {tbl_name}"));
    println!("\n\n");

    Ok(())
}

fn create_table_departments(db: &Database) -> ReboxResult<()> {
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
    let table = Table::new()
        .set_name("departments")?
        .set_schema(schema)?
        .build()?;
    dbg!(&table);

    // dbg!(&db);

    let table_name = db.create_table(table)?;
    println!("Table [{table_name}] created.\n\n");
    Ok(())
}

fn create_table_users(db: &Database) -> ReboxResult<()> {
    let id = SchemaColumn::new()
        .set_name("id")?
        .set_kind(ColumnKind::Natural)
        .set_nullable(false)
        .build();
    let login = SchemaColumn::new()
        .set_name("name")?
        .set_kind(ColumnKind::Text)
        .set_nullable(false)
        .build();

    let full_name = SchemaColumn::new()
        .set_name("full-name")?
        .set_kind(ColumnKind::Text)
        .set_nullable(false)
        .build();

    let is_active = SchemaColumn::new()
        .set_name("is-active")?
        .set_kind(ColumnKind::Bool)
        .set_nullable(false)
        .build();

    let created_at = SchemaColumn::new()
        .set_name("created-at")?
        .set_kind(ColumnKind::Integer)
        .set_nullable(false)
        .build();


    let schema: Vec<SchemaColumn> = vec![id, login, full_name, is_active, created_at];
    dbg!(&schema);
    let table = Table::new()
        .set_name("users")?
        .set_schema(schema)?
        .build()?;
    dbg!(&table);

    // dbg!(&db);

    let table_name = db.create_table(table)?;
    println!("Table [{table_name}] created.\n\n");
    Ok(())
}
