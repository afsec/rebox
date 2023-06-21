use anyhow::format_err;
use std::env;
use std::path::{Path, PathBuf};

pub type XtaskResult<T> = anyhow::Result<T>;

pub fn project_root() -> XtaskResult<PathBuf> {
    let project_root = Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .ok_or(format_err!("Error on getting project root path"))?
    .to_path_buf();

    Ok(project_root)
}
