use std::sync::LazyLock;

use anvil::Anvil;
use serde::Serialize;
use tera::Tera;

pub mod extensions;

pub static TEMPLATE: LazyLock<Tera> =
    LazyLock::new(|| match Tera::new("examples/basic/templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    });

// pub struct Tera<'a, T: Serialize>(&'a T);

// impl NewTrait for CustomType such taht CustomType implements Serialize such that re can render stuff quickly
// impl Anvil for Tera<', T> where T: NewTrait {
// ...
// just render
// }

// create a proc-macro to generate this given some type.
// impl<T: Serialize> Anvil for Tera<'_, T> {
//     type Error = std::io::Error;
//
//     fn anvil(&self, writer: &mut (impl std::io::Write + ?Sized)) -> Result<(), std::io::Error> {
//         let mut tera = tera::Tera::default();
//         tera.add_raw_template("template", include_str!("template.html"))
//             .unwrap();
//         let context = tera::Context::from_serialize(&self.0).unwrap();
//         let rendered = tera.render("template", &context).unwrap();
//         writer.write_all(rendered.as_bytes())?;
//         Ok(())
//     }
// }

pub mod prelude {}
