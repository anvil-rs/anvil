use std::{fs::File, io::BufWriter, path::Path};

use askama::Template;
use thiserror::Error;

use crate::Anvil;

pub struct Generate<'a, T>
where
    T: Template,
{
    template: &'a T,
}

#[derive(Error, Debug)]
pub enum GenerateError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
}

impl<T> Anvil for Generate<'_, T>
where
    T: Template,
{
    type Error = GenerateError;

    fn render(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();

        let prefix = path.parent().expect("no parent directory");
        std::fs::create_dir_all(prefix).map_err(GenerateError::StdIo)?;

        let file = File::create(path).map_err(GenerateError::StdIo)?;

        let mut writer = BufWriter::new(file);

        self.template
            .write_into(&mut writer)
            .map_err(GenerateError::StdIo)?;

        Ok(())
    }
}

impl<'a, T: Template> Generate<'a, T> {
    pub fn new(template: &'a T) -> Self {
        Self { template }
    }
}

#[macro_export]
macro_rules! generate {
    ($template:expr) => {
        Generate::new($template)
    };
}
