use std::future::Future;

use crate::http::body::Body;

use super::response::Response;

pub struct Request(http::Request<Body>);

pub trait FromRequest: Sized {
    type Error: Into<Response>;
    type Future: Future<Output = Result<Self, Self::Error>>;

    fn from_request(req: &Request) -> Result<Self, Self::Error>;
}
