use rebox_types::ReboxResult;

pub mod key_value;

pub trait Driver {}

pub trait DataStorage: Driver {
    const MAX_SIZE_DB: usize;
    fn max_dbsize(&self) -> usize;
}
