use std::{io::BufWriter, path::Path};
use thiserror::Error;

use crate::{Anvil, Forge};

#[derive(Error, Debug)]
pub enum AppendError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
    #[error("templating error")]
    // TODO: Store box dyn error (or option of box dyn error)
    Template,
}

/// A struct that can be used to append a Template to a file.
/// The file will NOT be created if it does not exist.
pub struct Append<A: Anvil> {
    template: A,
}

impl<A: Anvil> Forge for Append<A> {
    type Error = AppendError;

    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let file = std::fs::OpenOptions::new()
            .append(true)
            .open(path)
            .map_err(AppendError::StdIo)?;

        let mut writer = BufWriter::new(file);

        self.template
            .anvil(&mut writer)
            .map_err(|_e| AppendError::Template)?;

        Ok(())
    }
}

impl<A: Anvil> Append<A> {
    pub fn new(template: A) -> Self {
        Self { template }
    }
}
