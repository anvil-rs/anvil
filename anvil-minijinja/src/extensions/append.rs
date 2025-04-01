use anvil::{append::Append, Forge};

use crate::{Katana, Shrine};

pub trait MinijinjaAppendExt<'a, T: Shrine>: Forge {
    fn minijinja(template: &'a T) -> Self;
}

impl<'a, T: Shrine> MinijinjaAppendExt<'a, T> for Append<Katana<'a, T>> {
    fn minijinja(template: &'a T) -> Self {
        Self::new(Katana(template))
    }
}

#[inline(always)]
pub fn append<T: Shrine>(template: &T) -> Append<Katana<'_, T>> {
    Append::minijinja(template)
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Write};

    use super::*;
    use anvil::Forge;
    use serde::Serialize;
    use tempfile::tempdir;

    use crate::{extensions::append::append, make_minijinja_template};

    #[derive(Serialize)]
    struct TestTemplate {}

    impl Shrine for TestTemplate {
        fn minijinja(&self, writer: &mut dyn std::io::Write) -> Result<(), minijinja::Error> {
            let mut env = minijinja::Environment::new();
            env.add_template("test", "Appended content.")?;
            let tmpl = env.get_template("test")?;
            tmpl.render_to_write(self, writer)?;
            Ok(())
        }
    }

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

    make_minijinja_template!(TestFile, "test.txt");

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
        assert_eq!(content, "Initial content.\nHello, World!");
    }
}
