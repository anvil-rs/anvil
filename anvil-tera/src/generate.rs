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
