mod key_value;
mod memory;

use std::str::FromStr;

use anyhow::bail;
use rebox_types::ReboxResult;

// pub use key_value::DriverRkv;
// pub use memory::DriverMemory;

#[derive(Debug)]
pub enum Driver {
    Memory(memory::DriverMemory),
    KeyValue(key_value::DriverRkv),
}

impl FromStr for Driver {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outcome = match s {
            "memory" => Self::Memory(Default::default()),
            "kv" => Self::KeyValue(Default::default()),
            _ => bail!("Invalid driver"),
        };
        Ok(outcome)
    }
}

impl Driver {
    pub fn run(self) -> ReboxResult<()> {
        match self {
            Self::KeyValue(drv) => {
                dbg!(&drv);
                drv.run()?;
            }
            Self::Memory(drv) => {
                dbg!(&drv);
                drv.run()?;
            }
        };
        Ok(())
    }
}
