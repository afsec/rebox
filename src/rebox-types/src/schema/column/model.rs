use anyhow::bail;
use bincode::{Decode, Encode};
use rkv::OwnedValue as RkvOwnedValue;
use serde::Serialize;
use std::{fmt::Display, ops::Deref};

// const COLUMN_MAX_CAPACITY: usize = 1024 * 1024 * 50; // 50 MBytes

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Decode, Encode)]
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

impl PartialEq<ColumnValue> for ColumnKind {
    fn eq(&self, other: &ColumnValue) -> bool {
        other == self
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum ColumnValue {
    Bool(bool),
    Integer(i64),
    Natural(u64),
    Text(String),
}

impl From<bool> for ColumnValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl From<i64> for ColumnValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl From<u64> for ColumnValue {
    fn from(value: u64) -> Self {
        Self::Natural(value)
    }
}
impl From<String> for ColumnValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl Display for ColumnValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnValue::Bool(v) => write!(f, "{v}"),
            ColumnValue::Integer(v) => write!(f, "{v}"),
            ColumnValue::Natural(v) => write!(f, "{v}"),
            ColumnValue::Text(v) => write!(f, "{v}"),
        }
    }
}
impl PartialEq<ColumnKind> for ColumnValue {
    fn eq(&self, other: &ColumnKind) -> bool {
        match (self, other) {
            (ColumnValue::Bool(_), ColumnKind::Bool) => true,
            (ColumnValue::Integer(_), ColumnKind::Integer) => true,
            (ColumnValue::Natural(_), ColumnKind::Natural) => true,
            (ColumnValue::Text(_), ColumnKind::Text) => true,
            _ => false,
        }
    }
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

impl TryFrom<RkvOwnedValue> for ColumnValue {
    type Error = anyhow::Error;

    fn try_from(rkv_owned_value: RkvOwnedValue) -> Result<Self, Self::Error> {
        let converted = match rkv_owned_value {
            RkvOwnedValue::Bool(b) => ColumnValue::Bool(b),
            RkvOwnedValue::I64(i) => ColumnValue::Integer(i),
            RkvOwnedValue::U64(u) => ColumnValue::Natural(u),
            RkvOwnedValue::Str(s) => ColumnValue::Text(s),
            _ => bail!("OwnedValue not implemented for ColumnValue"),
        };
        Ok(converted)
    }
}
