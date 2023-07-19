use std::{fmt::Display, ops::Deref};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableName(String);

impl Deref for TableName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TableName {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self(name.as_ref().to_owned())
    }
}

impl AsRef<str> for TableName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
