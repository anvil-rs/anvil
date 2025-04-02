use anvil::{append::Append, Forge};

use crate::{Aqua, Water};

pub trait LiquidAppendExt<'a, T: Water>: Forge {
    fn liquid(template: &'a T) -> Self;
}

impl<'a, T: Water> LiquidAppendExt<'a, T> for Append<Aqua<'a, T>> {
    fn liquid(template: &'a T) -> Self {
        Self::new(Aqua(template))
    }
}

#[inline(always)]
pub fn append<T: Water>(template: &T) -> Append<Aqua<'_, T>> {
    Append::liquid(template)
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

    // NOTE: This template needs the dummy braces to be recognized by Tera because a completely
    // empty struct us not parseable json.
    #[derive(Serialize)]
    struct TestTemplate {}

    impl Water for TestTemplate {
        fn liquid(&self, writer: &mut dyn std::io::Write) -> Result<(), liquid::Error> {
            let object = liquid::to_object(self)?;
            let template = PARSER.parse("Appended content.")?;
            template.render_to(writer, &object)
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

    make_liquid_template!(TestFile, "templates/test.txt", PARSER);

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
        // trim newline to avoid platform differences
        assert_eq!(content.trim(), "Initial content.\nHello, World!");
    }
}
