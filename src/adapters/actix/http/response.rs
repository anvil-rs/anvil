use actix_web::{body::MessageBody, HttpResponse as ActixHttpResponse};

use crate::http::{
    body::Body,
    response::{IntoResponse, Responder, Response},
};

struct StatusCode(http::StatusCode);

impl From<StatusCode> for actix_web::http::StatusCode {
    fn from(value: StatusCode) -> Self {
        actix_web::http::StatusCode::from_u16(value.0.as_u16()).unwrap()
    }
}

impl From<Response> for ActixHttpResponse {
    fn from(value: Response) -> Self {
        let (parts, body) = value.0.into_parts();
        // TODO: Implement the rest of the http request.
        ActixHttpResponse::build(
            actix_web::http::StatusCode::from_u16(parts.status.as_u16()).unwrap(),
        )
        .body(body)
    }
}

impl From<ActixHttpResponse> for Response {
    fn from(value: ActixHttpResponse) -> Self {
        let (parts, body) = value.into_parts();

        let bytes = body.try_into_bytes();

        let bytes = match bytes {
            Ok(bytes) => bytes,
            Err(_) => unimplemented!(),
        };

        let body = Body::from(bytes);

        todo!()
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
