use crate::ReboxResult;

#[derive(Debug, PartialEq, Eq)]
pub enum ResultScenario {
    Success,
    Error,
}

impl<T> From<&ReboxResult<T>> for ResultScenario {
    fn from(value: &ReboxResult<T>) -> Self {
        match value {
            Ok(_) => Self::Success,
            Err(_) => Self::Error,
        }
    }
}
