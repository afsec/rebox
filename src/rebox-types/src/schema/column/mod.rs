use bincode::{Decode, Encode};

use crate::{helpers::check_valid_entity_name, ReboxResult};

use self::model::{ColumnKind, ColumnName};

pub mod model;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Decode, Encode)]
pub struct SchemaColumn {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
    is_unique: bool,
}

impl SchemaColumn {
    pub fn new() -> SchemaColumnBuilder {
        SchemaColumnBuilder
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

    // pub fn take(self) -> (ColumnName, ColumnKind, bool) {
    //     let Self {
    //         name,
    //         kind,
    //         is_nullable,
    //         is_unique,
    //     } = self;
    //     (name, kind, is_nullable)
    // }
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
    pub fn set_nullable(self, is_nullable: bool) -> SchemaColumnBuilderS3 {
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
    pub fn set_unique(self, is_unique: bool) -> SchemaColumnBuilderS4 {
        let Self { name, kind, is_nullable } = self;
        SchemaColumnBuilderS4 {
            name,
            kind,
            is_nullable,
            is_unique
        }
    }
}

#[derive(Debug)]
pub struct SchemaColumnBuilderS4 {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
    is_unique: bool
}

impl SchemaColumnBuilderS4 {
    pub fn build(self) -> SchemaColumn {
        let Self {
            name,
            kind,
            is_nullable,
            is_unique
        } = self;
        SchemaColumn {
            name,
            kind,
            is_nullable,
            is_unique,
        }
    }
}
