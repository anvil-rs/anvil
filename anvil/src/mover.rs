use std::path::{Path, PathBuf};

use crate::Forge;

pub struct Move {
    from: PathBuf,
}

impl Move {
    pub fn new(from: impl AsRef<Path>) -> Self {
        Self {
            from: from.as_ref().to_path_buf(),
        }
    }
}

impl Forge for Move {
    type Error = std::io::Error;
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let to = into.as_ref();
        std::fs::rename(&self.from, to)?;
        Ok(())
    }
}
