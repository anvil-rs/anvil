pub mod generate;

use anvil::Anvil;
use serde::Serialize;
use tera::Tera;
use std::borrow::Cow;

pub struct TeraTemplate<T: Serialize> {
    engine: Tera,
    template_path: Cow<'static, str>,
    context: T,
}

impl<T: Serialize> TeraTemplate<T> {
    pub fn new(
        engine: Tera,
        template_path: impl Into<Cow<'static, str>>,
        context: T,
    ) -> Self {
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

