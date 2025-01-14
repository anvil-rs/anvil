use regex::Regex;

use crate::Forge;

pub struct Remove {
    pattern: Regex,
}

impl Remove {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: Regex::new(pattern).expect("invalid regex pattern"),
        }
    }
}

impl Forge for Remove {
    type Error = std::io::Error;

    fn forge(&self, into: impl AsRef<std::path::Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let content = std::fs::read_to_string(path)?;
        let content = self.pattern.replace_all(&content, "").into_owned();
        std::fs::write(path, content)?;
        Ok(())
    }
}

pub fn remove(pattern: &str) -> Remove {
    Remove::new(pattern)
}
