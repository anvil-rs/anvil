use anvil::{Forge, Generate};
use serde::Serialize;
use std::borrow::Cow;
use tera::Tera;

use crate::TeraTemplate;

pub trait TeraGenerateExt<T: Serialize>: Forge {
    fn tera(
        engine: Tera,
        template_path: impl Into<Cow<'static, str>>,
        context: T,
    ) -> Generate<TeraTemplate<T>>;
}

impl<T: Serialize> TeraGenerateExt<T> for Generate<TeraTemplate<T>> { 
    fn tera(
        engine: Tera,
        template_path: impl Into<Cow<'static, str>>,
        context: T,
    ) -> Generate<TeraTemplate<T>> {
        Generate::new(TeraTemplate::new(engine, template_path, context))
    }
}

pub fn generate<T: Serialize>(
    engine: Tera,
    template_path: impl Into<Cow<'static, str>>,
    context: T,
) -> Generate<TeraTemplate<T>> {
    Generate::tera(engine, template_path, context)
}

#[cfg(test)]
mod test {

    use super::*;
    use anvil::Anvil;

    #[derive(Serialize)]
    struct Simple {
        name: String,
    }
    
    #[test]
    fn test_tera() {
        let mut tera = Tera::new("templates/**/*").unwrap();
        tera.add_raw_template("hello", "Hello, {{ name }}!").unwrap();
        let context = Simple { name: "world".to_string() };
        let tera_test = TeraTemplate::new(tera, "hello", context);

        let mut buffer = Vec::new();
        tera_test.render_into(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), "Hello, world!");
    }
}
