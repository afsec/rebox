use anyhow::format_err;
use rebox_types::ReboxResult;
use rkv::{
    backend::{SafeMode, SafeModeDatabase, SafeModeEnvironment},
    Manager, Rkv, SingleStore, StoreError, StoreOptions, Value,
};
use tempfile::Builder;

use super::KeyValueDriver;

impl KeyValueDriver {
    pub fn run(self) -> ReboxResult<()> {
        use std::{fs, str};
        let root = Builder::new().prefix("iterator").tempdir()?;
        fs::create_dir_all(root.path())?;
        let p = root.path();

        let mut manager = Manager::<SafeModeEnvironment>::singleton()
            .write()
            .map_err(|err| format_err!("{err}"))?;
        let created_arc = manager.get_or_create(p, Rkv::new::<SafeMode>)?;
        let k = created_arc.read().map_err(|err| format_err!("{err}"))?;
        let store = k.open_single("store", StoreOptions::create())?;

        Self::populate_store(&k, store).unwrap();

        let reader = k.read().unwrap();

        println!("Iterating from the beginning...");
        // Reader::iter_start() iterates from the first item in the store, and
        // returns the (key, value) tuples in order.
        let mut iter = store.iter_start(&reader).unwrap();
        while let Some(Ok((country, city))) = iter.next() {
            let country = str::from_utf8(country).unwrap();
            println!("{country}, {city:?}");
        }

        println!();
        println!("Iterating from the given key...");
        // Reader::iter_from() iterates from the first key equal to or greater
        // than the given key.
        let mut iter = store.iter_from(&reader, "Japan").unwrap();
        while let Some(Ok((country, city))) = iter.next() {
            println!("{}, {city:?}", str::from_utf8(country).unwrap());
        }

        println!();
        println!("Iterating from the given prefix...");
        let mut iter = store.iter_from(&reader, "Un").unwrap();
        while let Some(Ok((country, city))) = iter.next() {
            println!("{}, {city:?}", str::from_utf8(country).unwrap());
        }
        Ok(())
    }
    fn populate_store(
        k: &Rkv<SafeModeEnvironment>,
        store: SingleStore<SafeModeDatabase>,
    ) -> Result<(), StoreError> {
        let mut writer = k.write()?;
        for (country, city) in vec![
            ("Canada", Value::Str("Ottawa")),
            ("United States of America", Value::Str("Washington")),
            ("Germany", Value::Str("Berlin")),
            ("France", Value::Str("Paris")),
            ("Italy", Value::Str("Rome")),
            ("United Kingdom", Value::Str("London")),
            ("Japan", Value::Str("Tokyo")),
        ] {
            store.put(&mut writer, country, &city)?;
        }
        writer.commit()
    }
}
