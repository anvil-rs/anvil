pub mod into_response;
pub mod into_response_parts;

use http::{response::Parts, HeaderMap, HeaderValue, StatusCode};

use crate::http::body::Body;

//TODO: Make Response generic over body.
#[derive(Debug)]
pub struct Response<T = Body>(pub http::Response<T>);

impl<T> Response<T> {
    #[inline]
    pub fn new(body: T) -> Self {
        Self(http::Response::new(body))
    }

    #[inline]
    pub fn from_parts(parts: Parts, body: T) -> Self {
        Self(http::Response::from_parts(parts, body))
    }

    #[inline]
    pub fn status(&self) -> StatusCode {
        self.0.status()
    }

    #[inline]
    pub fn status_mut(&mut self) -> &mut StatusCode {
        self.0.status_mut()
    }

    #[inline]
    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.0.headers()
    }

    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        self.0.headers_mut()
    }

    #[inline]
    pub fn body(&self) -> &T {
        self.0.body()
    }

    #[inline]
    pub fn into_body(self) -> T {
        self.0.into_body()
    }

    #[inline]
    pub fn into_parts(self) -> (Parts, T) {
        self.0.into_parts()
    }

    #[inline]
    pub fn extensions(&self) -> &http::Extensions {
        self.0.extensions()
    }

    #[inline]
    pub fn extensions_mut(&mut self) -> &mut http::Extensions {
        self.0.extensions_mut()
    }

    #[inline]
    pub fn map<F, U>(self, f: F) -> Response<U>
    where
        F: FnOnce(T) -> U,
    {
        Response(self.0.map(f))
    }
}

impl From<http::Response<Body>> for Response {
    fn from(value: http::Response<Body>) -> Self {
        Self(value)
    }
}

impl From<Response> for http::Response<Body> {
    fn from(value: Response) -> Self {
        value.0
    }
}
