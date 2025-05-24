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

#[inline(always)]
pub fn generate<T: Template>(template: &T) -> Generate<Askama<T>> {
    Generate::askama(template)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[derive(Template)]
    #[template(source = "Generated content.", ext = "txt")]
    struct TestTemplate;

    #[test]
    fn it_fails_if_path_already_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result = generate(&TestTemplate).forge(&file_path);
        assert!(result.is_err());
        let file_contents = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_contents.trim(), "Initial content.");
    }

    #[test]
    fn it_generates_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let result = generate(&TestTemplate).forge(&file_path);
        assert!(result.is_ok());
        let file_contents = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_contents, "Generated content.");
    }
}
