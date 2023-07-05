use std::{env, path::PathBuf, str::FromStr};

use anyhow::format_err;

use rebox_types::{helpers::project_root, ReboxResult};

use crate::database::name::DatabaseName;

use super::KeyValueDriver;

#[derive(Debug, Default)]
pub(crate) struct KeyValueDriverBuilder;

impl KeyValueDriverBuilder {
    pub(crate) fn set_name(self, db_name: DatabaseName) -> ReboxResult<KeyValueDriverBuilderS1> {
        Ok(KeyValueDriverBuilderS1 {
            db_name,
            ..Default::default()
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct KeyValueDriverBuilderS1 {
    db_name: DatabaseName,
    maybe_path_str: Option<String>,
    create_mode: bool,
}

impl KeyValueDriverBuilderS1 {
    pub(crate) fn set_path<T: AsRef<str>>(self, path: T) -> Self {
        let Self {
            db_name,
            create_mode,
            ..
        } = self;
        Self {
            db_name,
            maybe_path_str: Some(path.as_ref().to_owned()),
            create_mode,
        }
    }
    pub(crate) fn create_mode(self, yes: bool) -> Self {
        let Self {
            db_name,
            maybe_path_str,
            ..
        } = self;
        Self {
            db_name,
            maybe_path_str,
            create_mode: yes,
        }
    }
    pub(crate) fn build(self) -> ReboxResult<KeyValueDriverBuilderS2> {
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
pub(crate) struct KeyValueDriverBuilderS2 {
    db_name: DatabaseName,
    base_path: PathBuf,
    create_mode: bool,
}
impl KeyValueDriverBuilderS2 {
    pub(crate) fn connect(self) -> ReboxResult<KeyValueDriver> {
        use rkv::{
            backend::{SafeMode, SafeModeEnvironment},
            Manager, Rkv,
        };
        use std::{fs, ops::Not};

        let root = self.base_path.clone();

        if root.is_dir().not() {
            dbg!(&root);
            fs::create_dir_all(&root)?;
        }

        let mut path_dbfile = self.base_path.clone();

        path_dbfile.push("data.safe.bin");

        dbg!(&root.as_path());
        dbg!(&path_dbfile.as_path());

        let manager = Manager::<SafeModeEnvironment>::singleton();

        // let mut reader = manager.read().map_err(|err| format_err!("{err}"))?;

        let mut writer = manager
            .write()
            .map_err(|err| format_err!("Writer Error: {err}"))?;
        dbg!();
        let connection = match (path_dbfile.is_file(), self.create_mode) {
            (false, true) => {
                Some(writer.get_or_create(root.as_path(), |p| Rkv::new::<SafeMode>(p))?)
            }
            (true, false) => writer.get(root.as_path())?,
            _ => None,
        }
        .ok_or(format_err!(
            "Error on open connection for Rkv database, try create mode"
        ))?;

        let Self {
            db_name,
            base_path,
            create_mode,
        } = self;
        let kv_driver = KeyValueDriver {
            db_name,
            base_path,
            create_mode,
            connection,
        };

        Ok(kv_driver)
    }
}
