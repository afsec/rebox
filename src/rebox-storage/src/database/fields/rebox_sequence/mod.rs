use anyhow::{bail, format_err};
use std::{collections::BTreeMap, path::PathBuf, str::FromStr};

use rebox_types::{
    helpers::project_root,
    schema::{CurrentRowId, TableName},
    ReboxResult,
};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct ReboxSequence {
    table_name: TableName,
    inner_data: BTreeMap<TableName, CurrentRowId>,
}

impl ReboxSequence {
    pub fn bump_table_cur_rowid(&mut self, table_name: &TableName) -> ReboxResult<()> {
        self.check_can_inc_rowid(table_name)?;
        let cur_row_id = self
            .inner_data
            .entry(table_name.to_owned())
            .or_insert(CurrentRowId::default());

        cur_row_id.inc()?;

        Ok(())
    }
    pub fn check_can_inc_rowid(&self, table_name: &TableName) -> ReboxResult<()> {
        if let Some(cur_row_id) = self.inner_data.get(table_name) {
            if cur_row_id.is_full() {
                bail!("Table [{table_name}] reached max row id");
            }
        }
        Ok(())
    }
    

    pub fn table_name(&self) -> &TableName {
        &self.table_name
    }
}
impl Default for ReboxSequence {
    fn default() -> Self {
        Self {
            table_name: TableName::new("sequence"),
            inner_data: Default::default(),
        }
    }
}
