use crate::{database::Database, test_helpers::ResultScenario, ReboxResult};

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
