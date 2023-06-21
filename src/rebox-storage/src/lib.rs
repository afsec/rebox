mod drivers;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::drivers::Driver;
    use rebox_types::ReboxResult;

    #[test]
    fn run_against_memory_driver() -> ReboxResult<()> {
        let driver_name = "memory";
        let driver = Driver::from_str(driver_name)?;
        assert!(driver.run().is_ok());
        Ok(())
    }
    #[test]
    fn run_against_kv_driver() -> ReboxResult<()> {
        let driver_name = "kv";
        let driver = Driver::from_str(driver_name)?;

        assert!(driver.run().is_ok());
        Ok(())
    }
    #[test]
    fn run_against_invalid_input_for_driver() -> ReboxResult<()> {
        let driver_name = "";
        let driver = Driver::from_str(driver_name);
        assert!(driver.is_err());
        Ok(())
    }
}
