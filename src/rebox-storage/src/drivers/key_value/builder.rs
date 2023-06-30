use std::env;
use std::{path::PathBuf, str::FromStr};

use anyhow::format_err;

use rebox_types::helpers::check_valid_entity_name;
use rebox_types::{helpers::project_root, schema::name::TableName, ReboxResult};

use crate::database::fields::rebox_master::ReboxMaster;
use crate::database::fields::rebox_schema::ReboxSchema;
use crate::database::fields::rebox_sequence::ReboxSequence;

use super::KeyValueStorage;

#[derive(Debug, Default)]
pub struct KeyValueStorageBuilder;

impl KeyValueStorageBuilder {
    pub fn set_name<T: AsRef<str>>(self, name: T) -> ReboxResult<KeyValueStorageBuilderS1> {
        let database_name = name.as_ref().to_owned();
        check_valid_entity_name(&database_name)?;
        Ok(KeyValueStorageBuilderS1 {
            database_name,
            ..Default::default()
        })
    }
}

#[derive(Debug, Default)]
pub struct KeyValueStorageBuilderS1 {
    database_name: String,
    maybe_path_str: Option<String>,
}
impl KeyValueStorageBuilderS1 {
    pub fn set_path<T: AsRef<str>>(self, path: T) -> ReboxResult<Self> {
        let Self { database_name, .. } = self;
        Ok(Self {
            database_name,
            maybe_path_str: Some(path.as_ref().to_owned()),
        })
    }
    pub fn build(self) -> ReboxResult<KeyValueStorage> {
        let Self {
            database_name,
            maybe_path_str,
        } = self;
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
        base_path.push(format!("{database_name}/"));

        Self::bootstrap_metadata(&base_path)?;

        Ok(KeyValueStorage { base_path })
    }
    fn bootstrap_metadata(base_path: &PathBuf) -> ReboxResult<()> {
        // TODO: Implement new/open session
        Self::create_metadata_table(base_path, ReboxSequence::default().table_name())?;
        Self::create_metadata_table(base_path, ReboxMaster::default().table_name())?;
        Self::create_metadata_table(base_path, ReboxSchema::default().table_name())?;
        Ok(())
    }
    fn create_metadata_table(base_path: &PathBuf, table_name: &TableName) -> ReboxResult<()> {
        use rkv::{
            backend::{SafeMode, SafeModeEnvironment},
            Manager, Rkv, StoreOptions,
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
