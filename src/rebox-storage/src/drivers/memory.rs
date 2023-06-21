use rebox_types::ReboxResult;

#[derive(Debug, Default)]
pub struct DriverMemory {
    master_table: MasterTable,
    tables: Vec<Table>,
}

#[derive(Debug, Default)]
pub struct TableName;

#[derive(Debug, Default)]
pub struct MasterTable(Vec<TableName>);

#[derive(Debug, Default)]
pub struct Table {}

impl DriverMemory {
    pub fn run(self) -> ReboxResult<()> {
        println!("Hello from: {} at line {}.", file!(), line!());
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Void;
