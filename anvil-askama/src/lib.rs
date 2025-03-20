use anvil::Anvil;
use askama::Template;

pub mod filters;

pub mod extensions;

pub use extensions::append::{append, AskamaAppendExt};
pub use extensions::generate::{generate, AskamaGenerateExt};

pub struct Askama<'a, T: Template>(&'a T);

impl<T: Template> Anvil for Askama<'_, T> {
    type Error = std::io::Error;

    fn anvil(&self, writer: &mut (impl std::io::Write + ?Sized)) -> Result<(), std::io::Error> {
        self.0.write_into(writer)
    }
}
