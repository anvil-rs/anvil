use crate::http::body::Body;

//TODO: Make Response generic over body.
#[derive(Debug)]
pub struct Response(pub http::Response<Body>);

impl Response {
    pub fn new(body: Body) -> Self {
        Self(http::Response::new(body))
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
