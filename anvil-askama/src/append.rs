use anvil::{Append, Forge};
use askama::Template;
use ref_cast::RefCast;

use crate::Askama;

pub trait AskamaAppendExt<'a, T: Template>: Forge {
    fn askama(template: &'a T) -> Self;
}

impl<'a, T: Template> AskamaAppendExt<'a, T> for Append<'a, Askama<T>> {
    fn askama(template: &'a T) -> Self {
        Self::new(Askama::ref_cast(template))
    }
}

pub fn append<T: Template>(template: &T) -> Append<Askama<T>> {
    Append::askama(template)
}
