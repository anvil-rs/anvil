use std::path::Path;
use std::{fs::File, io::BufWriter};

use thiserror::Error;

use crate::Anvil;
use crate::Forge;

/// A struct that can be used to generate a file from a Template.
/// The file will be created if it does not exist.
/// If the file already exists, it will be overwritten.
pub struct Generate<A: Anvil> {
    template: A,
}

#[derive(Error, Debug)]
pub enum GenerateError {
    #[error("file error {0}")]
    StdIo(#[from] std::io::Error),
    #[error("template error")]
    Template,
}

impl<A: Anvil> Forge for Generate<A> {
    type Error = GenerateError;

    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();

        let prefix = path.parent().expect("no parent directory");
        std::fs::create_dir_all(prefix).map_err(GenerateError::StdIo)?;

        let file = File::create_new(path).map_err(GenerateError::StdIo)?;

        let mut writer = BufWriter::new(file);

        self.template
            .anvil(&mut writer)
            .map_err(|_e| GenerateError::Template)?;

        Ok(())
    }
}

impl<A: Anvil> Generate<A> {
    pub fn new(template: A) -> Self {
        Self { template }
    }
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
//     #[template(source = "Generated content.", ext = "txt")]
//     struct TestTemplate;
//
//     #[test]
//     fn it_fails_if_path_already_exists() {
//         let dir = tempdir().unwrap();
//         let file_path = dir.path().join("my-temporary-note.txt");
//         let mut file = File::create(&file_path).unwrap();
//         writeln!(file, "Initial content.").unwrap();
//         let result = generate(&TestTemplate).forge(&file_path);
//         assert!(result.is_err());
//         let file_contents = std::fs::read_to_string(&file_path).unwrap();
//         assert_eq!(file_contents, "Initial content.\n");
//     }
//
//     #[test]
//     fn it_generates_file() {
//         let dir = tempdir().unwrap();
//         let file_path = dir.path().join("my-temporary-note.txt");
//         let result = generate(&TestTemplate).forge(&file_path);
//         assert!(result.is_ok());
//         let file_contents = std::fs::read_to_string(&file_path).unwrap();
//         assert_eq!(file_contents, "Generated content.");
//     }
// }
