// use std::vec;

// use crate::{
//     database::{
//         row::{
//             column::{ColumnValue, TableColumn},
//             TableRow,
//         },
//         Database,
//     },
//     drivers::key_value::KeyValueDriver,
// };

// use rebox_types::{
//     schema::{
//         column::{model::ColumnKind, SchemaColumn},
//         Table,
//     },
//     ReboxResult,
// };

// use test_case::test_case;

// #[test_case(vec!["db-name1","db-name2"] ; "when creating one table for earch of them")]
// fn digging_database(database_names: Vec<&str>) -> ReboxResult<()> {
//     let driver = KeyValueDriver;
//     let request_tbl_schema = vec![
//         SchemaColumn::new()
//             .set_name("request-id")?
//             .set_kind(ColumnKind::Text)
//             .set_nullable(false)
//             .build(),
//         SchemaColumn::new()
//             .set_name("method")?
//             .set_kind(ColumnKind::Text)
//             .set_nullable(false)
//             .build(),
//         SchemaColumn::new()
//             .set_name("url")?
//             .set_kind(ColumnKind::Text)
//             .set_nullable(false)
//             .build(),
//     ];

//     let response_tbl_schema = vec![
//         SchemaColumn::new()
//             .set_name("request-id")?
//             .set_kind(ColumnKind::Text)
//             .set_nullable(false)
//             .build(),
//         SchemaColumn::new()
//             .set_name("status")?
//             .set_kind(ColumnKind::Natural)
//             .set_nullable(false)
//             .build(),
//         SchemaColumn::new()
//             .set_name("url")?
//             .set_kind(ColumnKind::Text)
//             .set_nullable(false)
//             .build(),
//     ];

//     let requests_tbl = Table::new()
//         .set_name("requests")?
//         .set_schema(request_tbl_schema)?
//         .build()?;

//     let responses_tbl = Table::new()
//         .set_name("responses")?
//         .set_schema(response_tbl_schema)?
//         .build()?;

//     let mut databases = database_names
//         .iter()
//         .map(|name| {
//             Database::new()
//                 .set_name(name)?
//                 .set_driver(driver.clone())
//                 .build()
//         })
//         .collect::<ReboxResult<Vec<Database<KeyValueDriver>>>>()?;

//     // TODO
//     // DATABASE CRUD - CREATE TABLE
//     // databases
//     //     .iter_mut()
//     //     .map(|database: &mut Database<KeyValueDriver>| {
//     //         database.create_table(requests_tbl.clone())?;
//     //         database.create_table(responses_tbl.clone())?;
//     //         Ok(())
//     //     })
//     //     .collect::<ReboxResult<Vec<()>>>()?;

//     // TODO
//     // DATABASE CRUD - READ (LIST TABLES)
//     // databases.iter().for_each(|database| {
//     //     dbg!(database.name(), database.list_tables());
//     // });

//     // TABLE CRUD - CREATE ROW (INSERT)
//     let c1 = TableColumn::new()
//         .set_name("request-id")?
//         .set_kind(ColumnKind::Text)
//         .set_nullable(false)
//         .set_value(ColumnValue::Text("B46D427F2".to_owned()))?
//         .build()?;

//     let columns = vec![c1];
//     let table_row = TableRow::new(columns)?;
//     // TODO
//     // databases
//     //     .iter_mut()
//     //     .map(|database: &mut Database<KeyValueDriver>| {
//     //         database.insert_into_table("requests", table_row.clone())?;
//     //         database.insert_into_table("responses", table_row.clone())?;

//     //         Ok(())
//     //     })
//     //     .collect::<ReboxResult<Vec<()>>>()?;

//     Ok(())
// }
