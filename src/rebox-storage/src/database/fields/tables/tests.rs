use rebox_types::{
    schema::{ColumnKind, SchemaColumn, Table},
    test_helpers::ResultScenario,
    ReboxResult,
};

use test_case::test_case;

use crate::KeyValueDriver;

const DEFAULT_DB_NAME: &str = "rebox-123123123";

#[test_case(&["c1","c1"], ResultScenario::Error ; "when two column has same name")]
#[test_case(&["c1","c2"], ResultScenario::Success ; "when two column has different name")]
fn create_table(column_names: &[&str], result_scenario: ResultScenario) -> ReboxResult<()> {
    use crate::database::Database;
    let driver = KeyValueDriver;
    let mut db = Database::new()
        .set_name(DEFAULT_DB_NAME)?
        .set_driver(driver)
        .build()?;

    let schema = column_names
        .iter()
        .map(|name| {
            let outcome = SchemaColumn::new()
                .set_name(name)?
                .set_kind(ColumnKind::Text)
                .is_nullable(false)
                .build();
            Ok(outcome)
        })
        .collect::<ReboxResult<Vec<SchemaColumn>>>()?;

    assert_eq!(schema.len(), column_names.len());

    let res = Table::new().set_name("tbl2")?.set_schema(schema);
    let current_scenario = ResultScenario::from(&res);

    assert_eq!(current_scenario, result_scenario);

    // TODO
    // if current_scenario == ResultScenario::Success {
    //     let table = res?.build()?;

    //     let res = db.create_table(table);

    //     assert!(res.is_ok());
    // }

    Ok(())
}
