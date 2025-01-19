use std::{io::BufWriter, path::Path};

use askama::Template;
use thiserror::Error;

use crate::Anvil;

#[derive(Error, Debug)]
pub enum AppendError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
}

/// A struct that can be used to append a Template to a file.
/// The file will NOT be created if it does not exist.
pub struct Append<'a, T>
where
    T: Template,
{
    template: &'a T,
}

impl<T> Anvil for Append<'_, T>
where
    T: Template,
{
    type Error = AppendError;

    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let file = std::fs::OpenOptions::new()
            .append(true)
            .open(path)
            .map_err(AppendError::StdIo)?;

        let mut writer = BufWriter::new(file);

        self.template
            .write_into(&mut writer)
            .map_err(AppendError::StdIo)?;

        Ok(())
    }
}

impl<'a, T> Append<'a, T>
where
    T: Template,
{
    pub fn new(template: &'a T) -> Self {
        Self { template }
    }
}

pub fn append<T: Template>(template: &T) -> Append<T> {
    Append::new(template)
}
