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

/// Macro to generate the Earth trait implementation for a struct.
/// This macro takes in the name of the struct, the name of a template, and a reference to a global, lazy locked tera instance.
/// It then creates a context from self (because earth implies serialize and therefore we can create context from serialize) and then render_to template, with context.
/// # Example
/// ```
/// use tera::Tera;
/// use std::sync::LazyLock;
/// use serde::Serialize;
/// use anvil::Anvil;
/// use anvil_tera::{make_tera_template, prelude::*};
///
/// static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
///     let mut tera = Tera::default();
///     tera.add_raw_template("test", "Appended content.").unwrap();
///     tera
/// });
///
/// #[derive(Serialize)]
/// struct TestEarth { name: String }
///
/// make_tera_template!(TestEarth, "test.html", TEMPLATES);
/// ```
#[macro_export]
macro_rules! make_tera_template {
    ($struct:ident, $template:expr, $tera:expr) => {
        impl Earth for $struct {
            fn tera(&self, writer: &mut (impl std::io::Write + ?Sized)) -> tera::Result<()> {
                let context = tera::Context::from_serialize(self).unwrap();
                $tera.render_to($template, &context, writer)
            }
        }
    };
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

    use super::prelude::*;
    use super::*;
    use crate::make_tera_template;
    use serde::Serialize;
    use tera::Tera;

    #[derive(Serialize)]
    struct TestEarth {
        name: String,
    }

    // TODO: Make this a derive macro
    make_tera_template!(TestEarth, "test", TEMPLATES);

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
