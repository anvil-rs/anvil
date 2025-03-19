use std::{error::Error, path::Path};

use thiserror::Error;

use crate::Forge;

pub type BoxedError = Box<dyn Error + Send + Sync>;

pub struct Transform {
    transformer: Box<dyn Fn(String) -> Result<String, BoxedError>>,
}

impl Transform {
    pub fn new<F>(transformer: F) -> Self
    where
        F: Fn(String) -> Result<String, BoxedError> + 'static,
    {
        Self {
            transformer: Box::new(transformer),
        }
    }

    pub fn apply(&self, input: &str) -> Result<String, BoxedError> {
        (self.transformer)(input.to_string())
    }
}

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
    #[error("transform error {0}")]
    Transform(#[from] BoxedError),
}

impl Forge for Transform {
    type Error = TransformError;
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let content = std::fs::read_to_string(path).map_err(TransformError::StdIo)?;
        let transformed = self.apply(&content).map_err(TransformError::Transform)?;
        std::fs::write(path, transformed).map_err(TransformError::StdIo)?;
        Ok(())
    }
}
