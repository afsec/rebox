use anyhow::{bail, format_err};
use bincode::config::Configuration;
use rebox_types::{
    schema::{name::TableName, schema::TableSchema, RowId},
    ReboxResult,
};
use rkv::{StoreOptions, Value};

use crate::database::{row::TableRow, DatabaseMetadata};

use super::KvConnection;

pub(super) fn retrieve_schema(
    connection: &KvConnection,
    metadata: &DatabaseMetadata,
    table_name: &TableName,
) -> ReboxResult<TableSchema> {
    let table_name_str = &**table_name;
    let rebox_master = metadata.rebox_master().table_name().as_ref();
    let created_arc = connection.clone();

    let rkv_env = created_arc
        .read()
        .map_err(|err| format_err!("Read error: {err}"))?;

    let master_store = rkv_env.open_single(rebox_master, StoreOptions::default())?;
    let reader = rkv_env.read()?;
    let maybe_value: Option<Value> = master_store.get(&reader, table_name_str)?;

    let blob = match maybe_value {
    Some(Value::Blob(inner_blob)) => inner_blob,
    other => bail!(
    "Health check alert: Table [{table_name_str}] type mismatch in [{rebox_master}]. Reason: {other:?}"
),
};
    let (retrieved_table_schema, _) = bincode::decode_from_slice::<TableSchema, Configuration>(
        blob,
        bincode::config::standard(),
    )?;
    Ok(retrieved_table_schema)
}

pub(super) fn retrieve_last_row_id(
    connection: &KvConnection,
    metadata: &DatabaseMetadata,
    table_name: &TableName,
) -> ReboxResult<RowId> {
    let table_name_str = &**table_name;

    let created_arc = connection.clone();
    let rebox_sequence = metadata.rebox_sequence().table_name().as_ref();

    let rkv_env = created_arc
        .read()
        .map_err(|err| format_err!("Read error: {err}"))?;

    let sequence_store = rkv_env.open_single(rebox_sequence, StoreOptions::default())?;

    let last_row_id: RowId = match sequence_store.get(&rkv_env.read()?, table_name_str) {
        Ok(Some(Value::U64(inner))) => inner.try_into()?,
        _ => bail!(
            " Can't retrieve rebox_sequence for table {}",
            table_name_str
        ),
    };
    Ok(last_row_id)
}

pub(super) fn check_row_against_schema(
    connection: &KvConnection,
    metadata: &DatabaseMetadata,
    table_name: &TableName,
    table_row: &TableRow,
) -> ReboxResult<()> {
    let tbl_schema = retrieve_schema(connection, metadata, table_name)?;
    let schema_cols = tbl_schema.get_columns();
    for (col_name, tbl_column) in table_row.get() {
        let schema_column = schema_cols.get(col_name).ok_or(format_err!(
            "Impossible State at {} {}",
            file!(),
            line!()
        ))?;
        if tbl_column != schema_column {
            bail!("Row is not matching againt {table_name} schema");
        }
    }
    Ok(())
}

// pub(super) fn check_column_value_against_schema(
//     connection: &KvConnection,
//     metadata: &DatabaseMetadata,
//     table_name: &TableName,
//     column_name: &ColumnName,
//     column_value: &ColumnValue,
// ) -> ReboxResult<()> {
//     let tbl_schema = retrieve_schema(connection, metadata, table_name)?;
//     let schema_cols = tbl_schema.get_columns();

//     let schema_column = schema_cols.get(&**column_name).ok_or(format_err!(
//         "Impossible State at {} {}",
//         file!(),
//         line!()
//     ))?;
//     if column_value != schema_column.kind() {
//         bail!("Row is not matching against {table_name} schema");
//     }

//     Ok(())
// }
