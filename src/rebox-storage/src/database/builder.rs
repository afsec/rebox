use rebox_types::{helpers::check_valid_entity_name, ReboxResult};

use crate::database::DatabaseMetadata;

use super::{name::DatabaseName, Database};

impl Database {
    pub fn new() -> DatabaseBuilder {
        DatabaseBuilder
    }
}

#[derive(Debug)]
pub struct DatabaseBuilder;

impl DatabaseBuilder {
    pub fn name<S: AsRef<str>>(self, name: S) -> ReboxResult<DatabaseBuilderS1> {
        check_valid_entity_name(&name)?;

        Ok(DatabaseBuilderS1 {
            name: DatabaseName::new(name)?,
        })
    }
}

#[derive(Debug, Default)]
pub struct DatabaseBuilderS1 {
    name: DatabaseName,
}
impl DatabaseBuilderS1 {
    pub fn build(self) -> ReboxResult<Database> {
        use super::driver::key_value::KeyValueDriver;
        let driver = KeyValueDriver::new()
            .set_name(self.name.to_owned())?
            .create_mode(true)
            .build()?
            .connect()?;

        let Self { name, .. } = self;
        let db = Database {
            name,
            driver,
            metadata: DatabaseMetadata::default(),
        };

        db.bootstrap_metadata()?;

        Ok(db)
    }
}
