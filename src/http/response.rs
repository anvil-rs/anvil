use crate::http::body::Body;

pub type Response<T = Body> = http::Response<T>;

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
struct Responder<T: IntoResponse>(T);

impl<T> IntoResponse for Responder<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}

// impl<T> axum::response::IntoResponse for Responder<T>
// where
//     T: IntoResponse,
// {
//     fn into_response(self) -> axum::response::Response {
//         self.0.into_response().into()
//     }
// }

// pub trait IntoResponse {
//     fn into_response(self) -> Response;
// }
//
// impl<B> IntoResponse for http::Response<B>
// where
//     B: http_body::Body<Data = Bytes> + Send + 'static,
//     B::Error: Into<BoxError>,
// {
//     fn into_response(self) -> Response {
//         self.map(Body::new)
//     }
// }
//
//
