use axum::response::IntoResponse as AxumIntoResponse;

use crate::http::response::{IntoResponse, Responder, Response};

impl From<axum::response::Response> for Response {
    fn from(value: axum::response::Response) -> Self {
        let (parts, body) = value.into_parts();
        Response(http::response::Response::from_parts(parts, body.into()))
    }
}

impl From<Response> for axum::response::Response {
    fn from(value: Response) -> Self {
        let (parts, body) = value.0.into_parts();
        axum::response::Response::from_parts(parts, body.into())
    }
}

impl<T> AxumIntoResponse for Responder<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        self.0.into_response().into()
    }
}
