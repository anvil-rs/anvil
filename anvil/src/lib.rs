pub mod append;
pub mod either;
pub mod generate;
pub mod inject;
pub mod remove;

pub use append::Append;
pub use either::Either;
pub use generate::Generate;
pub use inject::Inject;

use std::error::Error;
use std::path::Path;

/// Anvil is the base trait for all rendering engines.
/// It provides the basic functionality to render a template into a string or write it to a file.
/// The error is the error that the rendering engine can produce.
pub trait Anvil {
    type Error: Error;
    fn render_into(&self, writer: &mut (impl std::io::Write + ?Sized)) -> Result<(), Self::Error>;

    fn render(&self) -> Result<String, Self::Error> {
        let mut buffer = Vec::new();
        self.render_into(&mut buffer)?;
        Ok(String::from_utf8(buffer).unwrap())
    }
}

pub trait Forge {
    type Error: Error;
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error>;
}

pub fn forge(forge: impl Forge, into: impl AsRef<Path>) {
    forge.forge(into).unwrap();
}
