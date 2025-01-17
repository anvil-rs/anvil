pub mod append;
pub mod filters;
pub mod generate;
pub mod inject;

use anvil::Anvil;
use askama::Template;

pub struct Askama<T: Template>(T);

impl<T: Template> Anvil for Askama<T> {
    type Error = std::io::Error;

    fn render_into(&self, writer: &mut (impl std::io::Write + ?Sized)) -> Result<(), Self::Error> {
        self.0.write_into(writer)
    }
}
