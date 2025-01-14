use std::{io::BufWriter, path::Path};

use askama::Template;
use thiserror::Error;

use crate::Anvil;

#[derive(Error, Debug)]
pub enum AppendError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
}

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

    fn render(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let file = std::fs::OpenOptions::new()
            .create(true)
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

#[macro_export]
macro_rules! append {
    ($template:expr) => {
        Append::new($template)
    };
}
