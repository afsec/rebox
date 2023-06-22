use table::Driver;

pub type ReboxResult<T> = anyhow::Result<T>;

pub mod table;
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

#[derive(Debug, Default)]
struct KeyValue;

#[derive(Debug, Default)]
struct Memory;

impl Driver for Memory {}

#[cfg(test)]
mod tests {
    use super::*;
    use table::Database;

    #[test]
    fn it_works() -> ReboxResult<()> {
        let db = Database::new()
            .set_driver(Memory)?
            .set_session_name("remora-123123123")?
            .connect()?;

        // assert_eq!(result, 4);
        Ok(())
    }
}
