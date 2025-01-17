use anvil::{Append, Forge};
use askama::Template;

use crate::Askama;

pub trait AskamaAppendExt<T: Template>: Forge {
    fn askama(template: T) -> Self;
}

impl<T: Template> AskamaAppendExt<T> for Append<Askama<T>> {
    fn askama(template: T) -> Self {
        Self::new(Askama(template))
    }
}

pub fn append<T: Template>(template: T) -> Append<Askama<T>> {
    Append::askama(template)
}
