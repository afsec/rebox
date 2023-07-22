pub mod data;

use anyhow::bail;
use rebox_types::{
    helpers::check_valid_entity_name,
    schema::column::{
        model::{ColumnKind, ColumnName, ColumnValue},
        SchemaColumn,
    },
    ReboxResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableColumn {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
    value: Option<ColumnValue>,
}

impl PartialEq<SchemaColumn> for TableColumn {
    fn eq(&self, other: &SchemaColumn) -> bool {
        self.name() == other.name()
            && self.kind() == other.kind()
            && self.is_nullable() == other.is_nullable()
    }
}

impl From<&SchemaColumn> for TableColumn {
    fn from(schema_column: &SchemaColumn) -> Self {
        Self {
            name: schema_column.name().clone(),
            kind: schema_column.kind().clone(),
            is_nullable: schema_column.is_nullable().clone(),
            value: None,
        }
    }
}
impl TableColumn {
    pub fn new() -> TableColumnBuilder {
        TableColumnBuilder
    }

    pub fn name(&self) -> &ColumnName {
        &self.name
    }

    pub fn kind(&self) -> &ColumnKind {
        &self.kind
    }

    pub fn is_nullable(&self) -> bool {
        self.is_nullable
    }

    pub fn value(&self) -> Option<&ColumnValue> {
        self.value.as_ref()
    }

    pub fn set_value<T: Into<ColumnValue>>(&mut self, value: T) {
        self.value = Some(value.into());
    }

    pub fn build(self) -> Self {
        self
    }
}

#[derive(Debug)]
pub struct TableColumnBuilder;
impl TableColumnBuilder {
    pub fn set_name<T: AsRef<str>>(self, name: T) -> ReboxResult<TableColumnBuilderS1> {
        check_valid_entity_name(&name)?;
        Ok(TableColumnBuilderS1 { name: name.into() })
    }
}

#[derive(Debug)]
pub struct TableColumnBuilderS1 {
    name: ColumnName,
}
impl TableColumnBuilderS1 {
    pub fn set_kind(self, kind: ColumnKind) -> TableColumnBuilderS2 {
        let Self { name } = self;
        TableColumnBuilderS2 { name, kind }
    }
}

#[derive(Debug)]
pub struct TableColumnBuilderS2 {
    name: ColumnName,
    kind: ColumnKind,
}
impl TableColumnBuilderS2 {
    pub fn set_nullable(self, is_nullable: bool) -> TableColumnBuilderS3 {
        let Self { name, kind } = self;
        TableColumnBuilderS3 {
            name,
            kind,
            is_nullable,
            value: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct TableColumnBuilderS3 {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
    value: Option<ColumnValue>,
}
impl TableColumnBuilderS3 {
    pub fn set_value(self, column_value: ColumnValue) -> ReboxResult<Self> {
        let Self {
            name,
            kind,
            is_nullable,
            value,
        } = self;
        if value.is_some() {
            bail!("Column value [{name}] is already defined");
        }

        Ok(Self {
            name,
            kind,
            is_nullable,
            value: Some(column_value),
        })
    }
    pub fn build(self) -> ReboxResult<TableColumn> {
        use std::ops::Not;
        let Self {
            name,
            kind,
            is_nullable,
            value,
        } = self;

        if is_nullable.not() && value.is_none() {
            bail!("Column value [{name}] is not a nullable type you should use `.set_value(...)` builder method");
        }
        Ok(TableColumn {
            name,
            kind,
            is_nullable,
            value,
        })
    }
}
