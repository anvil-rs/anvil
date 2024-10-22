use http_body_util::BodyExt;

use crate::http::body::Body;

#[derive(Debug)]
pub struct Response(pub http::Response<Body>);

impl From<http::Response<Body>> for Response {
    fn from(value: http::Response<Body>) -> Self {
        Self(value)
    }
}

impl From<Response> for http::Response<Body> {
    fn from(value: Response) -> Self {
        value.0
    }
}

pub trait IntoResponse {
    /// Create a response.
    #[must_use]
    fn into_response(self) -> Response;
}

// impl From<Response> for Response {
//     fn from(value: Response) -> Self {
//         todo!()
//     }
// }

#[derive(Debug)]
#[must_use]
pub struct ErrorResponse(Response);

impl<T> From<T> for ErrorResponse
where
    T: IntoResponse,
{
    fn from(value: T) -> Self {
        Self(value.into_response())
    }
}

impl<T> IntoResponse for Result<T, ErrorResponse>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Ok(ok) => ok.into_response(),
            Err(err) => err.0,
        }
    }
}

// Responder is a wrapper around a type that implements IntoResponse.
// This allows us to implement IntoResponse for any type that implements IntoResponse.
// This is useful for abstracting over different response types.
pub struct Responder<T: IntoResponse>(pub T);

impl<T> IntoResponse for Responder<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}

// impl<T> ActixResponder for Responder<T>
// where
//     T: IntoResponse,
// {
//     // TODO: Impl message body for body
//     type Body = Body;
//
//     fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
//         self.into_response().into()
//     }
// }

// impl<T> AxumIntoResponse for Responder<T>
// where
//     T: IntoResponse,
// {
//     // todo impl type converters from local respose to axum respose
//     fn into_response(self) -> axum::response::Response {
//         self.0.into_response().into()
//     }
// }

// impl<T> From<axum::response::Response> for Responder<T>
// where
//     T: IntoResponse,
// {
//     fn from(value: axum::response::Response) -> Self {
//
//     }
// }

// impl<T> From<Responder<T>> for axum::response::Response
// where
//     T: IntoResponse,
// {
//     fn from(value: Responder<T>) -> Self {
//         IntoResponse::into_response(value).into()
//         // value.into_response().into()
//     }
// }
//
// impl From<crate::http::body::Body> for axum::body::Body {
//     fn from(value: crate::http::body::Body) -> Self {
//         todo!()
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
