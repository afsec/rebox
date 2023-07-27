use chrono::Utc;
use rebox_storage::database::{
    row::{column::data::RowData, TableRow},
    Database,
};
use rebox_types::{
    query::{columns_filter::ColumnsFilter, values_to_match::ValuesToMatch},
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
    let db = Database::new().name(db_name)?.build()?;
    CrudDepartments::run(&db)?;
    CrudUsers::run(&db)?;

    Ok(())
}

struct CrudDepartments;

impl CrudDepartments {
    pub fn run(db: &Database) -> ReboxResult<()> {
        let table = Self::generate_table()?;

        let table_name = db.create_table(table.clone())?;
        println!("\nTable [{table_name}] created.");

        let rows = Self::generate_data(&table)?;
        let _ = rows
            .into_iter()
            .map(|row| db.insert_into_table(table_name.clone(), row))
            .collect::<ReboxResult<Vec<RowId>>>()?;
        // dbg!(&table_row);

        // let rows = db.get_table_rows(&table_name, Some(&row_id))?;
        // * All columns
        {
            let columns_filter = ColumnsFilter::new()
                // .column("oid")?
                .build();
            let values_to_match = ValuesToMatch::default();
            let rows = db.get_table_rows(&table_name, &columns_filter, &values_to_match)?;
            // print_rows_as_json(&rows)?;
            println!("SELECT {columns_filter} FROM [{table_name}] {values_to_match}");
            print_row_as_table(&rows)?;
        }
        // * Filtered by column `name`
        {
            let columns_filter = ColumnsFilter::new().column("name")?.build();
            let values_to_match = ValuesToMatch::default();
            let rows = db.get_table_rows(&table_name, &columns_filter, &values_to_match)?;
            // print_rows_as_json(&rows)?;
            println!("SELECT {columns_filter} FROM [{table_name}] {values_to_match}");
            print_row_as_table(&rows)?;
        }

        // let table_name = db.drop_table(table.name())?;
        // println!("Table [{table_name}] deleted.");

        Ok(())
    }
    fn generate_table() -> ReboxResult<Table> {
        let id = SchemaColumn::new()
            .name("oid")?
            .kind(ColumnKind::Text)
            .not_null(false)
            .unique(false)
            .primary_key(false)
            .auto_increment(false)
            .build();
        let name = SchemaColumn::new()
            .name("name")?
            .kind(ColumnKind::Text)
            .not_null(false)
            .unique(false)
            .primary_key(false)
            .auto_increment(false)
            .build();
        let columns = vec![id, name];
        let table = Table::new().name("departments")?.schema(columns)?.build()?;

        Ok(table)
    }
    fn generate_data(table: &Table) -> ReboxResult<Vec<TableRow>> {
        let mut rows = vec![];
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value("bfbc43216c956571".to_string()));
            let _ = btree
                .get_mut("name")
                .map(|column| column.set_value("IT".to_string()));

            row.verify()?;
            row.check_verified()?;
            rows.push(row);
        }
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value("d608b2b2d42ed554".to_string()));
            let _ = btree
                .get_mut("name")
                .map(|column| column.set_value("Accounting".to_string()));

            row.verify()?;
            row.check_verified()?;
            rows.push(row);
        }
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value("41ba2b37ddca8117".to_string()));
            let _ = btree
                .get_mut("name")
                .map(|column| column.set_value("Marketing".to_string()));

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
        println!("\nTable [{table_name}] created.");

        let rows = Self::generate_data(&table)?;
        // dbg!(&table_row);
        let _ = rows
            .into_iter()
            .map(|row| db.insert_into_table(table_name.clone(), row))
            .collect::<ReboxResult<Vec<RowId>>>()?;

        ///////////////////
        // * All columns
        {
            let columns_filter = ColumnsFilter::new()
                // .column("oid")?
                .build();
            let values_to_match = ValuesToMatch::default();
            let rows = db.get_table_rows(&table_name, &columns_filter, &values_to_match)?;
            // print_rows_as_json(&rows)?;
            println!("SELECT {columns_filter} FROM [{table_name}] {values_to_match}");
            print_row_as_table(&rows)?;
        }
        // * Filtered by column `name`
        {
            let columns_filter = ColumnsFilter::new()
                .column("department_id")?
                .column("full_name")?
                .column("is_active")?
                .build();
            let values_to_match = ValuesToMatch::new()
                .set_where("login".try_into()?)
                .is()
                .value(ColumnValue::Text("root".into()))
                .build()?;
            let rows = db.get_table_rows(&table_name, &columns_filter, &values_to_match)?;
            // print_rows_as_json(&rows)?;
            println!("SELECT {columns_filter} FROM [{table_name}] {values_to_match}");

            print_row_as_table(&rows)?;
        }

        ///////////////////

        // let table_name = db.drop_table(table.name())?;
        // println!("Table [{table_name}] deleted.");
        Ok(())
    }
    fn generate_table() -> ReboxResult<Table> {
        let id = SchemaColumn::new()
            .name("oid")?
            .kind(ColumnKind::Text)
            .not_null(false)
            .unique(false)
            .primary_key(false)
            .auto_increment(false)
            .build();
        let login = SchemaColumn::new()
            .name("login")?
            .kind(ColumnKind::Text)
            .not_null(false)
            .unique(false)
            .primary_key(false)
            .auto_increment(false)
            .build();

        let full_name = SchemaColumn::new()
            .name("full_name")?
            .kind(ColumnKind::Text)
            .not_null(false)
            .unique(false)
            .primary_key(false)
            .auto_increment(false)
            .build();

        let department_id = SchemaColumn::new()
            .name("department_id")?
            .kind(ColumnKind::Text)
            .not_null(false)
            .unique(false)
            .primary_key(false)
            .auto_increment(false)
            .build();

        let is_active = SchemaColumn::new()
            .name("is_active")?
            .kind(ColumnKind::Bool)
            .not_null(false)
            .unique(false)
            .primary_key(false)
            .auto_increment(false)
            .build();

        let created_at = SchemaColumn::new()
            .name("created_at")?
            .kind(ColumnKind::Integer)
            .not_null(false)
            .unique(false)
            .primary_key(false)
            .auto_increment(false)
            .build();

        let columns = vec![id, login, full_name, department_id, is_active, created_at];

        let table = Table::new().name("users")?.schema(columns)?.build()?;

        Ok(table)
    }
    fn generate_data(table: &Table) -> ReboxResult<Vec<TableRow>> {
        let mut rows = vec![];
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value(gen_new_oid().to_string()));
            let _ = btree
                .get_mut("login")
                .map(|column| column.set_value("root".to_string()));

            let _ = btree
                .get_mut("full_name")
                .map(|column| column.set_value("Charlie Root".to_string()));
            let _ = btree
                .get_mut("department_id")
                .map(|column| column.set_value("bfbc43216c956571".to_string()));

            let _ = btree
                .get_mut("is_active")
                .map(|column| column.set_value(true));

            let _ = btree
                .get_mut("created_at")
                .map(|column| column.set_value(Utc::now().timestamp()));
            row.verify()?;
            row.check_verified()?;
            rows.push(row);
        }
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value(gen_new_oid().to_string()));
            let _ = btree
                .get_mut("login")
                .map(|column| column.set_value("bob".to_string()));

            let _ = btree
                .get_mut("full_name")
                .map(|column| column.set_value("Bob Cooper".to_string()));

            let _ = btree
                .get_mut("department_id")
                .map(|column| column.set_value("d608b2b2d42ed554".to_string()));

            let _ = btree
                .get_mut("is_active")
                .map(|column| column.set_value(true));

            let _ = btree
                .get_mut("created_at")
                .map(|column| column.set_value(Utc::now().timestamp()));
            row.verify()?;
            row.check_verified()?;
            rows.push(row);
        }
        {
            let mut row = TableRow::from(table);
            let btree = row.get_mut();
            let _ = btree
                .get_mut("oid")
                .map(|column| column.set_value(gen_new_oid().to_string()));
            let _ = btree
                .get_mut("login")
                .map(|column| column.set_value("alice".to_string()));

            let _ = btree
                .get_mut("full_name")
                .map(|column| column.set_value("Alice Cooper".to_string()));
            let _ = btree
                .get_mut("department_id")
                .map(|column| column.set_value("41ba2b37ddca8117".to_string()));

            let _ = btree
                .get_mut("is_active")
                .map(|column| column.set_value(true));

            let _ = btree
                .get_mut("created_at")
                .map(|column| column.set_value(Utc::now().timestamp()));
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
