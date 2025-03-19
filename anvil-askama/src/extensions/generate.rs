use anvil::{generate::Generate, Forge};
use askama::Template;

use crate::Askama;

pub trait AskamaGenerateExt<'a, T: Template>: Forge {
    fn askama(template: &'a T) -> Self;
}

impl<'a, T: Template> AskamaGenerateExt<'a, T> for Generate<Askama<'a, T>> {
    fn askama(template: &'a T) -> Self {
        Self::new(Askama(template)) // Convert Box<T> into &'static T (safe due to 'a lifetime)
    }
}

pub fn append<T: Template>(template: &T) -> Generate<Askama<T>> {
    Generate::askama(template)
}
