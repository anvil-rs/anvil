use anvil::{append::Append, Forge};
use askama::Template;

use crate::Askama;

pub trait AskamaAppendExt<'a, T: Template>: Forge {
    fn askama(template: &'a T) -> Self;
}

impl<'a, T: Template> AskamaAppendExt<'a, T> for Append<Askama<'a, T>> {
    fn askama(template: &'a T) -> Self {
        Self::new(Askama(template))
    }
}

#[inline(always)]
pub fn append<T: Template>(template: &T) -> Append<Askama<T>> {
    Append::askama(template)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[derive(Template)]
    #[template(source = "Appended content.", ext = "txt")]
    struct TestTemplate;

    #[test]
    fn it_fails_if_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let result = append(&TestTemplate).forge(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn it_appends_to_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result = append(&TestTemplate).forge(&file_path);
        assert!(result.is_ok());
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Initial content.\nAppended content.");
    }
}
