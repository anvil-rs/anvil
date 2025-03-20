use anvil::{append::Append, Forge};
use askama::Template;

use crate::Askama;

pub trait AskamaAppendExt<'a, T: Template>: Forge {
    fn askama(template: &'a T) -> Self;
}

impl<'a, T: Template> AskamaAppendExt<'a, T> for Append<Askama<'a, T>> {
    fn askama(template: &'a T) -> Self {
        Self::new(Askama(template)) // Convert Box<T> into &'static T (safe due to 'a lifetime)
    }
}

#[inline(always)]
pub fn append<T: Template>(template: &T) -> Append<Askama<T>> {
    Append::askama(template)
}
