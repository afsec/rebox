use crate::{helpers::check_valid_name, ReboxResult};

pub use self::model::{ColumnData, ColumnKind, ColumnName};

mod model;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableColumn {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
}

impl TableColumn {
    pub fn new() -> TableColumnBuilder {
        TableColumnBuilder
    }
}

pub struct TableColumnBuilder;
impl TableColumnBuilder {
    pub fn set_name<T: AsRef<str>>(self, name: T) -> ReboxResult<TableColumnBuilderS1> {
        check_valid_name(&name)?;
        Ok(TableColumnBuilderS1 { name: name.into() })
    }
}

pub struct TableColumnBuilderS1 {
    name: ColumnName,
}
impl TableColumnBuilderS1 {
    pub fn set_kind(self, kind: ColumnKind) -> TableColumnBuilderS2 {
        let Self { name } = self;
        TableColumnBuilderS2 { name, kind }
    }
}
pub struct TableColumnBuilderS2 {
    name: ColumnName,
    kind: ColumnKind,
}

impl TableColumnBuilderS2 {
    pub fn is_nullable(self, is_nullable: bool) -> TableColumnBuilderS3 {
        let Self { name, kind } = self;
        TableColumnBuilderS3 {
            name,
            kind,
            is_nullable,
        }
    }
}

pub struct TableColumnBuilderS3 {
    name: ColumnName,
    kind: ColumnKind,
    is_nullable: bool,
}

impl TableColumnBuilderS3 {
    pub fn build(self) -> TableColumn {
        let Self {
            name,
            kind,
            is_nullable,
        } = self;
        TableColumn {
            name,
            kind,
            is_nullable,
        }
    }
}
