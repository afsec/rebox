use std::marker::PhantomData;

use rebox_types::{helpers::check_valid_entity_name, ReboxResult};

use crate::{database::DatabaseMetadata, drivers::Driver};

use super::{fields::name::DatabaseName, Database};

impl Database {
    pub fn new() -> DatabaseBuilder {
        DatabaseBuilder
    }
}

#[derive(Debug)]
pub struct DatabaseBuilder;

impl DatabaseBuilder {
    pub fn set_name<S: AsRef<str>>(self, name: S) -> ReboxResult<DatabaseBuilderS1> {
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
        use crate::drivers::key_value::KeyValueDriver;
        let mut driver = KeyValueDriver::new()
            .set_name(self.name.to_owned())?
            .build()?
            .connect()?;

        let Self { name, .. } = self;
        Ok(Database {
            name,
            driver,
            metadata: DatabaseMetadata::default(),
        })
    }
}
