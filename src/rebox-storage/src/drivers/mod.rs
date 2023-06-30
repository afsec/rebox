pub(crate) mod key_value;
pub(crate) mod memory;

pub(crate) trait Driver {}

// pub trait Driver {
//     type Storage<DS>: DataStorage;
// }

// pub trait DataStorage {
//     const MAX_SIZE_DB: usize;
//     fn max_dbsize(&self) -> usize;
// }

// pub use memory::{InMemoryDriver, InMemoryStorage};
