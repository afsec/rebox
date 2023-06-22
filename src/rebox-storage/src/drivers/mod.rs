mod key_value;
mod memory;

use std::str::FromStr;

use anyhow::bail;
use rebox_types::ReboxResult;

pub trait Driver {}

pub use key_value::KeyValue;
pub use memory::InMemory;

// #[derive(Debug)]
// pub enum Driver {
//     Memory(memory::DriverMemory),
//     KeyValue(key_value::KeyValue),
// }

// impl FromStr for Driver {
//     type Err = anyhow::Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let outcome = match s {
//             "memory" => Self::Memory(Default::default()),
//             "kv" => Self::KeyValue(Default::default()),
//             _ => bail!("Invalid driver"),
//         };
//         Ok(outcome)
//     }
// }

// impl Driver {
//     pub fn run(self) -> ReboxResult<()> {
//         match self {
//             Self::KeyValue(drv) => {
//                 dbg!(&drv);
//                 drv.run()?;
//             }
//             Self::Memory(drv) => {
//                 dbg!(&drv);
//                 drv.run()?;
//             }
//         };
//         Ok(())
//     }
// }
