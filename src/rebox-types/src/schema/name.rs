use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableName(String);

impl From<TableName> for String {
    fn from(value: TableName) -> Self {
        value.0
    }
}

impl TableName {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self(name.as_ref().to_owned())
    }
}

impl Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
