pub mod builder;
mod moz_rkv;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::format_err;
use rebox_types::{
    schema::{name::TableName, Table},
    ReboxResult,
};
use rkv::{
    backend::{SafeMode, SafeModeEnvironment},
    Manager, Rkv,
};

use std::ops::Not;

use crate::database::fields::name::DatabaseName;
use crate::drivers::Driver;

use self::builder::KeyValueDriverBuilder;

use super::DataStorage;

pub type KvConnection = Arc<std::sync::RwLock<Rkv<SafeModeEnvironment>>>;

#[derive(Debug)]
pub struct KeyValueDriver {
    db_name: DatabaseName,
    base_path: PathBuf,
    create_mode: bool,
    connection: KvConnection,
}
impl KeyValueDriver {
    pub fn new() -> KeyValueDriverBuilder {
        KeyValueDriverBuilder::default()
    }
    fn bootstrap_metadata(&self) -> ReboxResult<()> {
        use crate::database::fields::{
            rebox_master::ReboxMaster, rebox_schema::ReboxSchema, rebox_sequence::ReboxSequence,
        };
        // TODO: Implement new/open session
        self.create_metadata_table(ReboxSequence::default().table_name())?;
        self.create_metadata_table(ReboxMaster::default().table_name())?;
        self.create_metadata_table(ReboxSchema::default().table_name())?;
        Ok(())
    }
    fn create_metadata_table(&self, table_name: &TableName) -> ReboxResult<()> {
        use rkv::{
            backend::{SafeMode, SafeModeEnvironment},
            Manager, Rkv, StoreOptions,
        };
        use std::fs;
        use std::ops::Not;

        // * Bootstrap DIRECTORY

        let mut root = self.base_path.clone();

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
// impl Driver for KeyValueDriver {

// }

// impl DataStorage for KeyValueDriver {
//     const MAX_SIZE_DB: usize = 1024 * 1024 * 1024 * 20; // 20 GBytes
//     fn max_dbsize(&self) -> usize {
//         Self::MAX_SIZE_DB
//     }
// }
