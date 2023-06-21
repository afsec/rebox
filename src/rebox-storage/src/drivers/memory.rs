use rebox_types::ReboxResult;

#[derive(Debug, Default)]
pub struct DriverMemory;

impl DriverMemory {
    pub fn run(self) -> ReboxResult<()> {
        println!("Hello from: {} at line {}.", file!(), line!());
        Ok(())
    }
}
