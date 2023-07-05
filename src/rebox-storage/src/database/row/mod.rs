use std::collections::BTreeMap;

use anyhow::bail;

use rebox_types::{schema::column::model::ColumnName, ReboxResult};

use self::column::TableColumn;

pub(super) mod column;

#[derive(Debug, Default, Clone)]
pub struct TableRow(BTreeMap<ColumnName, TableColumn>);

impl TableRow {
    pub fn new(columns: Vec<TableColumn>) -> ReboxResult<Self> {
        if columns.is_empty() {
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
