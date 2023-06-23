#[derive(Debug, Default)]
pub struct TableFileName(String);
impl TableFileName {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self(name.as_ref().to_string())
    }
}

impl From<String> for TableFileName {
    fn from(value: String) -> Self {
        Self(value)
    }
}
