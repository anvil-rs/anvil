use crate::routes::handler::Handler;

#[derive(Default, Clone)]
pub struct Routes {
    prefix: Option<String>,
    handlers: Vec<Handler>,
}

impl Routes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn at(prefix: &str) -> Self {
        Self {
            prefix: Some(prefix.to_string()),
            ..Self::default()
        }
    }

    pub fn prefix(mut self, uri: &str) -> Self {
        self.prefix = Some(uri.to_owned());
        self
    }

    pub fn add(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }
}
