use anvil::{Forge, Inject};
use askama::Template;
use ref_cast::RefCast;
use regex::Regex;

use crate::Askama;

pub trait AskamaInjectExt<'a, T: Template>: Forge {
    fn askama(template: &'a T, before: Option<Regex>, after: Option<Regex>) -> Self;
}

impl<'a, T: Template> AskamaInjectExt<'a, T> for Inject<'a, Askama<T>> {
    fn askama(template: &'a T, before: Option<Regex>, after: Option<Regex>) -> Self {
        Self::new(Askama::ref_cast(template), before, after)
    }
}

pub fn inject<T: Template>(template: &T, before: Regex, after: Regex) -> Inject<Askama<T>> {
    Inject::askama(template, Some(before), Some(after))
}

pub fn inject_before<T: Template>(template: &T, before: Regex) -> Inject<Askama<T>> {
    Inject::askama(template, Some(before), None)
}

pub fn inject_after<T: Template>(template: &T, after: Regex) -> Inject<Askama<T>> {
    Inject::askama(template, None, Some(after))
}
