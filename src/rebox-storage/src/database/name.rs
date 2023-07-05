use std::ops::Deref;

use rebox_types::{helpers::check_valid_entity_name, ReboxResult};

#[derive(Debug, Default, Clone)]
pub(crate) struct DatabaseName(String);

impl Deref for DatabaseName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for DatabaseName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl DatabaseName {
    pub(crate) fn new<T: AsRef<str>>(name: T) -> ReboxResult<Self> {
        check_valid_entity_name(&name)?;
        Ok(Self(name.as_ref().to_owned()))
    }
}
