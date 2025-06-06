use std::io::Write;

use anvil::Anvil;
use serde::Serialize;

pub mod extensions;

pub trait Water: Serialize {
    fn liquid(&self, writer: &mut dyn Write) -> Result<(), liquid::Error>;
}

pub struct Aqua<'a, T: Water>(&'a T);

impl<T: Water> Anvil for Aqua<'_, T> {
    type Error = liquid::Error;

    fn anvil(&self, writer: &mut (impl std::io::Write + Sized)) -> Result<(), Self::Error> {
        self.0.liquid(writer)
    }
}

pub mod prelude {
    pub use crate::extensions::{
        append::{append, LiquidAppendExt},
        generate::{generate, LiquidGenerateExt},
    };

    pub use crate::Water;
}

#[cfg(test)]
mod test {
    use crate::{Aqua, Water};
    use anvil::Anvil;
    use liquid::ParserBuilder;
    use serde::Serialize;
    use std::sync::LazyLock;

    static PARSER: LazyLock<liquid::Parser> =
        LazyLock::new(|| ParserBuilder::with_stdlib().build().unwrap());

    #[derive(Serialize)]
    struct TestTemplate {
        name: String,
    }

    impl Water for TestTemplate {
        fn liquid(&self, writer: &mut dyn std::io::Write) -> Result<(), liquid::Error> {
            let object = liquid::to_object(self)?;
            let template = PARSER.parse("Hello, {{ name }}!")?;
            template.render_to(writer, &object)
        }
    }

    #[test]
    fn it_can_render_template() {
        let template = TestTemplate {
            name: "World".to_string(),
        };

        let mut buf = Vec::new();
        let aqua = Aqua(&template);
        aqua.anvil(&mut buf).unwrap();
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(result, "Hello, World!");
    }
}
