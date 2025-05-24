use std::io::Write;

use anvil::Anvil;
use serde::Serialize;

pub mod extensions;

pub trait Shrine: Serialize {
    fn minijinja(&self, writer: &mut dyn Write) -> Result<(), minijinja::Error>;
}

pub struct Katana<'a, T: Shrine>(&'a T);

impl<T: Shrine> Anvil for Katana<'_, T> {
    type Error = minijinja::Error;
    fn anvil(&self, writer: &mut (impl std::io::Write + Sized)) -> Result<(), Self::Error> {
        self.0.minijinja(writer)
    }
}

pub mod prelude {
    pub use crate::extensions::{
        append::{append, MinijinjaAppendExt},
        generate::{generate, MinijinjaGenerateExt},
    };
    pub use crate::Shrine;
}

#[macro_export]
macro_rules! make_minijinja_template {
    ($struct:ident, $template:expr) => {
        impl Shrine for $struct {
            fn minijinja(&self, writer: &mut dyn Write) -> Result<(), minijinja::Error> {
                let mut env = minijinja::Environment::new();
                minijinja_embed::load_templates!(&mut env);
                let tmpl = env.get_template($template)?;
                tmpl.render_to_write(self, writer)?;
                Ok(())
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    #[derive(Serialize)]
    struct TestTemplate {}

    impl Shrine for TestTemplate {
        fn minijinja(&self, writer: &mut dyn Write) -> Result<(), minijinja::Error> {
            let mut env = minijinja::Environment::new();
            env.add_template("test", "Hello, World!")?;
            let tmpl = env.get_template("test")?;
            tmpl.render_to_write(self, writer)?;
            Ok(())
        }
    }

    #[test]
    fn it_renders_template() {
        let template = TestTemplate {};
        let mut buf = Vec::new();
        let aqua = Katana(&template);
        aqua.anvil(&mut buf).unwrap();
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(result, "Hello, World!");
    }
}
