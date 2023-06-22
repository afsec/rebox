pub type ReboxResult<T> = anyhow::Result<T>;

pub mod table;

#[cfg(test)]
mod tests;
