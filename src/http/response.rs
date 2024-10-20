use crate::http::body::Body;

pub type Response<T = Body> = http::Response<T>;

pub struct ErrorResponse(Response);

impl<T> From<T> for ErrorResponse
where
    T: IntoResponse,
{
    fn from(value: T) -> Self {
        Self(value.into_response())
    }
}

struct Responder<T>(T);

pub trait IntoResponse {
    /// Create a response.
    #[must_use]
    fn into_response(self) -> Response;
}

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
