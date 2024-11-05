use std::str::FromStr;

use actix_web::{dev::Payload, error::PayloadError, HttpRequest};

use bytes::Bytes;
use futures_util::{StreamExt, TryStreamExt};
use http_body::{Body as HttpBody, Frame};
use std::io::Error as IoError;

use crate::http::{body::Body, request::Request};

trait FromActixRequest {
    fn from_actix_request(req: &HttpRequest, payload: &mut Payload) -> Self;
}

struct PayloadWrapper(Payload);

unsafe impl Send for PayloadWrapper {}

impl HttpBody for PayloadWrapper {
    type Data = Bytes;

    type Error = PayloadError;

    fn poll_frame(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<http_body::Frame<Self::Data>, Self::Error>>> {
        self.get_mut()
            .0
            .poll_next_unpin(cx)
            .map(|opt| opt.map(|res| res.map(Frame::data)))
    }
}

// impl Body for

impl FromActixRequest for Request {
    fn from_actix_request(req: &HttpRequest, payload: &mut Payload) -> Self {
        let body = Body::new(PayloadWrapper(payload.take()));

        let mut builder = http::Request::builder()
            .uri(req.uri().to_string())
            .method(req.method().as_str());

        let version = match payload {
            Payload::None => http::version::Version::HTTP_2,
            Payload::Stream { .. } => http::version::Version::HTTP_2,
            Payload::H1 { .. } => http::version::Version::HTTP_11,
            Payload::H2 { .. } => http::version::Version::HTTP_2,
        };

        builder = builder.version(version);

        for (key, value) in req.headers() {
            builder = builder.header(key.as_str(), value.to_str().unwrap());
        }

        Request(builder.body(body).unwrap())
    }
}

impl From<Request> for HttpRequest {
    fn from(value: Request) -> Self {
        let (parts, _) = value.into_parts();

        let method = actix_web::http::Method::from_str(parts.method.as_str()).unwrap();

        let mut req =
            actix_web::test::TestRequest::with_uri(parts.uri.to_string().as_str()).method(method);

        for (key, value) in parts.headers {
            req = req.insert_header((key.unwrap().as_str(), value.as_bytes()));
        }

        req.to_http_request()
    }
}

impl From<Request> for Payload {
    fn from(value: Request) -> Self {
        let (_, body) = value.into_parts();

        let stream = body
            .into_data_stream()
            .map_err(|e| {
                // TODO: Map this better?
                PayloadError::Incomplete(Some(IoError::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                )))
            })
            .boxed_local();

        actix_web::dev::Payload::from(stream)
    }
}

#[cfg(test)]
mod test {
    use http::Version;

    use super::*;

    async fn payload_to_bytes(payload: Payload) -> Result<Bytes, PayloadError> {
        actix_web::body::to_bytes(actix_web::body::BodyStream::new(payload)).await
    }

    #[actix_web::test]
    async fn should_convert_from_request_body_to_payload() {
        let body = Body::new("Hello, World!".to_string());

        let request: Request<Body> = Request::new(body);

        let payload = Payload::from(request);

        let bytes = payload_to_bytes(payload).await.unwrap();

        assert_eq!(bytes, "Hello, World!");
    }

    #[actix_web::test]
    async fn should_convert_from_request_to_actix_request() {
        let body = Body::new("Hello, World!".to_string());

        let builder = http::Request::builder()
            .uri("http://localhost:8080")
            .method("GET")
            .header("content-type", "text/plain")
            .version(Version::HTTP_2)
            .body(body)
            .unwrap();

        let request: Request<Body> = Request(builder);
        let actix_request: HttpRequest = request.into();

        assert_eq!(actix_request.method(), actix_web::http::Method::GET);
        assert_eq!(actix_request.uri().path(), "/");
        assert_eq!(actix_request.uri().host().unwrap(), "localhost");
        assert_eq!(actix_request.uri().port_u16().unwrap(), 8080);
        assert_eq!(
            actix_request.headers().get("content-type").unwrap(),
            "text/plain"
        );
    }
}
