use anvil::{append::Append, Forge};

use crate::{Earth, Firma};

pub trait TeraAppendExt<'a, T: Earth>: Forge {
    fn tera(template: &'a T) -> Self;
}

impl<'a, T: Earth> TeraAppendExt<'a, T> for Append<Firma<'a, T>> {
    fn tera(template: &'a T) -> Self {
        Self::new(Firma(template))
    }
}

#[inline(always)]
pub fn append<T: Earth>(template: &T) -> Append<Firma<'_, T>> {
    Append::tera(template)
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
        tera.add_raw_template("test", "Appended content.").unwrap();
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
    fn it_fails_if_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let result = append(&TestTemplate {}).forge(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn it_appends_to_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result = append(&TestTemplate {}).forge(&file_path);
        assert!(result.is_ok());
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Initial content.\nAppended content.")
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
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result = append(&TestFile {
            name: "World".to_string(),
        })
        .forge(&file_path);
        assert!(result.is_ok());
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content.trim(), "Initial content.\nHello, World!");
    }
}
