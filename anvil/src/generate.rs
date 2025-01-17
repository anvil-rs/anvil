use std::{fs::File, io::BufWriter, path::Path};

use thiserror::Error;

use crate::{Anvil, Forge};

pub struct Generate<A: Anvil> {
    template: A,
}

#[derive(Error, Debug)]
pub enum GenerateError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
    #[error("template error")]
    Template,
}

impl<A: Anvil> Forge for Generate< A> {
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

impl<T: Anvil> Generate<T> {
    pub fn new(template: T) -> Self {
        Self { template }
    }
}
