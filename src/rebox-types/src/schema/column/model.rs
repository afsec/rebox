use std::fmt::Display;

use bytes::BytesMut;

// const COLUMN_MAX_CAPACITY: usize = 1024 * 1024 * 50; // 50 MBytes

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColumnName(String);

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColumnKind {
    Bool,
    Integer,
    Natural,
    Text,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColumnData(BytesMut);

// impl ColumnData {
//     pub(crate) fn store(mut self, payload: Box<dyn Buf>) -> ReboxResult<()> {
//         if self.0.capacity() >= COLUMN_MAX_CAPACITY {
//             bail!("Out ot space inside a column")
//         }
//         self.0.put(payload);
//         Ok(())
//     }
// }
