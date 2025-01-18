pub mod append;
pub mod generate;
pub mod inject;

use anvil::Anvil;
use serde::Serialize;
use std::borrow::Cow;
use tera::Tera;

pub struct TeraTemplate<T: Serialize> {
    engine: Tera,
    template_path: Cow<'static, str>,
    context: T,
}

impl<T: Serialize> TeraTemplate<T> {
    pub fn new(engine: Tera, template_path: impl Into<Cow<'static, str>>, context: T) -> Self {
        Self {
            engine,
            template_path: template_path.into(),
            context,
        }
    }
}

impl<T: Serialize> Anvil for TeraTemplate<T> {
    type Error = tera::Error;

    fn render_into(&self, writer: &mut (impl std::io::Write + ?Sized)) -> Result<(), Self::Error> {
        let ctx = tera::Context::from_serialize(&self.context)?;
        self.engine.render_to(&self.template_path, &ctx, writer)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Serialize)]
    struct HelloTemplate {
        name: String,
    }

    #[test]
    fn can_render_raw_tera_template() {
        let mut tera = Tera::default();

        let context = HelloTemplate {
            name: "World".to_string(),
        };

        tera.add_raw_template("hello", "Hello, {{ name }}!")
            .unwrap();
        let template = TeraTemplate::new(tera, "hello", context);
        let mut buf = Vec::new();
        template.render_into(&mut buf).unwrap();
        assert_eq!(buf, b"Hello, World!");
    }

    #[derive(Serialize)]
    struct Simple {
        name: String,
    }

    #[test]
    fn can_render_tera_template_from_file() {
        let tera = Tera::new("templates/**/*").unwrap();
        let context = Simple {
            name: "world".to_string(),
        };
        let tera_test = TeraTemplate::new(tera, "test.t", context);

        let mut buffer = Vec::new();
        tera_test.render_into(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), "Hello, world!\n");
    }
}
