use std::{io::Write, path::Path};

use regex::Regex;
use thiserror::Error;

use crate::{Anvil, Forge};

pub struct Inject<'a, A>
where
    A: Anvil,
{
    template: &'a A,
    before: Option<Regex>,
    after: Option<Regex>,
}

impl<'a, A> Inject<'a, A>
where
    A: Anvil,
{
    pub fn new(template: &'a A, before: Option<Regex>, after: Option<Regex>) -> Self {
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
    #[error("template error")]
    Template,
}

impl<A> Forge for Inject<'_, A>
where
    A: Anvil,
{
    type Error = InjectError;

    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let file_contents = std::fs::read_to_string(path).map_err(InjectError::StdIo)?;
        let template = self.template.render().unwrap();

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

#[cfg(test)]
mod test {
    // use super::*;
    //
    // #[derive(Template)]
    // #[template(path = "tests/test.html")]
    // struct TestTemplate;
    //
    // #[test]
    // fn test_inject() {
    //     let inject = inject!(&TestTemplate, "tests/inject.html");
    // }
    //
    // #[test]
    // fn test_inject_before() {
    //     let inject = inject!(
    //         &TestTemplate,
    //         "tests/inject.html",
    //         before = Regex::new(r"<!DOCTYPE html>").unwrap()
    //     );
    // }
    //
    // #[test]
    // fn test_inject_after() {
    //     let inject = inject!(
    //         &TestTemplate,
    //         "tests/inject.html",
    //         after = Regex::new(r"<!DOCTYPE html>").unwrap()
    //     );
    // }
    //
    // #[test]
    // fn test_inject_before_after() {
    //     let inject = inject!(
    //         &TestTemplate,
    //         "tests/inject.html",
    //         before = Regex::new(r"<!DOCTYPE html>").unwrap(),
    //         after = Regex::new(r"<html>").unwrap()
    //     );
    // }
}

// manually implement Template for inject?
// Then we don't need a custom trait.
