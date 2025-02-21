use std::io::Write;

use askama::Template;
use regex::Regex;
use thiserror::Error;

use crate::Anvil;

/// A struct that can be used to replace a line in a file with a Template.
/// The file will NOT be created if it does not exist.
/// The Template will replace the line that matches the given regex.
/// If the line is not found, the Template will not be injected.
/// All lines that match the regex will be replaced.
pub struct Replace<'a, T: Template> {
    template: &'a T,
    regex: Regex,
}

impl<'a, T: Template> Replace<'a, T> {
    pub fn new(template: &'a T, regex: Regex) -> Self {
        Self { template, regex }
    }
}

#[derive(Error, Debug)]
pub enum ReplaceError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
    #[error("askama error {0}")]
    Askama(#[from] askama::Error),
}

impl<T: Template> Anvil for Replace<'_, T> {
    type Error = ReplaceError;

    fn forge(&self, into: impl AsRef<std::path::Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let file_contents = std::fs::read_to_string(path).map_err(ReplaceError::StdIo)?;
        let template = self.template.render().map_err(ReplaceError::Askama)?;

        let mut lines = file_contents.lines().collect::<Vec<_>>();

        for line in lines.iter_mut() {
            if self.regex.is_match(line) {
                *line = template.as_str();
            }
        }

        let file = std::fs::File::create(path).map_err(ReplaceError::StdIo)?;
        let mut writer = std::io::BufWriter::new(file);

        writer
            .write_all(lines.join("\n").as_bytes())
            .map_err(ReplaceError::StdIo)?;

        Ok(())
    }
}

fn replace<T: Template>(template: &T, regex: Regex) -> Replace<T> {
    Replace::new(template, regex)
}

#[cfg(test)]
mod test {

    use super::*;

    #[derive(Template)]
    #[template(source = "Inject content.", ext = "txt")]
    struct TestTemplate;

    #[test]
    fn it_fails_if_file_does_not_exist() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let result =
            replace(&TestTemplate, Regex::new("Initial content.").unwrap()).forge(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn it_replaces_line() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = std::fs::File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result =
            replace(&TestTemplate, Regex::new("Initial content.").unwrap()).forge(&file_path);
        assert!(result.is_ok());
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Inject content.");
    }

    #[test]
    fn it_does_not_replace_if_match_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = std::fs::File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result =
            replace(&TestTemplate, Regex::new("Non-existent content.").unwrap()).forge(&file_path);
        assert!(result.is_ok());
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Initial content.");
    }
}
