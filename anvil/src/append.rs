use std::{io::BufWriter, path::Path};

use thiserror::Error;

use crate::{Anvil, Forge};

#[derive(Error, Debug)]
pub enum AppendError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
    #[error("template error")]
    Template,
}

pub struct Append<A: Anvil> {
    template: A,
}

impl<A: Anvil> Forge for Append<A> {
    type Error = AppendError;

    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(AppendError::StdIo)?;

        let mut writer = BufWriter::new(file);
        self.template
            .render_into(&mut writer)
            .map_err(|_| AppendError::Template)?;

        Ok(())
    }
}

impl<T: Anvil> Append<T> {
    pub fn new(template: T) -> Self {
        Self { template }
    }
}
