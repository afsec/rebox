use anyhow::bail;
use bytes::{Buf, BufMut, BytesMut};

use rebox_types::ReboxResult;

const COLUMN_MAX_CAPACITY: usize = 1024 * 1024 * 50; // 50 MBytes

#[derive(Debug)]
pub struct ColumnDesc {
    column_name: ColumnName,
    column_type: ColumnType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Column {
    column_name: ColumnName,
    column_type: ColumnType,
    columen_value: ColumnValue,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ColumnName(String);

#[derive(Debug, PartialEq, Eq)]
pub enum ColumnType {
    Null,
    Bool(bool),
    Integer(u32),
    Float((u16, u16)),
    Text(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ColumnValue(BytesMut);

impl ColumnValue {
    pub fn store(mut self, payload: Box<dyn Buf>) -> ReboxResult<()> {
        if self.0.capacity() >= COLUMN_MAX_CAPACITY {
            bail!("Out ot space inside a column")
        }
        self.0.put(payload);
        Ok(())
    }
}
