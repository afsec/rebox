use crate::database::MetadataTable;
use anyhow::bail;
use rebox_derive::DbEntity;
use rebox_types::{
    schema::{name::TableName, RowId},
    DbPrefix, ReboxResult,
};
use std::collections::BTreeMap;

#[derive(Debug, DbEntity)]
pub(crate) struct ReboxMaster {
    table_name: TableName,
    inner_data: BTreeMap<TableName, RowId>,
}

impl MetadataTable for ReboxMaster {
    fn table_name(&self) -> &TableName {
        &self.table_name
    }
}

impl ReboxMaster {
    pub(crate) fn bump_table_cur_rowid(&mut self, table_name: &TableName) -> ReboxResult<()> {
        self.check_can_inc_rowid(table_name)?;
        let cur_row_id = self
            .inner_data
            .entry(table_name.to_owned())
            .or_insert(RowId::default());

        cur_row_id.inc()?;

        Ok(())
    }
    pub(crate) fn check_can_inc_rowid(&self, table_name: &TableName) -> ReboxResult<()> {
        if let Some(cur_row_id) = self.inner_data.get(table_name) {
            if cur_row_id.is_full() {
                bail!("Table [{table_name}] reached max row id");
            }
        }
        Ok(())
    }

    pub(crate) fn table_name(&self) -> &TableName {
        &self.table_name
    }
}

impl Default for ReboxMaster {
    fn default() -> Self {
        Self {
            table_name: TableName::new(format!("{}_master", Self::prefix())),
            inner_data: Default::default(),
        }
    }
}
