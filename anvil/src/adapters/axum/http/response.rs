use axum::response::IntoResponse as AxumIntoResponse;

use crate::http::response::{
    into_response::{IntoResponse, Responder},
    Response,
};

impl From<axum::response::Response> for Response {
    fn from(value: axum::response::Response) -> Self {
        let (parts, body) = value.into_parts();
        Response(http::response::Response::from_parts(parts, body.into()))
    }
}

impl From<Response> for axum::response::Response {
    fn from(value: Response) -> Self {
        let (parts, body) = value.into_parts();
        axum::response::Response::from_parts(parts, body.into())
    }
}

impl<T> AxumIntoResponse for Responder<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::from(self.0.into_response())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_anvil_to_axum_response_body_conversion() {
        let response = Response::new("Hello, World!".into());

        let axum_response: axum::response::Response = response.into();

        let bytes = axum_response.collect().await.unwrap().to_bytes();
        assert_eq!(bytes, "Hello, World!");
    }

    #[tokio::test]
    async fn test_axum_to_anvil_response_body_conversion() {
        let axum_response =
            axum::response::Response::new(axum::body::Body::new("Hello, World!".to_string()));

        let response: Response = axum_response.into();

        let bytes = response.0.collect().await.unwrap().to_bytes();
        assert_eq!(bytes, "Hello, World!");
    }

    #[tokio::test]
    async fn test_anvil_to_axum_response_header_conversion() {
        let mut response = Response::new("Hello, World!".into());

        response
            .headers_mut()
            .insert("Content-Type", "text/plain".parse().unwrap());
        response
            .headers_mut()
            .insert("Content-Length", "13".parse().unwrap());

        let axum_response: axum::response::Response = response.into();

        assert_eq!(
            axum_response.headers().get("Content-Type").unwrap(),
            "text/plain"
        );

        assert_eq!(axum_response.headers().get("Content-Length").unwrap(), "13");
    }

    #[tokio::test]
    async fn test_axum_to_anvil_response_header_conversion() {
        let mut axum_response =
            axum::response::Response::new(axum::body::Body::new("Hello, World!".to_string()));

        axum_response
            .headers_mut()
            .insert("Content-Type", "text/plain".parse().unwrap());

        axum_response
            .headers_mut()
            .insert("Content-Length", "13".parse().unwrap());

        let response: Response = axum_response.into();

        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "text/plain"
        );

        assert_eq!(response.headers().get("Content-Length").unwrap(), "13");
    }

    #[tokio::test]
    async fn test_axum_to_anvil_response_extension_conversion() {
        let mut axum_response =
            axum::response::Response::new(axum::body::Body::new("Hello, World!".to_string()));

        axum_response.extensions_mut().insert("key");

        let response: Response = axum_response.into();

        assert_eq!(response.0.extensions().len(), 1);
        assert_eq!(response.0.extensions().get::<&str>(), Some(&"key"));
    }

    #[tokio::test]
    async fn test_anvil_to_axum_response_extension_conversion() {
        let mut response = Response::new("Hello, World!".into());
        response.0.extensions_mut().insert("key");
        let axum_response: axum::response::Response = response.into();
        assert_eq!(axum_response.extensions().len(), 1);
        assert_eq!(axum_response.extensions().get::<&str>(), Some(&"key"));
    }
}
