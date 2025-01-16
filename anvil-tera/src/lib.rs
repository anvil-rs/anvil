use anvil::Anvil;
use serde::Serialize;
use tera::Context;
use tera::Tera;

pub struct AnvilTera<'a, T: Serialize> {
    inner: T,
    tera: &'a Tera,
    template: &'static str,
}

impl<T: Serialize> Anvil for AnvilTera<'_, T> {
    type Error = tera::Error;

    fn render_into(&self, writer: &mut (impl std::io::Write + ?Sized)) -> Result<(), Self::Error> {
        let context = Context::from_serialize(&self.inner)?;
        self.tera.render_to(self.template, &context, writer)
    }
}
