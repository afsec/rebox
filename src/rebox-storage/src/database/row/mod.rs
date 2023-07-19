use self::column::TableColumn;
use anyhow::bail;
use rebox_types::{schema::Table, ReboxResult};
use std::collections::BTreeMap;

pub mod column;

#[derive(Debug, Default, Clone)]
pub struct TableRow {
    row: BTreeMap<String, TableColumn>,
    is_verified: bool,
}
impl From<&Table> for TableRow {
    fn from(table: &Table) -> Self {
        let row: BTreeMap<String, TableColumn> = table
            .schema()
            .get_columns()
            .iter()
            .map(|(col_name, schema_col)| ((**col_name).to_owned(), TableColumn::from(schema_col)))
            .collect::<BTreeMap<String, TableColumn>>();

        Self {
            row,
            ..Default::default()
        }
    }
}
impl TableRow {
    pub fn new(columns: Vec<TableColumn>) -> ReboxResult<Self> {
        if columns.is_empty() {
            bail!("Can't build a table row without column")
        }

        let mut row = BTreeMap::new();

        columns
            .into_iter()
            .map(|column| {
                let col_name = column.name();
                let col_name_str = &**col_name;
                if row.contains_key(col_name_str) {
                    bail!("Column already defined");
                } else {
                    row.insert(col_name_str.to_owned(), column);
                }

                Ok(())
            })
            .collect::<ReboxResult<Vec<()>>>()?;

        Ok(Self {
            row,
            ..Default::default()
        })
    }

    pub fn is_verified(&self) -> bool {
        self.is_verified
    }

    pub fn verify(&mut self) -> ReboxResult<()> {
        self.get().iter().try_for_each(|(key, column)| {
            match (column.is_nullable(), column.value().is_some()) {
                (true, true) | (false, _) => Ok(()),
                _ => bail!(
                    "TableRow verification failed in key: [{key}] with value: [{:?}]",
                    column.value()
                ),
            }
        })?;
        self.is_verified = true;
        Ok(())
    }
    pub fn check_verified(&self) -> ReboxResult<()> {
        if self.is_verified {
            Ok(())
        } else {
            bail!("TableRow is not vefified try `.verify()?;`")
        }
    }
    pub fn get(&self) -> &BTreeMap<String, TableColumn> {
        &self.row
    }

    pub fn get_mut(&mut self) -> &mut BTreeMap<String, TableColumn> {
        &mut self.row
    }
}
