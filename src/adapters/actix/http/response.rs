use std::{any::Any, borrow::BorrowMut, ops::Deref};

use actix_web::HttpResponse as ActixHttpResponse;

use crate::http::{
    body::Body,
    response::{IntoResponse, Responder, Response},
};

impl From<Response> for ActixHttpResponse {
    fn from(value: Response) -> Self {
        let (parts, body) = value.into_parts();

        let mut builder = ActixHttpResponse::build(
            actix_web::http::StatusCode::from_u16(parts.status.as_u16())
                .expect("Invalid status code"),
        );

        for (key, value) in parts.headers {
            if let (Some(name), Ok(val)) = (key, value.to_str()) {
                builder.append_header((name.as_str(), val));
            }
        }

        builder.body(body)
    }
}

impl From<ActixHttpResponse> for Response {
    fn from(value: ActixHttpResponse) -> Self {
        let (parts, body) = value.into_parts();

        let mut builder = http::response::Builder::new().status(parts.status().as_u16());

        for (key, value) in parts.headers().iter() {
            builder = builder.header(key.as_str(), value.as_bytes());
        }

        let response: http::response::Response<Body> =
            builder.body(body.into()).expect("Invalid body");

        Response::from(response)
    }
}

impl<T> actix_web::Responder for Responder<T>
where
    T: IntoResponse,
{
    type Body = actix_web::body::BoxBody;

    /// WARN: The `req` parameter is not used.
    fn respond_to(self, _req: &actix_web::HttpRequest) -> ActixHttpResponse<Self::Body> {
        self.0.into_response().into()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::body::to_bytes;

    #[actix_web::test]
    async fn test_anvil_to_actix_response_body_conversion() {
        let response = Response::new("Hello, World!".into());
        let actix_response: ActixHttpResponse = response.into();
        let bytes = to_bytes(actix_response.into_body()).await.unwrap();
        assert_eq!(bytes, "Hello, World!");
    }

    #[actix_web::test]
    async fn test_actix_to_anvil_response_body_conversion() {
        let actix_response = actix_web::HttpResponse::Ok().body("Hello, World!");
        let response: Response = actix_response.into();
        let bytes = to_bytes(response.0.into_body()).await.unwrap();
        assert_eq!(bytes, "Hello, World!");
    }

    #[actix_web::test]
    async fn test_anvil_to_actix_response_header_conversion() {
        let mut response = Response::new("Hello, World!".into());

        response
            .headers_mut()
            .insert("Content-Type", "text/plain".parse().unwrap());

        response
            .headers_mut()
            .insert("Content-Length", "13".parse().unwrap());
        let actix_response: ActixHttpResponse = response.into();

        assert_eq!(
            actix_response.headers().get("Content-Type").unwrap(),
            "text/plain"
        );

        assert_eq!(
            actix_response.headers().get("Content-Length").unwrap(),
            "13"
        );
    }

    #[actix_web::test]
    async fn test_actix_to_anvil_response_header_conversion() {
        let mut actix_response = actix_web::HttpResponse::Ok().body("Hello, World!");
        actix_response.headers_mut().append(
            actix_web::http::header::CONTENT_TYPE,
            "text/plain".parse().unwrap(),
        );
        actix_response.headers_mut().append(
            actix_web::http::header::CONTENT_LENGTH,
            "13".parse().unwrap(),
        );

        let response: Response = actix_response.into();

        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "text/plain"
        );

        assert_eq!(response.headers().get("Content-Length").unwrap(), "13");
    }
}
