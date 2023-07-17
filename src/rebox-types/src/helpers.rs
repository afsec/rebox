use std::path::{Path, PathBuf};

use anyhow::{bail, format_err};

use crate::ReboxResult;

const ENTITY_NAME_MAX_CHARS: usize = 255;

pub fn check_valid_entity_name<T: AsRef<str>>(name: T) -> ReboxResult<()> {
    let input_str = name.as_ref();
    if input_str.chars().map(|_| 1).sum::<usize>() >= ENTITY_NAME_MAX_CHARS {
        bail!("Entity name can't be larger than {ENTITY_NAME_MAX_CHARS} characters.");
    }

    input_str.chars().try_for_each(|ch| {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            Ok(())
        } else {
            bail!("Entity name has invalid char [`{ch}`]")
        }
    })?;
    Ok(())
}

pub fn project_root() -> ReboxResult<PathBuf> {
    use std::env;
    let project_root = Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(2)
    .ok_or(format_err!("Error on getting project root path"))?
    .to_path_buf();

    Ok(project_root)
}
