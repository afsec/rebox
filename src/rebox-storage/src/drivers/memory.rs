use std::{collections::BTreeMap, io::Bytes};

use anyhow::bail;
use bytes::{Buf, BufMut, BytesMut};

use rebox_types::ReboxResult;

use crate::table::ColumnContent;

use super::{DataStorage, Driver};

const COLUMN_MAX_CAPACITY: usize = 1024 * 1024 * 50; // 50 MBytes

const MAX_DB_SIZE: usize = u32::MAX as usize; // 4 GBytes

impl Driver for InMemoryDriver {
    type Storage<DS> = InMemoryStorage;
   
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct InMemoryDriver;

impl DataStorage for InMemoryStorage {
    fn max_dbsize(&self) -> usize {
        MAX_DB_SIZE
    }
}

#[derive(Debug, Default)]
pub struct InMemoryStorage(BTreeMap<u32, ColumnContent>);
