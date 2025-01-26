use std::path::Path;

use crate::Anvil;

/// A struct that can be used to combine two Anvil structs
/// and write them into a file. If the first Anvil struct
/// fails to write, the second one will be written instead.
pub struct Either<L, R>
where
    L: Anvil,
    R: Anvil,
{
    left: L,
    right: R,
}

impl<L, R> Anvil for Either<L, R>
where
    L: Anvil,
    R: Anvil,
{
    type Error = R::Error;
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        self.left.forge(&into).or_else(|_| self.right.forge(&into))
    }
}

impl<L, R> Either<L, R>
where
    L: Anvil,
    R: Anvil,
{
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

#[inline]
pub fn either<L: Anvil, R: Anvil>(left: L, right: R) -> Either<L, R> {
    Either::new(left, right)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{append::append, generate::generate};
    use askama::Template;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[derive(Template)]
    #[template(source = "Generated content.", ext = "txt")]
    struct GeneratedTemplate;

    #[derive(Template)]
    #[template(source = "Appended content.", ext = "txt")]
    struct AppendedTemplate;

    #[test]
    fn it_runs_second_if_first_fails() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content.").unwrap();
        let result =
            either(generate(&GeneratedTemplate), append(&AppendedTemplate)).forge(&file_path);
        assert!(result.is_ok());
        let file_contents = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_contents, "Initial content.\nAppended content.");
    }

    #[test]
    fn it_succeeds_if_first_succeeds() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let result =
            either(generate(&GeneratedTemplate), append(&AppendedTemplate)).forge(&file_path);
        assert!(result.is_ok());
        let file_contents = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_contents, "Generated content.");
    }
}
