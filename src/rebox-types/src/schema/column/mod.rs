use bincode::{Decode, Encode};

use crate::{helpers::check_valid_entity_name, ReboxResult};

use self::model::{ColumnKind, ColumnName};

pub mod model;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Decode, Encode)]
pub struct SchemaColumn {
    name: ColumnName,
    kind: ColumnKind,
    // TODO
    is_nullable: bool,
    // TODO
    is_unique: bool,
    // TODO
    is_primary_key: bool,
    // TODO
    is_auto_increment: bool,
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
}

#[derive(Debug)]
pub struct SchemaColumnBuilder;
impl SchemaColumnBuilder {
    pub fn name<T: AsRef<str>>(self, name: T) -> ReboxResult<SchemaColumnBuilderS1> {
        check_valid_entity_name(&name)?;
        Ok(SchemaColumnBuilderS1 { name: name.into() })
    }
}

#[derive(Debug)]
pub struct SchemaColumnBuilderS1 {
    name: ColumnName,
}
impl SchemaColumnBuilderS1 {
    pub fn kind(self, kind: ColumnKind) -> SchemaColumnBuilderS2 {
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
    pub fn not_null(self, yes: bool) -> SchemaColumnBuilderS3 {
        let Self { name, kind } = self;
        SchemaColumnBuilderS3 {
            name,
            kind,
            is_nullable: yes,
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
    pub fn unique(self, yes: bool) -> SchemaColumnBuilderS4 {
        let Self {
            name,
            kind,
            is_nullable,
        } = self;
        SchemaColumnBuilderS4 {
            name,
            kind,
            is_nullable,
            is_unique: yes,
        }
    }
}

#[derive(Debug)]
pub struct SchemaColumnBuilderS4 {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
    is_unique: bool,
}

impl SchemaColumnBuilderS4 {
    pub fn primary_key(self, yes: bool) -> SchemaColumnBuilderS5 {
        let Self {
            name,
            kind,
            is_nullable,
            is_unique,
        } = self;
        SchemaColumnBuilderS5 {
            name,
            kind,
            is_nullable,
            is_unique,
            is_primary_key: yes,
        }
    }
}

#[derive(Debug)]
pub struct SchemaColumnBuilderS5 {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
    is_unique: bool,
    is_primary_key: bool,
}
impl SchemaColumnBuilderS5 {
    pub fn auto_increment(self, yes: bool) -> SchemaColumnBuilderS6 {
        let Self {
            name,
            kind,
            is_nullable,
            is_unique,
            is_primary_key,
        } = self;
        SchemaColumnBuilderS6 {
            name,
            kind,
            is_nullable,
            is_unique,
            is_primary_key,
            is_auto_increment: yes,
        }
    }
}

#[derive(Debug)]
pub struct SchemaColumnBuilderS6 {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
    is_unique: bool,
    is_primary_key: bool,
    is_auto_increment: bool,
}

impl SchemaColumnBuilderS6 {
    pub fn build(self) -> SchemaColumn {
        let Self {
            name,
            kind,
            is_nullable,
            is_unique,
            is_primary_key,
            is_auto_increment,
        } = self;
        SchemaColumn {
            name,
            kind,
            is_nullable,
            is_unique,
            is_primary_key,
            is_auto_increment,
        }
    }
}
