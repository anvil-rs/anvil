use anvil::{generate::Generate, Forge};

use crate::{Earth, Firma};

pub trait TeraGenerateExt<'a, T: Earth>: Forge {
    fn tera(template: &'a T) -> Self;
}

impl<'a, T: Earth> TeraGenerateExt<'a, T> for Generate<Firma<'a, T>> {
    fn tera(template: &'a T) -> Self {
        Self::new(Firma(template))
    }
}

#[inline(always)]
pub fn generate<T: Earth>(template: &T) -> Generate<Firma<'_, T>> {
    Generate::tera(template)
}

#[cfg(test)]
mod test {
    static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.add_raw_template("test", "Generated content.").unwrap();
        tera
    });

    use super::*;
    use crate::make_tera_template;
    use serde::Serialize;
    use std::{fs::File, io::Write, sync::LazyLock};
    use tempfile::tempdir;
    use tera::Tera;

    // NOTE: This template needs the dummy braces to be recognized by Tera because a completely
    // empty struct us not parseable json.
    #[derive(Serialize)]
    struct TestTemplate {}

    make_tera_template!(TestTemplate, "test", TEMPLATES);

    #[test]
    fn it_fails_if_path_already_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result = generate(&TestTemplate {}).forge(&file_path);
        assert!(result.is_err());
        let file_contents = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_contents, "Initial content.\n");
    }

    #[test]
    fn it_generates_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let result = generate(&TestTemplate {}).forge(&file_path);
        assert!(result.is_ok());
        let file_contents = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_contents, "Generated content.");
    }

    #[derive(Serialize)]
    struct TestFile {
        name: String,
    }

    make_tera_template!(TestFile, "test.txt", TEMPLATES);

    #[test]
    fn it_can_render_from_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let result = generate(&TestFile {
            name: "World".to_string(),
        })
        .forge(&file_path);
        assert!(result.is_ok());
        let file_contents = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_contents.trim(), "Hello, World!");
    }
}
