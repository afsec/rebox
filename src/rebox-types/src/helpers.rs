use anyhow::bail;

use crate::ReboxResult;

const ENTITY_NAME_MAX_CHARS: usize = 255;

pub fn check_valid_name<T: AsRef<str>>(name: T) -> ReboxResult<()> {
    let input_str = name.as_ref();
    if input_str.chars().map(|_| 1).sum::<usize>() >= ENTITY_NAME_MAX_CHARS {
        bail!("Entity name can't be larger than {ENTITY_NAME_MAX_CHARS} characters.");
    }

    input_str
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' {
                Ok(())
            } else {
                bail!("Entity name has invalid chars")
            }
        })
        .collect::<ReboxResult<()>>()?;
    Ok(())
}
