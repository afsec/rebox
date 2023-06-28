use crate::{helpers::check_valid_entity_name, ReboxResult};

pub use self::model::{ColumnKind, ColumnName};

// pub(crate) use self::model::ColumnData;

mod model;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaColumn {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
}

impl SchemaColumn {
    pub fn new() -> SchemaColumnBuilder {
        SchemaColumnBuilder
    }

    pub fn name(&self) -> &ColumnName {
        &self.name
    }
    pub fn take(self) -> (ColumnName, ColumnKind, bool) {
        let Self {
            name,
            kind,
            is_nullable,
        } = self;
        (name, kind, is_nullable)
    }
}

#[derive(Debug)]
pub struct SchemaColumnBuilder;
impl SchemaColumnBuilder {
    pub fn set_name<T: AsRef<str>>(self, name: T) -> ReboxResult<SchemaColumnBuilderS1> {
        check_valid_entity_name(&name)?;
        Ok(SchemaColumnBuilderS1 { name: name.into() })
    }
}

#[derive(Debug)]
pub struct SchemaColumnBuilderS1 {
    name: ColumnName,
}
impl SchemaColumnBuilderS1 {
    pub fn set_kind(self, kind: ColumnKind) -> SchemaColumnBuilderS2 {
        let Self { name } = self;
        SchemaColumnBuilderS2 { name, kind }
    }
}

#[derive(Debug)]
pub struct SchemaColumnBuilderS2 {
    name: ColumnName,
    kind: ColumnKind,
}

impl SchemaColumnBuilderS2 {
    pub fn is_nullable(self, is_nullable: bool) -> SchemaColumnBuilderS3 {
        let Self { name, kind } = self;
        SchemaColumnBuilderS3 {
            name,
            kind,
            is_nullable,
        }
    }
}

#[derive(Debug)]
pub struct SchemaColumnBuilderS3 {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
}

impl SchemaColumnBuilderS3 {
    pub fn build(self) -> SchemaColumn {
        let Self {
            name,
            kind,
            is_nullable,
        } = self;
        SchemaColumn {
            name,
            kind,
            is_nullable,
        }
    }
}
