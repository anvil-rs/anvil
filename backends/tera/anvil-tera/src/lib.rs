use anvil::Anvil;
use serde::Serialize;

pub mod extensions;

// General newtype wrapper for tera context to allow user-implementations of the trait.
// pub struct Earth<'a, T: Serialize>(&'a T);
pub trait Earth: Serialize {
    fn tera(&self, writer: &mut (impl std::io::Write + ?Sized)) -> tera::Result<()>;
}

pub struct Firma<'a, T: Earth>(&'a T);

impl<T: Earth> Anvil for Firma<'_, T> {
    type Error = tera::Error;
    fn anvil(&self, writer: &mut (impl std::io::Write + ?Sized)) -> Result<(), Self::Error> {
        self.0.tera(writer)
    }
}

pub mod prelude {
    pub use crate::extensions::{
        append::{append, TeraAppendExt},
        generate::{generate, TeraGenerateExt},
    };
    pub use crate::Earth;
}

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use crate::Firma;

    static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
        let mut tera = Tera::default();
        tera.add_raw_template("test", "Hello, {{ name }}!\n")
            .unwrap();
        tera
    });

    use super::*;
    use serde::Serialize;
    use tera::Tera;

    #[derive(Serialize)]
    struct TestEarth {
        name: String,
    }

    impl Earth for TestEarth {
        fn tera(&self, writer: &mut (impl std::io::Write + ?Sized)) -> tera::Result<()> {
            let context = ::tera::Context::from_serialize(self)?;
            // Use the extracted tera instance expression
            TEMPLATES.render_to("test", &context, writer)
        }
    }

    #[test]
    fn it_can_render_template() {
        let earth = TestEarth {
            name: "World".to_string(),
        };

        let mut buf = Vec::new();

        let firma = Firma(&earth);

        firma.anvil(&mut buf).unwrap();

        let result = String::from_utf8(buf).unwrap();

        assert_eq!(result, "Hello, World!\n");
    }
}
