#[derive(Debug, Default, Clone)]
pub(crate) struct DatabaseName(String);
impl AsRef<str> for DatabaseName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl DatabaseName {
    pub(crate) fn new<T: AsRef<str>>(name: T) -> Self {
        Self(name.as_ref().to_owned())
    }
}
