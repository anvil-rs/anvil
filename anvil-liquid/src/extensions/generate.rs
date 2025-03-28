use anvil::{generate::Generate, Forge};

use crate::{Aqua, Water};

pub trait TeraGenerateExt<'a, T: Water>: Forge {
    fn tera(template: &'a T) -> Self;
}

impl<'a, T: Water> TeraGenerateExt<'a, T> for Generate<Aqua<'a, T>> {
    fn tera(template: &'a T) -> Self {
        Self::new(Aqua(template))
    }
}

#[inline(always)]
pub fn generate<T: Water>(template: &T) -> Generate<Aqua<'_, T>> {
    Generate::tera(template)
}

#[cfg(test)]
mod test {
    use crate::make_liquid_template;

    use super::*;
    use liquid::ParserBuilder;
    use serde::Serialize;
    use std::{fs::File, io::Write, sync::LazyLock};
    use tempfile::tempdir;

    static PARSER: LazyLock<liquid::Parser> =
        LazyLock::new(|| ParserBuilder::with_stdlib().build().unwrap());

    #[derive(Serialize)]
    struct TestTemplate {}

    impl Water for TestTemplate {
        fn liquid(&self, writer: &mut dyn std::io::Write) -> Result<(), liquid::Error> {
            let object = liquid::to_object(self)?;
            let template = PARSER.parse("Generated content.")?;
            template.render_to(writer, &object)
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

    make_liquid_template!(TestFile, "templates/test.txt", PARSER);

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
        assert_eq!(file_contents, "Hello, World!\n");
    }
}
