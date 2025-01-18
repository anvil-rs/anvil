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

#[cfg(test)]
mod test {
    use super::*;
    use askama::Template;

    #[derive(Template)]
    #[template(path = "hello.txt")]
    struct HelloTemplate {
        name: String,
    }

    #[test]
    fn can_render_askama_template() {
        let template = HelloTemplate {
            name: "World".to_string(),
        };

        let anvil = Askama(template);

        let mut buffer = Vec::new();
        anvil.render_into(&mut buffer).unwrap();
        assert_eq!(String::from_utf8(buffer).unwrap(), "Hello, World!");
    }
}
