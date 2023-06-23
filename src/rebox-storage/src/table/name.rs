use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableName(String);

impl TableName {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self(name.as_ref().to_string())
    }
}

impl From<String> for TableName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
