use anvil::Anvil;
use askama::Template;

pub mod filters;

pub mod extensions;

pub struct Askama<'a, T: Template>(&'a T);

impl<T: Template> Anvil for Askama<'_, T> {
    type Error = std::io::Error;

    fn anvil(&self, writer: &mut (impl std::io::Write + ?Sized)) -> Result<(), std::io::Error> {
        Template::write_into(&self.0, writer)
    }
}

pub mod prelude {
    pub use crate::extensions::{
        append::{append, AskamaAppendExt},
        generate::{generate, AskamaGenerateExt},
    };
    pub use crate::filters::*;
    pub use crate::Askama;
}
