use std::{collections::BTreeMap, io::Bytes};

use anyhow::bail;
use bytes::{Buf, BufMut, BytesMut};

use rebox_types::ReboxResult;

use crate::table::ColumnContent;

use super::{DataStorage, Driver};

const COLUMN_MAX_CAPACITY: usize = 1024 * 1024 * 1024 * 50; // 50 MBytes

#[derive(Debug)]
struct MaxSize(usize);
impl Default for MaxSize {
    /// 4 GBytes
    fn default() -> Self {
        // Supporting from 32 bit architectures
        Self(u32::MAX as usize)
    }
}

impl Driver for InMemoryDriver {}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct InMemoryDriver;

impl DataStorage for InMemoryStorage {}

#[derive(Debug, Default)]
pub struct InMemoryStorage(BTreeMap<u32, ColumnContent>);
