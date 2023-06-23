use anyhow::bail;

use crate::{
    table::{CurrentRowId, Table, TableName, TableRow},
    ReboxResult,
};

use std::{cell::RefCell, fmt::Debug};

use self::tables::{DatabaseTables, ReboxSequence};

mod tables;

#[cfg(test)]
mod tests;

const DB_NAME_MAX_CHARS: usize = 255;

#[derive(Debug)]
pub struct Database {
    name: String,
    rebox_sequence: ReboxSequence,
    tables: DatabaseTables,
}

impl Database {
    pub fn new() -> DatabaseBuilder {
        DatabaseBuilder
    }

    pub fn list_tables(&self) -> Vec<&TableName> {
        self.tables.list_tables()
    }
    pub fn create_table(&mut self, table: Table) -> ReboxResult<TableName> {
        let table_name = self.tables.create_table(table)?;

        Ok(table_name)
    }
    pub fn insert_into_table(
        &mut self,
        table_name: TableName,
        table_row: TableRow,
    ) -> ReboxResult<CurrentRowId> {
        self.rebox_sequence.check_can_inc_rowid(&table_name)?;
        let cur_row_id = self
            .tables
            .insert_into_table(table_name.to_owned(), table_row)?;
        self.rebox_sequence.bump_table_cur_rowid(&table_name)?;
        Ok(cur_row_id)
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

pub struct DatabaseBuilder;

impl DatabaseBuilder {
    pub fn database_name<S: AsRef<str>>(self, database_name: S) -> ReboxResult<BuilderWithParams> {
        let res = check_valid_name(&database_name);
        dbg!(res);
        Ok(BuilderWithParams {
            name: database_name.as_ref().to_string(),
        })
    }
}

#[derive(Debug, Default)]
pub struct BuilderWithParams {
    name: String,
}
impl BuilderWithParams {
    pub fn build(self) -> ReboxResult<Database> {
        let Self { name } = self;
        // TODO
        Ok(Database {
            name,
            rebox_sequence: ReboxSequence::default(),
            tables: Default::default(),
        })
    }
}

fn check_valid_name<T: AsRef<str>>(name: T) -> ReboxResult<()> {
    let input_str = name.as_ref();
    if input_str.chars().map(|_| 1).sum::<usize>() >= DB_NAME_MAX_CHARS {
        bail!("Database name can't be larger than {DB_NAME_MAX_CHARS} characters.");
    }
    let mut current_position = RefCell::new(0usize);
    input_str
        .chars()
        .map(|ch| {
            dbg!(ch);
            if ch.is_ascii_alphanumeric() || ch == '-' {
                *current_position.get_mut() += 1;

                Ok(())
            } else {
                dbg!("Should return Error!");
                bail!("Database name has invalid chars")
            }
        })
        .collect::<ReboxResult<()>>()?;

    dbg!(current_position);

    dbg!(&input_str);
    Ok(())
}
