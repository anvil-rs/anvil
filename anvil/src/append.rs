use std::{io::BufWriter, path::Path};

use askama::Template;
use thiserror::Error;

use crate::Anvil;

#[derive(Error, Debug)]
pub enum AppendError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
}

/// A struct that can be used to append a Template to a file.
/// The file will NOT be created if it does not exist.
pub struct Append<'a, T>
where
    T: Template,
{
    template: &'a T,
}

impl<T> Anvil for Append<'_, T>
where
    T: Template,
{
    type Error = AppendError;

    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let file = std::fs::OpenOptions::new()
            .append(true)
            .open(path)
            .map_err(AppendError::StdIo)?;

        let mut writer = BufWriter::new(file);

        self.template
            .write_into(&mut writer)
            .map_err(AppendError::StdIo)?;

        Ok(())
    }
}

impl<'a, T> Append<'a, T>
where
    T: Template,
{
    pub fn new(template: &'a T) -> Self {
        Self { template }
    }
}

#[inline]
pub fn append<T: Template>(template: &T) -> Append<T> {
    Append::new(template)
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
