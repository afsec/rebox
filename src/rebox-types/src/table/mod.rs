use crate::ReboxResult;

#[derive(Debug, Default)]
pub struct Database<D: Driver>(DatabaseState<D>);

impl<D: Driver> Database<D> {
    pub fn run(self) -> ReboxResult<()> {
        println!("Hello from: {} at line {}.", file!(), line!());
        Ok(())
    }
}

pub trait Driver {}

#[derive(Debug, Default)]
pub struct DatabaseOps<D: Driver> {
    driver: D,
}

#[derive(Debug, Default)]
pub enum DatabaseState<D: Driver> {
    #[default]
    NotStarted,
    Opening(D),
    Ready(Ready<D>),
}

#[derive(Debug, Default)]
pub struct Ready<D: Driver> {
    driver: D,
    master_table: MasterTable,
    tables: Vec<Table>,
}

#[derive(Debug, Default)]
pub struct TableName;

#[derive(Debug, Default)]
pub struct MasterTable(Vec<TableName>);

#[derive(Debug, Default)]
pub struct Table {}

#[derive(Debug, Default)]
pub struct Void;
