use std::convert::Infallible;

use crate::{
    extractors::Extractor,
    http::request::{FromRequest, Request},
};

impl From<axum::extract::Request> for Request {
    fn from(value: axum::extract::Request) -> Self {
        let (parts, body) = value.into_parts();

        Request(http::Request::from_parts(parts, body.into()))
    }
}

impl From<Request> for axum::extract::Request {
    fn from(value: Request) -> Self {
        let (parts, body) = value.into_parts();
        axum::extract::Request::from_parts(parts, body.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;
    #[tokio::test]
    async fn test_anvil_to_axum_request_body_conversion() {
        let request = Request::new("Hello, World!".into());
        let axum_request: axum::extract::Request = request.into();
        let bytes = axum_request.collect().await.unwrap().to_bytes();
        assert_eq!(bytes, "Hello, World!");
    }

    #[tokio::test]
    async fn test_axum_to_anvil_request_body_conversion() {
        let axum_request =
            axum::extract::Request::new(axum::body::Body::new("Hello, World!".to_string()));
        let request: Request = axum_request.into();
        let bytes = request.0.collect().await.unwrap().to_bytes();
        assert_eq!(bytes, "Hello, World!");
    }
}
