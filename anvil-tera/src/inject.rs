
use anvil::{Forge, Inject};
use regex::Regex;
use serde::Serialize;
use std::borrow::Cow;
use tera::Tera;

use crate::TeraTemplate;

pub trait TeraInjectExt<T: Serialize>: Forge {
    fn tera(
        engine: Tera,
        template_path: impl Into<Cow<'static, str>>,
        context: T,
        before: Option<Regex>,
        after: Option<Regex>,
    ) -> Inject<TeraTemplate<T>>;
}

impl<T: Serialize> TeraInjectExt<T> for Inject<TeraTemplate<T>> {
    fn tera(
        engine: Tera,
        template_path: impl Into<Cow<'static, str>>,
        context: T,
        before: Option<Regex>,
        after: Option<Regex>,
    ) -> Inject<TeraTemplate<T>> {
        Inject::new(
            TeraTemplate::new(engine, template_path, context),
            before,
            after,
        )
    }
}

pub fn inject<T: Serialize>(
    engine: Tera,
    template_path: impl Into<Cow<'static, str>>,
    context: T,
    before: Option<Regex>,
    after: Option<Regex>,
) -> Inject<TeraTemplate<T>> {
    Inject::tera(engine, template_path, context, before, after)
}
