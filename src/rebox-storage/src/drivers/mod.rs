mod key_value;
mod memory;

use std::str::FromStr;

use rebox_types::ReboxResult;

// pub use key_value::DriverRkv;
// pub use memory::DriverMemory;

#[derive(Debug, Default)]
pub enum Driver {
    Memory(memory::DriverMemory),
    KeyValue(key_value::DriverRkv),
    #[default]
    Nil,
}

impl FromStr for Driver {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outcome = match s {
            "memory" => Self::Memory(Default::default()),
            "kv" => Self::KeyValue(Default::default()),
            _ => Self::default(),
        };
        Ok(outcome)
    }
}

impl Driver {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn run(self) -> ReboxResult<()> {
        use anyhow::bail;
        match self {
            Self::KeyValue(drv) => {
                dbg!(drv);
                todo!()
            }
            Self::Memory(drv) => {
                dbg!(drv);
                todo!()
            }
            Self::Nil => bail!("No driver selected"),
        };
        Ok(())
    }
}
