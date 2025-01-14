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

pub struct Append<'a, A: Anvil> {
    template: &'a A,
}

impl<A: Anvil> Forge for Append<'_, A> {
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

impl<'a, T: Anvil> Append<'a, T> {
    pub fn new(template: &'a T) -> Self {
        Self { template }
    }
}
