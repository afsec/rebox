mod key_value;
mod memory;

use std::str::FromStr;

use anyhow::bail;
use rebox_types::ReboxResult;

pub trait Driver {}

pub trait DataStorage {}

pub use key_value::{KeyValue, KeyValueStorage};
pub use memory::{InMemoryDriver, InMemoryStorage};
