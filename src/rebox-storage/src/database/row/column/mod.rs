use anyhow::bail;

use rebox_types::{
    helpers::check_valid_entity_name,
    schema::column::model::{ColumnKind, ColumnName},
    ReboxResult,
};

#[derive(Debug, Clone)]
pub(crate) struct TableColumn {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
    value: Option<ColumnValue>,
}

impl TableColumn {
    pub(crate) fn new() -> TableColumnBuilder {
        TableColumnBuilder
    }
}

/*
TableColumn
 */
impl TableColumn {
    pub(crate) fn name(&self) -> &ColumnName {
        &self.name
    }

    pub(crate) fn set_value(mut self, value: ColumnValue) -> Self {
        self.value = Some(value);
        self
    }
    pub(crate) fn build(self) -> Self {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ColumnValue {
    Bool(bool),
    Integer(i32),
    Natural(u32),
    Text(String),
}

pub(crate) struct TableColumnBuilder;
impl TableColumnBuilder {
    pub(crate) fn set_name<T: AsRef<str>>(self, name: T) -> ReboxResult<TableColumnBuilderS1> {
        check_valid_entity_name(&name)?;
        Ok(TableColumnBuilderS1 { name: name.into() })
    }
}

pub(crate) struct TableColumnBuilderS1 {
    name: ColumnName,
}
impl TableColumnBuilderS1 {
    pub(crate) fn set_kind(self, kind: ColumnKind) -> TableColumnBuilderS2 {
        let Self { name } = self;
        TableColumnBuilderS2 { name, kind }
    }
}
pub(crate) struct TableColumnBuilderS2 {
    name: ColumnName,
    kind: ColumnKind,
}

impl TableColumnBuilderS2 {
    pub(crate) fn set_nullable(self, is_nullable: bool) -> TableColumnBuilderS3 {
        let Self { name, kind } = self;
        TableColumnBuilderS3 {
            name,
            kind,
            is_nullable,
            value: Default::default(),
        }
    }
}

pub(crate) struct TableColumnBuilderS3 {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
    value: Option<ColumnValue>,
}

impl TableColumnBuilderS3 {
    pub(crate) fn set_value(self, column_value: ColumnValue) -> ReboxResult<Self> {
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
    pub(crate) fn build(self) -> ReboxResult<TableColumn> {
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
