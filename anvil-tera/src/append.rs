use anvil::{Forge, Generate};
use tera::Tera;
use serde::Serialize;
use crate::AnvilTera;

// pub trait AnvilTeraGenerateExt<'a, T: Serialize>: Forge {
//     fn tera(template: &'a T) -> Self;
// }
//
// impl<'a, T: Serialize> AnvilTeraGenerateExt<'a, T> for Generate<'a, AnvilTera<T>> {
//     fn tera(template: &'a T, tera: &'a Tera, template: &'static str) -> Self {
//         Self::new(AnvilTera {
//             inner: template,
//             tera: tera,
//             template: template,
//         })
//     }
// }
//
// #[cfg(test)]
// mod test {
//
//     use super::*;
//     use std::collections::HashMap;
//
//     #[test]
//     fn test_tera() {
//         let mut tera = Tera::default();
//         tera.add_raw_template("hello", "Hello, {{ name }}!").unwrap();
//         let mut context = HashMap::new();
//         context.insert("name", "world");
//         let forge = Generate::tera(&context, &tera, "hello");
//         let mut buffer = Vec::new();
//         forge.render_into(&mut buffer).unwrap();
//         assert_eq!(String::from_utf8(buffer).unwrap(), "Hello, world!");
//     }
//
// }
