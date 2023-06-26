use anyhow::bail;

use crate::{
    schema::{
        column::{ColumnKind, ColumnName, SchemaColumn},
        name::TableName,
        CurrentRowId, Table,
    },
    ReboxResult,
};

use std::{collections::BTreeMap, fmt::Debug};

use self::builder::DatabaseBuilder;
pub use self::fields::{name::DatabaseName, rebox_sequence::ReboxSequence, tables::DatabaseTables};

pub mod builder;
mod fields;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Database {
    name: DatabaseName,
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
    pub fn insert_into_table<T: AsRef<str>>(
        &mut self,
        table: T,
        row: TableRow,
    ) -> ReboxResult<CurrentRowId> {
        let table_name = TableName::new(table.as_ref());
        self.rebox_sequence.check_can_inc_rowid(&table_name)?;
        let cur_row_id = self.tables.insert_into_table(table_name.to_owned(), row)?;
        self.rebox_sequence.bump_table_cur_rowid(&table_name)?;
        Ok(cur_row_id)
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Debug, Default, Clone)]
pub struct TableRow(BTreeMap<ColumnName, TableColumn>);

impl TableRow {
    pub fn new(columns: Vec<TableColumn>) -> ReboxResult<Self> {
        let mut row = BTreeMap::new();

        columns
            .into_iter()
            .map(|column| {
                if row.contains_key(column.name()) {
                    bail!("Column already defined");
                } else {
                    row.insert(column.name().to_owned(), column);
                }

                Ok(())
            })
            .collect::<ReboxResult<Vec<()>>>()?;

        Ok(Self(row))
    }
}

#[derive(Debug, Clone)]
pub struct TableColumn {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
    value: Option<ColumnValue>,
}

impl From<SchemaColumn> for TableColumn {
    fn from(value: SchemaColumn) -> Self {
        let (name, kind, is_nullable) = value.take();
        Self {
            name,
            kind,
            is_nullable,
            value: None,
        }
    }
}

/*
SchemaColumn
 */
impl TableColumn {
    pub fn name(&self) -> &ColumnName {
        &self.name
    }

    pub fn set_value(&mut self, value: ColumnValue) {
        self.value = Some(value);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColumnValue {
    Bool(bool),
    Integer(i32),
    Natural(u32),
    Text(String),
}
