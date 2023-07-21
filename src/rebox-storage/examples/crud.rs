use chrono::Utc;
use rebox_storage::database::{
    row::{column::data::RowData, TableRow},
    Database,
};
use rebox_types::{
    schema::{
        column::{
            model::{ColumnKind, ColumnValue},
            SchemaColumn,
        },
        RowId, Table,
    },
    ReboxResult,
};

fn main() -> ReboxResult<()> {
    let db_name = "example_crud";
    let db = Database::new().set_name(db_name)?.build()?;
    CrudDepartments::run(&db)?;
    CrudUsers::run(&db)?;

    Ok(())
}

struct CrudDepartments;

impl CrudDepartments {
    pub fn run(db: &Database) -> ReboxResult<()> {
        let table = Self::generate_table()?;

        let table_name = db.create_table(table.clone())?;
        println!("\nTable [{table_name}] created.\n");

        let rows = Self::generate_data(&table)?;
        let _ = rows
            .into_iter()
            .map(|row| db.insert_into_table(table_name.clone(), row))
            .collect::<ReboxResult<Vec<RowId>>>()?;
        // dbg!(&table_row);

        // let rows = db.get_table_rows(&table_name, Some(&row_id))?;
        let rows = db.get_table_rows(&table_name, None)?;
        // print_rows_as_json(&rows)?;
        print_row_as_table(&rows)?;

        // let table_name = db.drop_table(table.name())?;
        // println!("Table [{table_name}] deleted.");

        Ok(())
    }
    fn generate_table() -> ReboxResult<Table> {
        let id = SchemaColumn::new()
            .set_name("oid")?
            .set_kind(ColumnKind::Text)
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
    fn generate_data(table: &Table) -> ReboxResult<Vec<TableRow>> {
        let mut rows = vec![];
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value(ColumnValue::Text(gen_new_oid())));
            let _ = btree
                .get_mut("name")
                .map(|column| column.set_value(ColumnValue::Text("IT".into())));

            row.verify()?;
            row.check_verified()?;
            rows.push(row);
        }
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value(ColumnValue::Text(gen_new_oid())));
            let _ = btree
                .get_mut("name")
                .map(|column| column.set_value(ColumnValue::Text("Accounting".into())));

            row.verify()?;
            row.check_verified()?;
            rows.push(row);
        }
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value(ColumnValue::Text(gen_new_oid())));
            let _ = btree
                .get_mut("name")
                .map(|column| column.set_value(ColumnValue::Text("Marketing".into())));

            row.verify()?;
            row.check_verified()?;
            rows.push(row);
        }

        Ok(rows)
    }
}

struct CrudUsers;
impl CrudUsers {
    pub fn run(db: &Database) -> ReboxResult<()> {
        let table = Self::generate_table()?;

        let table_name = db.create_table(table.clone())?;
        println!("\nTable [{table_name}] created.\n");

        let rows = Self::generate_data(&table)?;
        // dbg!(&table_row);
        let _ = rows
            .into_iter()
            .map(|row| db.insert_into_table(table_name.clone(), row))
            .collect::<ReboxResult<Vec<RowId>>>()?;

        // let rows = db.get_table_rows(&table_name, Some(&row_id))?;
        let rows = db.get_table_rows(&table_name, None)?;
        // print_rows_as_json(&rows)?;
        print_row_as_table(&rows)?;

        // let table_name = db.drop_table(table.name())?;
        // println!("Table [{table_name}] deleted.");
        Ok(())
    }
    fn generate_table() -> ReboxResult<Table> {
        let id = SchemaColumn::new()
            .set_name("oid")?
            .set_kind(ColumnKind::Text)
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
    fn generate_data(table: &Table) -> ReboxResult<Vec<TableRow>> {
        let mut rows = vec![];
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value(ColumnValue::Text(gen_new_oid())));
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
            rows.push(row);
        }
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value(ColumnValue::Text(gen_new_oid())));
            let _ = btree
                .get_mut("login")
                .map(|column| column.set_value(ColumnValue::Text("admin".into())));

            let _ = btree
                .get_mut("full_name")
                .map(|column| column.set_value(ColumnValue::Text("Administrator".into())));

            let _ = btree
                .get_mut("is_active")
                .map(|column| column.set_value(ColumnValue::Bool(true)));

            let _ = btree
                .get_mut("created_at")
                .map(|column| column.set_value(ColumnValue::Integer(Utc::now().timestamp())));
            row.verify()?;
            row.check_verified()?;
            rows.push(row);
        }
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value(ColumnValue::Text(gen_new_oid())));
            let _ = btree
                .get_mut("login")
                .map(|column| column.set_value(ColumnValue::Text("staff".into())));

            let _ = btree
                .get_mut("full_name")
                .map(|column| column.set_value(ColumnValue::Text("Staff".into())));

            let _ = btree
                .get_mut("is_active")
                .map(|column| column.set_value(ColumnValue::Bool(true)));

            let _ = btree
                .get_mut("created_at")
                .map(|column| column.set_value(ColumnValue::Integer(Utc::now().timestamp())));
            row.verify()?;
            row.check_verified()?;
            rows.push(row);
        }

        Ok(rows)
    }
}

fn print_rows_as_json(rows: &Vec<RowData>) -> ReboxResult<()> {
    let maybe_first = rows.first();
    dbg!(maybe_first);
    let json = serde_json::to_string_pretty(&rows)?;
    println!("{}", json);
    Ok(())
}

fn print_row_as_table(rows: &Vec<RowData>) -> ReboxResult<()> {
    use tabled::{builder::Builder, settings::Style};
    let mut headers: Vec<String> = vec!["row_id".into()];
    let mut col_names: Vec<String> = rows
        .first()
        .map(|item| {
            item.col_names()
                .iter()
                .map(|col_name| col_name.to_string())
                .collect()
        })
        .unwrap();
    let mut builder = Builder::default();
    headers.append(&mut col_names);
    builder.set_header(headers);
    rows.iter().for_each(|item| {
        let mut output = vec![item.row_id().to_string()];
        let mut columns: Vec<String> = item
            .col_values()
            .iter()
            .map(|col_value| col_value.to_string())
            .collect();
        output.append(&mut columns);
        builder.push_record(output);
    });

    let mut table = builder.build();
    table.with(Style::rounded());

    println!("{}", table);
    Ok(())
}

fn gen_new_oid() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;
    use ulid::Ulid;
    let t = Ulid::new();
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    let hash = s.finish().to_be_bytes();

    hex::encode(hash)
}
