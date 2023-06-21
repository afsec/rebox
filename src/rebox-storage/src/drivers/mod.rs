pub(super) mod key_value;
pub(super) mod memory;

pub enum Driver {
    Memory(memory::DriverMemory),
    KeyValue(key_value::DriverRkv),
}
