pub mod append;
pub mod either;
pub mod filters;
pub mod generate;
pub mod inject;
pub mod render;

use std::{error::Error, path::Path};

/// Anvil is the base trait for all rendering engines.
/// It provides the basic functionality to render a template into a string or write it to a file.
/// The error is the error that the rendering engine can produce.
pub trait Anvil {
    type Error: Error;
    fn render(&self, into: impl AsRef<Path>) -> Result<(), Self::Error>;
}

impl<T: Template> Anvil for T {
    type Error = askama::Error;

    fn render(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let prefix = path.parent().expect("no parent directory");
        std::fs::create_dir_all(prefix).unwrap();
        let file = std::fs::File::create(path).unwrap();
        let mut writer = std::io::BufWriter::new(file);
        self.write_into(&mut writer).unwrap();
        Ok(())
    }
}

pub use append::Append;
use askama::Template;
pub use either::Either;
pub use generate::Generate;
pub use inject::Inject;
