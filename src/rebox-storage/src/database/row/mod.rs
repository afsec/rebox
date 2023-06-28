use std::collections::BTreeMap;

use anyhow::bail;

use rebox_types::{schema::ColumnName, ReboxResult};

use self::column::TableColumn;

pub(crate) mod column;

#[derive(Debug, Default, Clone)]
pub(crate) struct TableRow(BTreeMap<ColumnName, TableColumn>);

impl TableRow {
    pub(crate) fn new(columns: Vec<TableColumn>) -> ReboxResult<Self> {
        if columns.len() < 1 {
            bail!("Can't build a table row without column")
        }

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
