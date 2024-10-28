use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::Context,
};

use actix_web::body::{BodySize, BoxBody, MessageBody};
use bytes::Bytes;
use http_body::{Body as HttpBody, Frame};

use crate::{
    error::Error,
    http::body::{boxed, Body},
};

impl MessageBody for Body {
    type Error = Error;

    fn size(&self) -> BodySize {
        match self.size_hint().exact() {
            // If we do know the size, set it.
            Some(size) => BodySize::Sized(size),
            // We are streaming by default, so we probably don't know the body size
            None => BodySize::Stream,
        }
    }

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> std::task::Poll<Option<Result<Bytes, Self::Error>>> {
        self.poll_frame(cx).map(|opt| {
            opt.map(|res| res.map(|e| Frame::into_data(e).expect("Frame should have data")))
        })
    }
}

struct BoxBodyWrapper(BoxBody);

impl HttpBody for BoxBodyWrapper {
    type Data = Bytes;
    type Error = Error;
    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> std::task::Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        self.get_mut()
            .0
            .as_pin_mut()
            .poll_next(cx)
            .map(|opt| opt.map(|res| res.map_err(|e| Error::new(e.to_string())).map(Frame::data)))
    }
}

/// The `BoxBodyWrapper` is !Send by default, so we implement Send for it.
/// We will ensure that any error that is sent by actix is Send anyway.
unsafe impl Send for BoxBodyWrapper {}

/// Convert an [`actix_web::body::BoxBody`] into an [`Body`].
/// An Actix BoxBody is !Send by default, so we wrap it in a Mutex.
impl From<BoxBody> for Body {
    fn from(value: BoxBody) -> Self {
        let wrapper = BoxBodyWrapper(value);
        Body::new(boxed(wrapper))
    }
}

impl From<Body> for BoxBody {
    /// `Body` implements [`actix_web::body::MessageBody`] therefore we can construct a [`actix_web::body::BoxBody`]
    ///  from our own body type.
    ///
    /// [`actix_web::body::BoxBody`]: https://docs.rs/actix-web/latest/actix_web/body/struct.BoxBody.html
    /// [`actix_web::body::MessageBody`]: https://docs.rs/actix-web/latest/actix_web/body/trait.MessageBody.html
    fn from(value: Body) -> Self {
        BoxBody::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;

    #[actix_web::test]
    async fn test_actix_to_anvil_body_conversion() {
        let actix_body = actix_web::body::BoxBody::new("Hello, World!");
        let body: Body = actix_body.into();
        let bytes = to_bytes(body).await.unwrap();
        assert_eq!(bytes, "Hello, World!");
    }

    #[actix_web::test]
    async fn test_anvil_to_actix_body_conversion() {
        let body = Body::from("Hello, World!");
        let actix_body: actix_web::body::BoxBody = body.into();
        let bytes = to_bytes(actix_body).await.unwrap();
        assert_eq!(bytes, "Hello, World!");
    }
}
