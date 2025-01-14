use std::{fs::File, io::BufWriter, path::Path};

use thiserror::Error;

use crate::{Anvil, Forge};

pub struct Generate<'a, A: Anvil> {
    template: &'a A,
}

#[derive(Error, Debug)]
pub enum GenerateError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
    #[error("template error")]
    Template,
}

impl<A: Anvil> Forge for Generate<'_, A> {
    type Error = GenerateError;

    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();

        let prefix = path.parent().expect("no parent directory");
        std::fs::create_dir_all(prefix).map_err(GenerateError::StdIo)?;

        let file = File::create(path).map_err(GenerateError::StdIo)?;

        let mut writer = BufWriter::new(file);

        self.template
            .render_into(&mut writer)
            .map_err(|_| GenerateError::Template)?;

        Ok(())
    }
}

impl<'a, T: Anvil> Generate<'a, T> {
    pub fn new(template: &'a T) -> Self {
        Self { template }
    }
}
