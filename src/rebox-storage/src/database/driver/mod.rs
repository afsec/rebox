pub(super) mod key_value;

pub(super) trait Driver {}

pub(super) trait DataStorage: Driver {
    const MAX_SIZE_DB: usize;
    fn max_dbsize(&self) -> usize;
}
