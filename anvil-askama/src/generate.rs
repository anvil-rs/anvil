use anvil::{Forge, Generate};
use askama::Template;
use ref_cast::RefCast;

use crate::Askama;

pub trait AskamaGenerateExt<'a, T: Template>: Forge {
    fn askama(template: &'a T) -> Self;
}

impl<'a, T: Template> AskamaGenerateExt<'a, T> for Generate<'a, Askama<T>> {
    fn askama(template: &'a T) -> Self {
        Self::new(Askama::ref_cast(template))
    }
}

pub fn generate<T: Template>(template: &T) -> Generate<Askama<T>> {
    Generate::askama(template)
}
