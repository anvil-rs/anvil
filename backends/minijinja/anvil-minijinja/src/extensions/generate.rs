use anvil::{generate::Generate, Forge};

use crate::{Katana, Shrine};

pub trait MinijinjaGenerateExt<'a, T: Shrine>: Forge {
    fn minijinja(template: &'a T) -> Self;
}

impl<'a, T: Shrine> MinijinjaGenerateExt<'a, T> for Generate<Katana<'a, T>> {
    fn minijinja(template: &'a T) -> Self {
        Self::new(Katana(template))
    }
}

#[inline(always)]
pub fn generate<T: Shrine>(template: &T) -> Generate<Katana<'_, T>> {
    Generate::minijinja(template)
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Write};

    use super::*;
    use anvil::Forge;
    use serde::Serialize;
    use tempfile::tempdir;

    use crate::make_minijinja_template;

    #[derive(Serialize)]
    struct TestTemplate {}

    impl Shrine for TestTemplate {
        fn minijinja(&self, writer: &mut dyn std::io::Write) -> Result<(), minijinja::Error> {
            let mut env = minijinja::Environment::new();
            env.add_template("test", "Generated content.")?;
            let tmpl = env.get_template("test")?;
            tmpl.render_to_write(self, writer)?;
            Ok(())
        }
    }

    #[test]
    fn it_fails_if_path_already_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result = generate(&TestTemplate {}).forge(&file_path);
        assert!(result.is_err());
        let file_contents = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_contents.trim(), "Initial content.");
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

    make_minijinja_template!(TestFile, "test.txt");

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
        assert_eq!(file_contents, "Hello, World!");
    }
}
