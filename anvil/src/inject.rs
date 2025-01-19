use std::{io::Write, path::Path};

use askama::Template;
use regex::Regex;
use thiserror::Error;

use crate::Anvil;

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
}

impl<T> Anvil for Inject<'_, T>
where
    T: Template,
{
    type Error = InjectError;

    fn render(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
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

// macro rule for inject, can have 0 or 2 regex additional arguments. but not necessary, sets as
// None if not provided. The inject macro should take the before and after fields as named inputs,


pub fn inject<T: Template>(template: &T, before: Regex, after: Regex) -> Inject<T> {
    Inject::new(template, Some(before), Some(after))
}

pub fn inject_before<T: Template>(template: &T, before: Regex) -> Inject<T> {
    Inject::new(template, Some(before), None)
}

pub fn inject_after<T: Template>(template: &T, after: Regex) -> Inject<T> {
    Inject::new(template, None, Some(after))
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
