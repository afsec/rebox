#[derive(Debug, Default)]
pub struct TableName(String);

impl TableName {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self(name.as_ref().to_string())
    }
}
