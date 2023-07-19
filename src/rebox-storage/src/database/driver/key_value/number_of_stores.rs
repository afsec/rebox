use super::KeyValueDriver;
use anyhow::format_err;
use rebox_types::ReboxResult;

pub(super) struct NumberOfStores<'a>(&'a KeyValueDriver);
impl<'a> NumberOfStores<'a> {
    pub(super) fn connect(driver: &'a KeyValueDriver) -> ReboxResult<Self> {
        Ok(Self(driver))
    }

    pub(super) fn len(self) -> ReboxResult<u16> {
        let created_arc = self.0.connection();
        let rkv_env = created_arc
            .read()
            .map_err(|err| format_err!("Read error: {err}"))?;
        let dbs = rkv_env.get_dbs()?;
        Ok(dbs.len().try_into()?)
    }
}
