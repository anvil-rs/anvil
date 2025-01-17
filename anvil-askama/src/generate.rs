use anvil::{Forge, Generate};
use askama::Template;

use crate::Askama;

pub trait AskamaGenerateExt<T: Template>: Forge {
    fn askama(template: T) -> Self;
}

impl<T: Template> AskamaGenerateExt<T> for Generate<Askama<T>> {
    fn askama(template: T) -> Self {
        Self::new(Askama(template))
    }
}

pub fn generate<T: Template>(template: T) -> Generate<Askama<T>> {
    Generate::askama(template)
}
