use std::{io::Write, path::Path};

use askama::Template;
use regex::Regex;
use thiserror::Error;

use crate::Anvil;

/// A struct that can be used to inject a Template into a file.
/// The file will NOT be created if it does not exist.
/// The Template will be injected before or after a given line.
/// If the line is not found, the Template will not be injected.
pub struct Inject<'a, T>
where
    T: Template,
{
    template: &'a T,
    before: Option<Regex>,
    after: Option<Regex>,
}

impl<'a, T> Inject<'a, T>
where
    T: Template,
{
    pub fn new(template: &'a T, before: Option<Regex>, after: Option<Regex>) -> Self {
        Self {
            template,
            before,
            after,
        }
    }
}

#[derive(Error, Debug)]
pub enum InjectError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
    #[error("askama error {0}")]
    Askama(#[from] askama::Error),
}

impl<T> Anvil for Inject<'_, T>
where
    T: Template,
{
    type Error = InjectError;

    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let file_contents = std::fs::read_to_string(path).map_err(InjectError::StdIo)?;
        let template = self.template.render().map_err(InjectError::Askama)?;

        let mut lines = file_contents.lines().collect::<Vec<_>>();

        if let Some(before) = &self.before {
            let pos = lines.iter().position(|ln| before.is_match(ln));
            if let Some(pos) = pos {
                lines.insert(pos, template.as_str());
            }
        }

        if let Some(after) = &self.after {
            let pos = lines.iter().position(|ln| after.is_match(ln));
            if let Some(pos) = pos {
                lines.insert(pos + 1, template.as_str());
            }
        }

        let file = std::fs::File::create(path).map_err(InjectError::StdIo)?;
        let mut writer = std::io::BufWriter::new(file);

        writer
            .write_all(lines.join("\n").as_bytes())
            .map_err(InjectError::StdIo)
    }
}

// macro rule for inject, can have 0 or 2 regex additional arguments. but not necessary, sets as
// None if not provided. The inject macro should take the before and after fields as named inputs,

#[inline]
pub fn inject<T: Template>(template: &T, before: Regex, after: Regex) -> Inject<T> {
    Inject::new(template, Some(before), Some(after))
}

#[inline]
pub fn inject_before<T: Template>(template: &T, before: Regex) -> Inject<T> {
    Inject::new(template, Some(before), None)
}

#[inline]
pub fn inject_after<T: Template>(template: &T, after: Regex) -> Inject<T> {
    Inject::new(template, None, Some(after))
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[derive(Template)]
    #[template(source = "Inject content.", ext = "txt")]
    struct TestTemplate;

    #[test]
    fn it_fails_if_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let result = inject(
            &TestTemplate,
            Regex::new("Initial content.").unwrap(),
            Regex::new("Initial content.").unwrap(),
        )
        .forge(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn it_injects_before_line() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result =
            inject_before(&TestTemplate, Regex::new("Initial content.").unwrap()).forge(&file_path);
        assert!(result.is_ok());
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Inject content.\nInitial content.");
    }

    #[test]
    fn it_injects_after_line() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result =
            inject_after(&TestTemplate, Regex::new("Initial content.").unwrap()).forge(&file_path);
        assert!(result.is_ok());
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Initial content.\nInject content.");
    }

    #[test]
    fn it_does_not_inject_if_match_not_found() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result = inject_after(&TestTemplate, Regex::new("Non-existent content.").unwrap())
            .forge(&file_path);
        assert!(result.is_ok());
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Initial content.");
    }

    #[test]
    fn it_injects_before_and_after_line() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result = inject(
            &TestTemplate,
            Regex::new("Initial content.").unwrap(),
            Regex::new("Initial content.").unwrap(),
        )
        .forge(&file_path);
        assert!(result.is_ok());
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(
            content,
            "Inject content.\nInitial content.\nInject content."
        );
    }
}
