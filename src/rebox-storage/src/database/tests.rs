use std::vec;

use crate::database::{
    row::{ColumnValue, TableColumn},
    Database,
};

use rebox_types::{test_helpers::ResultScenario, ReboxResult};

use test_case::test_case;

#[test_case(&["db-name-1","db-name1"],ResultScenario::Success ; "when name is valid")]
#[test_case(&["db-name_1","db_name1"],ResultScenario::Error  ; "when name is invalid")]
fn create_database(database_names: &[&str], result_scenario: ResultScenario) -> ReboxResult<()> {
    let res = database_names
        .iter()
        .map(|name| Database::new().set_name(name)?.build())
        .collect::<ReboxResult<Vec<Database>>>();

    let current_scenario = ResultScenario::from(&res);

    assert_eq!(current_scenario, result_scenario);

    if current_scenario == ResultScenario::Success {
        assert!(res.is_ok());
        assert_eq!(res?.len(), database_names.len());
    }

    Ok(())
}

/////
#[test_case(vec!["db-name1","db-name2"] ; "when creating one table for earch of them")]
fn digging_database(database_names: Vec<&str>) -> ReboxResult<()> {
    use crate::database::TableRow;
    use rebox_types::schema::{ColumnKind, SchemaColumn, Table};

    let request_tbl_schema = vec![
        SchemaColumn::new()
            .set_name("request-id")?
            .set_kind(ColumnKind::Text)
            .is_nullable(false)
            .build(),
        SchemaColumn::new()
            .set_name("method")?
            .set_kind(ColumnKind::Text)
            .is_nullable(false)
            .build(),
        SchemaColumn::new()
            .set_name("url")?
            .set_kind(ColumnKind::Text)
            .is_nullable(false)
            .build(),
    ];

    let response_tbl_schema = vec![
        SchemaColumn::new()
            .set_name("request-id")?
            .set_kind(ColumnKind::Text)
            .is_nullable(false)
            .build(),
        SchemaColumn::new()
            .set_name("status")?
            .set_kind(ColumnKind::Natural)
            .is_nullable(false)
            .build(),
        SchemaColumn::new()
            .set_name("url")?
            .set_kind(ColumnKind::Text)
            .is_nullable(false)
            .build(),
    ];

    let requests_tbl = Table::new()
        .set_name("requests")?
        .set_schema(request_tbl_schema)?
        .build()?;

    let responses_tbl = Table::new()
        .set_name("responses")?
        .set_schema(response_tbl_schema)?
        .build()?;

    let mut databases = database_names
        .iter()
        .map(|name| Database::new().set_name(name)?.build())
        .collect::<ReboxResult<Vec<Database>>>()?;

    // DATABASE CRUD - CREATE TABLE
    databases
        .iter_mut()
        .map(|database: &mut Database| {
            database.create_table(requests_tbl.clone())?;
            database.create_table(responses_tbl.clone())?;
            Ok(())
        })
        .collect::<ReboxResult<Vec<()>>>()?;

    // DATABASE CRUD - READ (LIST TABLES)
    databases.iter().for_each(|database| {
        dbg!(database.name(), database.list_tables());
    });

    // TABLE CRUD - CREATE ROW (INSERT)
    let c1 = TableColumn::new()
        .set_name("request-id")?
        .set_kind(ColumnKind::Text)
        .is_nullable(false)
        .set_value(ColumnValue::Text("B46D427F2".to_owned()))?
        .build()?;

    let columns = vec![c1];
    let table_row = TableRow::new(columns)?;
    databases
        .iter_mut()
        .map(|database: &mut Database| {
            database.insert_into_table("requests", table_row.clone())?;
            database.insert_into_table("responses", table_row.clone())?;

            Ok(())
        })
        .collect::<ReboxResult<Vec<()>>>()?;

    Ok(())
}