use anvil::{Forge, Inject};
use askama::Template;
use regex::Regex;

use crate::Askama;

pub trait AskamaInjectExt<T: Template + Clone>: Forge {
    fn askama(template: T, before: Option<Regex>, after: Option<Regex>) -> Self;
}

impl<T: Template + Clone> AskamaInjectExt<T> for Inject<Askama<T>> {
    fn askama(template: T, before: Option<Regex>, after: Option<Regex>) -> Self {
        Self::new(Askama(template), before, after)
    }
}

pub fn inject<T: Template + Clone>(template: &T, before: Regex, after: Regex) -> Inject<Askama<T>> {
    Inject::askama(template.clone(), Some(before), Some(after))
}

pub fn inject_before<T: Template + Clone>(template: &T, before: Regex) -> Inject<Askama<T>> {
    Inject::askama(template.clone(), Some(before), None)
}

pub fn inject_after<T: Template + Clone>(template: &T, after: Regex) -> Inject<Askama<T>> {
    Inject::askama(template.clone(), None, Some(after))
}
