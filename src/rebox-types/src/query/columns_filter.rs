use std::{collections::BTreeSet, fmt::Display, ops::Not};

use anyhow::bail;

use crate::{schema::column::model::ColumnName, ReboxResult};

#[derive(Debug)]
pub enum ColumnsFilter {
    NoFilter,
    Filter(BTreeSet<ColumnName>),
}

impl Display for ColumnsFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::NoFilter => vec!["*".into()],
            Self::Filter(inner) => inner
                .iter()
                .map(|column_name| column_name.to_string())
                .collect::<Vec<String>>(),
        };
        write!(f, "({})", output.join(", "))
    }
}

impl ColumnsFilter {
    pub fn new() -> ColumnsFilterBuilder {
        Default::default()
    }
    pub fn contains(&self, column_name: &ColumnName) -> bool {
        match self {
            Self::Filter(inner) => inner.contains(column_name),
            Self::NoFilter => true,
        }
    }
}

#[derive(Debug, Default)]
pub struct ColumnsFilterBuilder(BTreeSet<ColumnName>);
impl ColumnsFilterBuilder {
    pub fn column<T: AsRef<str>>(mut self, column_name: T) -> ReboxResult<Self> {
        let is_ok = self.0.insert(column_name.as_ref().try_into()?);
        if is_ok.not() {
            bail!("Column name has already added to retrieve")
        }
        Ok(self)
    }
    pub fn build(self) -> ColumnsFilter {
        if self.0.is_empty() {
            ColumnsFilter::NoFilter
        } else {
            ColumnsFilter::Filter(self.0)
        }
    }
}
