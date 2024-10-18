use crate::http::body::Body;

pub type Response<T = Body> = http::Response<T>;

pub struct ErrorResponse(Response);

// impl<T> From<T> for ErrorResponse
// where
//     T: IntoResponse,
// {
//     fn from(value: T) -> Self {
//         Self(value.into_response())
//     }
// }
