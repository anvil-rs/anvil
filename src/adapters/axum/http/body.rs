use crate::http::body::boxed;

use crate::http::body::Body;

impl From<axum::body::Body> for Body {
    fn from(value: axum::body::Body) -> Self {
        Body::new(boxed(value))
    }
}

impl From<Body> for axum::body::Body {
    fn from(value: Body) -> Self {
        axum::body::Body::new(boxed(value))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_anvil_to_axum_body_conversion() {
        let body = Body::from("Hello, World!".to_string());
        let axum_body: axum::body::Body = body.into();

        let bytes = axum_body.collect().await.unwrap().to_bytes();

        assert_eq!(bytes, "Hello, World!");
    }

    #[tokio::test]
    async fn test_axum_to_anvil_body_conversion() {
        let axum_body = axum::body::Body::from("Hello, World!".to_string());
        let body: Body = axum_body.into();
        let bytes = body.collect().await.unwrap().to_bytes();
        assert_eq!(bytes, "Hello, World!");
    }
}
