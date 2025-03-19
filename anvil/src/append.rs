use std::{io::BufWriter, path::Path};
use thiserror::Error;

use crate::{Anvil, Forge};

#[derive(Error, Debug)]
pub enum AppendError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
    #[error("templating error")]
    // TODO: Store box dyn error (or option of box dyn error)
    Template,
}

/// A struct that can be used to append a Template to a file.
/// The file will NOT be created if it does not exist.
pub struct Append<A: Anvil> {
    template: A,
}

impl<A: Anvil> Forge for Append<A> {
    type Error = AppendError;

    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let file = std::fs::OpenOptions::new()
            .append(true)
            .open(path)
            .map_err(AppendError::StdIo)?;

        let mut writer = BufWriter::new(file);

        self.template
            .anvil(&mut writer)
            .map_err(|_e| AppendError::Template)?;

        Ok(())
    }
}

impl<A: Anvil> Append<A> {
    pub fn new(template: A) -> Self {
        Self { template }
    }
}

#[inline]
pub fn append<A: Anvil>(template: A) -> Append<A> {
    Append::new(template)
}

// #[cfg(test)]
// mod test {
//
//     use super::*;
//     use std::fs::File;
//     use std::io::Write;
//     use tempfile::tempdir;
//
//     #[derive(Template)]
//     #[template(source = "Appended content.", ext = "txt")]
//     struct TestTemplate;
//
//     #[test]
//     fn it_fails_if_file_does_not_exist() {
//         let dir = tempdir().unwrap();
//         let file_path = dir.path().join("my-temporary-note.txt");
//         let result = append(&TestTemplate).forge(&file_path);
//         assert!(result.is_err());
//     }
//
//     #[test]
//     fn it_appends_to_file() {
//         let dir = tempdir().unwrap();
//         let file_path = dir.path().join("my-temporary-note.txt");
//         let mut file = File::create(&file_path).unwrap();
//         writeln!(file, "Initial content.").unwrap();
//         let result = append(&TestTemplate).forge(&file_path);
//         assert!(result.is_ok());
//         let content = std::fs::read_to_string(&file_path).unwrap();
//         assert_eq!(content, "Initial content.\nAppended content.");
//     }
// }
