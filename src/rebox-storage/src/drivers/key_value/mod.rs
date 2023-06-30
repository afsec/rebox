mod builder;
mod moz_rkv;

use std::path::PathBuf;

use anyhow::format_err;
use rebox_types::{schema::Table, ReboxResult};

use crate::drivers::Driver;

use self::builder::KeyValueStorageBuilder;

impl Driver for KeyValueDriver {}

// impl Driver for KeyValueDriver {
//     type Storage<DS> = KeyValueStorage;
// }

// impl DataStorage for KeyValueStorage {
//     const MAX_SIZE_DB: usize = 1024 * 1024 * 1024 * 20; // 20 GBytes
//     fn max_dbsize(&self) -> usize {
//         Self::MAX_SIZE_DB
//     }
// }

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KeyValueDriver;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct KeyValueStorage {
    base_path: PathBuf,
}
impl<'a> KeyValueStorage {
    pub fn new() -> KeyValueStorageBuilder {
        KeyValueStorageBuilder::default()
    }
    pub fn open_table(&self, table: &Table, create_mode: bool) -> ReboxResult<()> {
        use rkv::{
            backend::{SafeMode, SafeModeEnvironment},
            Manager, Rkv, StoreOptions,
        };
        use std::fs;
        use std::ops::Not;

        let mut root = self.base_path.clone();
        root.push("tables/");
        root.push(format!("{}/", table.name()));
        if root.is_dir().not() {
            dbg!(&root);
            fs::create_dir_all(&root)?;
        }

        dbg!(&root);
        let mut path_dbfile = PathBuf::from(&root);

        path_dbfile.push("data.safe.bin");

        if path_dbfile.exists().not() && create_mode {
            let mut manager = Manager::<SafeModeEnvironment>::singleton()
                .write()
                .map_err(|err| format_err!("{err}"))?;
            let created_arc = manager
                .get_or_create(root.as_path(), |p| Rkv::new::<SafeMode>(p))
                .unwrap();
            let k = created_arc.read().unwrap();
            {
                let store = k.open_single("master", StoreOptions::create())?;
                let mut writer = k.write()?;
                // TODO: Define schema
                // store.put(&mut writer, "some_key", &Value::Str("some_value"))?;
                writer.commit().map_err(|err| format_err!("{err}"))?;
            }
            {
                let store = k.open_single("schema", StoreOptions::create())?;
                let mut writer = k.write()?;
                // TODO: Define schema
                // store.put(&mut writer, "some_key", &Value::Str("some_value"))?;
                writer.commit().map_err(|err| format_err!("{err}"))?;
            }

            {
                let store = k.open_single("sequence", StoreOptions::create())?;
                let mut writer = k.write()?;
                // TODO: Define schema
                // store.put(&mut writer, "some_key", &Value::Str("some_value"))?;
                writer.commit().map_err(|err| format_err!("{err}"))?;
            }
        }
        // TODO Return some connection data structure
        Ok(())
    }
}
