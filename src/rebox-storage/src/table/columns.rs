use anyhow::bail;
use bytes::{Buf, BufMut, BytesMut};

use rebox_types::ReboxResult;

use crate::drivers::DataStorage;

const COLUMN_MAX_CAPACITY: usize = 1024 * 1024 * 50; // 50 MBytes

#[derive(Debug, Default)]
pub struct Column<DS: DataStorage> {
    column_name: ColumnName,
    column_storage: DS,
}

#[derive(Debug, Default)]
pub struct ColumnName(String);

#[derive(Debug, Default)]
pub struct ColumnContent(BytesMut);

impl ColumnContent {
    pub fn store(mut self, payload: Box<dyn Buf>) -> ReboxResult<()> {
        if self.0.capacity() >= COLUMN_MAX_CAPACITY {
            bail!("Out ot space inside a column")
        }
        self.0.put(payload);
        Ok(())
    }
}
