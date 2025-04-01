use std::path::Path;
use std::{fs::File, io::BufWriter};

use thiserror::Error;

use crate::Anvil;
use crate::Forge;

/// A struct that can be used to generate a file from a Template.
/// The file will be created if it does not exist.
/// If the file already exists, it will be overwritten.
pub struct Generate<A: Anvil> {
    template: A,
}

#[derive(Error, Debug)]
pub enum GenerateError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
    #[error("template error {0}")]
    Template(#[from] Box<dyn std::error::Error>),
}

impl<A: Anvil> Forge for Generate<A> {
    type Error = GenerateError;

    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();

        let prefix = path.parent().expect("no parent directory");
        std::fs::create_dir_all(prefix).map_err(GenerateError::StdIo)?;

        let file = File::create_new(path).map_err(GenerateError::StdIo)?;

        let mut writer = BufWriter::new(file);

        self.template
            .anvil(&mut writer)
            .map_err(|e| GenerateError::Template(Box::new(e)))?;

        Ok(())
    }
}

impl<A: Anvil> Generate<A> {
    pub fn new(template: A) -> Self {
        Self { template }
    }
}
