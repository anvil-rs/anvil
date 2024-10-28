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

pub trait IntoResponse {
    /// Create a response.
    #[must_use]
    fn into_response(self) -> Response;
}

#[derive(Debug)]
#[must_use]
pub struct ErrorResponse(Response);

impl<T> From<T> for ErrorResponse
where
    T: IntoResponse,
{
    fn from(value: T) -> Self {
        Self(value.into_response())
    }
}

impl<T> IntoResponse for Result<T, ErrorResponse>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Ok(ok) => ok.into_response(),
            Err(err) => err.0,
        }
    }
}

// Responder is a wrapper around a type that implements IntoResponse.
// This allows us to implement IntoResponse for any type that implements IntoResponse.
// This is useful for abstracting over different response types.
pub struct Responder<T: IntoResponse>(pub T);

impl<T> IntoResponse for Responder<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}
