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

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_anvil_to_axum_response_conversion() {
        use crate::http::response::Response;
        let response = Response::new("Hello, World!".into());
        let axum_response: axum::response::Response = response.into();
        let bytes = axum_response.collect().await.unwrap().to_bytes();
        assert_eq!(bytes, "Hello, World!");
    }

    #[tokio::test]
    async fn test_axum_to_anvil_response_conversion() {
        use crate::http::response::Response;
        let axum_response =
            axum::response::Response::new(axum::body::Body::new("Hello, World!".to_string()));
        let response: Response = axum_response.into();
        let bytes = response.0.collect().await.unwrap().to_bytes();
        assert_eq!(bytes, "Hello, World!");
    }
}
