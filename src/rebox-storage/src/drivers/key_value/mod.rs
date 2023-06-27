mod moz_rkv;

use anyhow::format_err;
use rebox_types::{helpers::project_root, schema::TableName, ReboxResult};

use crate::drivers::Driver;

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

#[derive(Debug, Default, PartialEq, Eq)]
pub struct KeyValueStorage;
impl<'a> KeyValueStorage {
    pub fn open_metadata<T: Into<&'a TableName>>(table: T) -> ReboxResult<()> {
        use rkv::{
            backend::{SafeMode, SafeModeDatabase, SafeModeEnvironment},
            Manager, Rkv, SingleStore, StoreError, StoreOptions, Value,
        };
        use std::fs;
        use std::ops::Not;

        // * Bootstrap DIRECTORY

        let mut root = project_root()?;
        root.push("rebox_data/");
        root.push("metadata/");
        if root.is_dir().not() {
            dbg!(&root);
            fs::create_dir_all(&root)?;
        }

        dbg!(&root);

        let table_name = table.into().to_string();

        let mut manager = Manager::<SafeModeEnvironment>::singleton()
            .write()
            .map_err(|err| format_err!("{err}"))?;
        let created_arc = manager
            .get_or_create(root.as_path(), |p| Rkv::new::<SafeMode>(p))
            .unwrap();
        let k = created_arc.read().unwrap();
        let store = k.open_single(table_name.as_str(), StoreOptions::create())?;
        let mut writer = k.write()?;
        // store.put(&mut writer, "some_key", &Value::Str("some_value"))?;
        writer.commit().map_err(|err| format_err!("{err}"))
    }
    pub fn open_table<T: Into<&'a TableName>>(table: T) -> ReboxResult<()> {
        use rkv::{
            backend::{SafeMode, SafeModeDatabase, SafeModeEnvironment},
            Manager, Rkv, SingleStore, StoreError, StoreOptions, Value,
        };
        use std::fs;
        use std::ops::Not;

        // * Bootstrap DIRECTORY

        let mut root = project_root()?;

        let table_name = table.into().to_string();

        root.push("rebox_data/");
        root.push("tables/");
        root.push(format!("{}/", table_name));
        if root.is_dir().not() {
            dbg!(&root);
            fs::create_dir_all(&root)?;
        }

        dbg!(&root);

        let mut manager = Manager::<SafeModeEnvironment>::singleton()
            .write()
            .map_err(|err| format_err!("{err}"))?;
        let created_arc = manager
            .get_or_create(root.as_path(), |p| Rkv::new::<SafeMode>(p))
            .unwrap();
        let k = created_arc.read().unwrap();
        let store = k.open_single(table_name.as_str(), StoreOptions::create())?;
        let mut writer = k.write()?;
        // store.put(&mut writer, "some_key", &Value::Str("some_value"))?;
        writer.commit().map_err(|err| format_err!("{err}"))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KeyValueDriver;
