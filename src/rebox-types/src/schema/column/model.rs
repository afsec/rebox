use bincode::{Decode, Encode};
use rkv::OwnedValue as RkvOwnedValue;
use std::{fmt::Display, ops::Deref};
// const COLUMN_MAX_CAPACITY: usize = 1024 * 1024 * 50; // 50 MBytes

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Decode, Encode)]
pub struct ColumnName(String);

impl Deref for ColumnName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: AsRef<str>> From<T> for ColumnName {
    fn from(value: T) -> Self {
        ColumnName(value.as_ref().to_owned())
    }
}

impl Display for ColumnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Decode, Encode)]
pub enum ColumnKind {
    Bool,
    Integer,
    Natural,
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColumnValue {
    Bool(bool),
    Integer(i64),
    Natural(u64),
    Text(String),
}

impl From<ColumnValue> for RkvOwnedValue {
    fn from(column_value: ColumnValue) -> Self {
        match column_value {
            ColumnValue::Bool(b) => Self::Bool(b),
            ColumnValue::Integer(i) => Self::I64(i),
            ColumnValue::Natural(u) => Self::U64(u),
            ColumnValue::Text(s) => Self::Str(s),
        }
    }
}
