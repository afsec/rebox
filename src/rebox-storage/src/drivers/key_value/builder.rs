use std::env;
use std::{path::PathBuf, str::FromStr};

use anyhow::format_err;

use rebox_types::helpers::check_valid_entity_name;
use rebox_types::{helpers::project_root, schema::name::TableName, ReboxResult};

use crate::database::fields::name::DatabaseName;
use crate::database::fields::rebox_master::ReboxMaster;
use crate::database::fields::rebox_schema::ReboxSchema;
use crate::database::fields::rebox_sequence::ReboxSequence;

use super::{KeyValueDriver, KvConnection};

// use super::KeyValueDriver;

#[derive(Debug, Default)]
pub struct KeyValueDriverBuilder;

impl KeyValueDriverBuilder {
    pub fn set_name(self, db_name: DatabaseName) -> ReboxResult<KeyValueDriverBuilderS1> {
        Ok(KeyValueDriverBuilderS1 {
            db_name,
            ..Default::default()
        })
    }
}

#[derive(Debug, Default)]
pub struct KeyValueDriverBuilderS1 {
    db_name: DatabaseName,
    maybe_path_str: Option<String>,
    create_mode: bool,
}

impl KeyValueDriverBuilderS1 {
    pub fn set_path<T: AsRef<str>>(self, path: T) -> Self {
        let Self {
            db_name,
            maybe_path_str,
            create_mode,
        } = self;
        Self {
            db_name,
            maybe_path_str: Some(path.as_ref().to_owned()),
            create_mode,
        }
    }
    pub fn create_mode(self, create_mode: bool) -> Self {
        let Self {
            db_name,
            maybe_path_str,
            create_mode,
        } = self;
        Self {
            db_name,
            maybe_path_str,
            create_mode,
        }
    }
    pub fn build(self) -> ReboxResult<KeyValueDriverBuilderS2> {
        let Self {
            db_name,
            maybe_path_str,
            create_mode,
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
        base_path.push(format!("{}/", &*db_name));

        Ok(KeyValueDriverBuilderS2 {
            db_name,
            base_path,
            create_mode,
        })
    }
}
pub struct KeyValueDriverBuilderS2 {
    db_name: DatabaseName,
    base_path: PathBuf,
    create_mode: bool,
}
impl KeyValueDriverBuilderS2 {
    pub fn connect(self) -> ReboxResult<KeyValueDriver> {
        use rkv::{
            backend::{SafeMode, SafeModeEnvironment},
            Manager, Rkv,
        };
        use std::{fs, ops::Not};

        let mut root = self.base_path.clone();
        root.push(format!("{}/", &*self.db_name));

        if root.is_dir().not() {
            dbg!(&root);
            fs::create_dir_all(&root)?;
        }

        dbg!(&root);
        let mut path_dbfile = PathBuf::from(&root);

        path_dbfile.push("data.safe.bin");

        let mut manager = Manager::<SafeModeEnvironment>::singleton()
            .write()
            .map_err(|err| format_err!("{err}"))?;

        let connection = match (path_dbfile.exists(), self.create_mode) {
            (false, true) => {
                Some(manager.get_or_create(root.as_path(), |p| Rkv::new::<SafeMode>(p))?)
            }
            (true, _) => manager.get(root.as_path())?,
            (false, false) => None,
        }
        .ok_or(format_err!("Error on open connection for Rkv database"))?;

        let Self {
            db_name,
            base_path,
            create_mode,
        } = self;
        Ok(KeyValueDriver {
            db_name,
            base_path,
            create_mode,
            connection,
        })
    }
}
