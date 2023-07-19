use chrono::Utc;
use rebox_types::{
    schema::{
        column::{
            model::{ColumnKind, ColumnValue},
            SchemaColumn,
        },
        Table,
    },
    ReboxResult,
};

use rebox_storage::database::{row::TableRow, Database};
fn main() -> ReboxResult<()> {
    let db_name = "example_crud";

    let db = Database::new().set_name(db_name)?.build()?;
    CrudDepartments::run(&db)?;
    CrudUsers::run(&db)?;

    show_tables(&db)?;
    Ok(())
}

fn show_tables(db: &Database) -> ReboxResult<()> {
    println!("\n");
    println!("Tables");
    println!("======\n");
    db.list_tables()?
        .iter()
        .for_each(|tbl_name| println!(" - {tbl_name}"));
    println!("\n");

    Ok(())
}

struct CrudDepartments;

impl CrudDepartments {
    pub fn run(db: &Database) -> ReboxResult<()> {
        let table = Self::generate_table()?;

        let table_name = db.create_table(table.clone())?;
        println!("Table [{table_name}] created.");

        show_tables(&db)?;

        let table_row = Self::generate_data(&table)?;
        dbg!(&table_row);
        db.insert_into_table(table_name, table_row)?;

        // let table_name = db.drop_table(table.name())?;
        // println!("Table [{table_name}] deleted.");

        Ok(())
    }
    fn generate_table() -> ReboxResult<Table> {
        let id = SchemaColumn::new()
            .set_name("id")?
            .set_kind(ColumnKind::Natural)
            .set_nullable(false)
            .build();
        let name = SchemaColumn::new()
            .set_name("name")?
            .set_kind(ColumnKind::Text)
            .set_nullable(false)
            .build();
        let columns = vec![id, name];
        let table = Table::new()
            .set_name("departments")?
            .set_schema(columns)?
            .build()?;

        Ok(table)
    }
    fn generate_data(table: &Table) -> ReboxResult<TableRow> {
        let mut row = TableRow::from(table);
        let btree = row.get_mut();
        let _ = btree
            .get_mut("id")
            .map(|column| column.set_value(ColumnValue::Natural(1)));
        let _ = btree
            .get_mut("name")
            .map(|column| column.set_value(ColumnValue::Text("Marketing".into())));

        row.verify()?;
        row.check_verified()?;
        Ok(row)
    }
}

struct CrudUsers;
impl CrudUsers {
    pub fn run(db: &Database) -> ReboxResult<()> {
        let table = Self::generate_table()?;

        let table_name = db.create_table(table.clone())?;
        println!("Table [{table_name}] created.");

        show_tables(&db)?;

        let table_row = Self::generate_data(&table)?;
        dbg!(&table_row);

        // let table_name = db.drop_table(table.name())?;
        // println!("Table [{table_name}] deleted.");
        Ok(())
    }
    fn generate_table() -> ReboxResult<Table> {
        let id = SchemaColumn::new()
            .set_name("id")?
            .set_kind(ColumnKind::Natural)
            .set_nullable(false)
            .build();
        let login = SchemaColumn::new()
            .set_name("login")?
            .set_kind(ColumnKind::Text)
            .set_nullable(false)
            .build();

        let full_name = SchemaColumn::new()
            .set_name("full_name")?
            .set_kind(ColumnKind::Text)
            .set_nullable(false)
            .build();

        let is_active = SchemaColumn::new()
            .set_name("is_active")?
            .set_kind(ColumnKind::Bool)
            .set_nullable(false)
            .build();

        let created_at = SchemaColumn::new()
            .set_name("created_at")?
            .set_kind(ColumnKind::Integer)
            .set_nullable(false)
            .build();

        let columns = vec![id, login, full_name, is_active, created_at];

        let table = Table::new()
            .set_name("users")?
            .set_schema(columns)?
            .build()?;

        Ok(table)
    }
    fn generate_data(table: &Table) -> ReboxResult<TableRow> {
        let mut row = TableRow::from(table);
        let btree = row.get_mut();
        let _ = btree
            .get_mut("id")
            .map(|column| column.set_value(ColumnValue::Natural(1)));
        let _ = btree
            .get_mut("login")
            .map(|column| column.set_value(ColumnValue::Text("root".into())));

        let _ = btree
            .get_mut("full_name")
            .map(|column| column.set_value(ColumnValue::Text("Charlie Root".into())));

        let _ = btree
            .get_mut("is_active")
            .map(|column| column.set_value(ColumnValue::Bool(true)));

        let _ = btree
            .get_mut("created_at")
            .map(|column| column.set_value(ColumnValue::Integer(Utc::now().timestamp())));
        row.verify()?;
        row.check_verified()?;
        Ok(row)
    }
}
