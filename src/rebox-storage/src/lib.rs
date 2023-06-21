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
        driver.run()?;
        Ok(())
    }
    fn run_against_kv_driver() -> ReboxResult<()> {
        let driver_name = "kv";
        let driver = Driver::from_str(driver_name)?;
        driver.run()?;
        Ok(())
    }
}
