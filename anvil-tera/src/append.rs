use anvil::{Append, Forge};
use serde::Serialize;
use std::borrow::Cow;
use tera::Tera;

use crate::TeraTemplate;

pub trait TeraAppendExt<T: Serialize>: Forge {
    fn tera(
        engine: Tera,
        template_path: impl Into<Cow<'static, str>>,
        context: T,
    ) -> Append<TeraTemplate<T>>;
}

impl<T: Serialize> TeraAppendExt<T> for Append<TeraTemplate<T>> {
    fn tera(
        engine: Tera,
        template_path: impl Into<Cow<'static, str>>,
        context: T,
    ) -> Append<TeraTemplate<T>> {
        Append::new(TeraTemplate::new(engine, template_path, context))
    }
}

pub fn append<T: Serialize>(
    engine: Tera,
    template_path: impl Into<Cow<'static, str>>,
    context: T,
) -> Append<TeraTemplate<T>> {
    Append::tera(engine, template_path, context)
}
