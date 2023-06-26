use crate::{helpers::check_valid_entity_name, ReboxResult};

use super::{Database, DatabaseName, ReboxSequence};

pub struct DatabaseBuilder;

impl DatabaseBuilder {
    pub fn set_name<S: AsRef<str>>(self, name: S) -> ReboxResult<DatabaseBuilderS1> {
        check_valid_entity_name(&name)?;

        Ok(DatabaseBuilderS1 {
            name: DatabaseName::new(name),
        })
    }
}

#[derive(Debug, Default)]
pub struct DatabaseBuilderS1 {
    name: DatabaseName,
}
impl DatabaseBuilderS1 {
    pub fn build(self) -> ReboxResult<Database> {
        let Self { name } = self;
        // TODO
        Ok(Database {
            name,
            rebox_sequence: ReboxSequence::default(),
            tables: Default::default(),
        })
    }
}
