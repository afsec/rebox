mod drivers;

#[cfg(test)]
mod tests {
    use super::drivers::{key_value::DriverRkv, memory::DriverMemory};
    use rebox_types::ReboxResult;

    #[test]
    fn it_works() -> ReboxResult<()> {
        DriverMemory::run()?;
        DriverRkv::run()?;
        Ok(())
    }
}
