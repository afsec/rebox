use std::env;
use std::{path::PathBuf, str::FromStr};

use anyhow::format_err;
use rebox_types::helpers::project_root;
use rebox_types::{schema::TableName, ReboxResult};

use crate::database::{ReboxMaster, ReboxSchema, ReboxSequence};
use crate::KeyValueStorage;

#[derive(Debug, Default)]
pub struct KeyValueStorageBuilder {
    maybe_path_str: Option<String>,
}

impl<'a> KeyValueStorageBuilder {
    pub fn set_path<T: AsRef<str>>(self, path: T) -> ReboxResult<Self> {
        // * Bootstrap DIRECTORY

        Ok(KeyValueStorageBuilder {
            maybe_path_str: Some(path.as_ref().to_string()),
        })
    }
    pub fn build(self) -> ReboxResult<KeyValueStorage> {
        let Self { maybe_path_str } = self;
        let mut base_path = match maybe_path_str {
            Some(path_str) => PathBuf::from_str(&path_str)?,
            None => {
                if cfg!(debug_assertions) {
                    project_root()?
                } else {
                    env::current_dir()?
                }
            }
        };
        base_path.push("rebox_data/");

        Self::create_metadata_tables(&base_path)?;

        Ok(KeyValueStorage { base_path })
    }
    fn create_metadata_tables(base_path: &PathBuf) -> ReboxResult<()> {
        // TODO: Implement new/open session
        let rebox_sequence = ReboxSequence::default();
        Self::create_metatable(&base_path, rebox_sequence.table_name())?;
        let rebox_master = ReboxMaster::default();
        Self::create_metatable(&base_path, rebox_master.table_name())?;
        let rebox_schema = ReboxSchema::default();
        Self::create_metatable(&base_path, rebox_schema.table_name())?;
        Ok(())
    }
    fn create_metatable(base_path: &PathBuf, table_name: &TableName) -> ReboxResult<()> {
        use rkv::{
            backend::{SafeMode, SafeModeDatabase, SafeModeEnvironment},
            Manager, Rkv, SingleStore, StoreError, StoreOptions, Value,
        };
        use std::fs;
        use std::ops::Not;

        // * Bootstrap DIRECTORY

        let mut root = PathBuf::from(base_path);

        root.push("metadata/");
        root.push(format!("{}/", table_name));
        if root.is_dir().not() {
            dbg!(&root);
            fs::create_dir_all(&root)?;
        }

        dbg!(&root);

        let mut path_dbfile = PathBuf::from(&root);
        
        path_dbfile.push("data.safe.bin");
        
        if path_dbfile.exists().not() {
            let mut manager = Manager::<SafeModeEnvironment>::singleton()
                .write()
                .map_err(|err| format_err!("{err}"))?;
            let created_arc = manager
                .get_or_create(root.as_path(), |p| Rkv::new::<SafeMode>(p))
                .unwrap();
            let k = created_arc.read().unwrap();
            let store = k.open_single(table_name.to_string().as_str(), StoreOptions::create())?;
            let mut writer = k.write()?;
            // store.put(&mut writer, "some_key", &Value::Str("some_value"))?;
            writer.commit().map_err(|err| format_err!("{err}"))?;
        }
        Ok(())
    }
}
