use std::{pin::Pin, sync::Mutex};

use actix_web::body::{BoxBody, MessageBody};
use bytes::Bytes;
use http_body::{Body as HttpBody, Frame};
use http_body_util::BodyExt;

use crate::{
    error::Error,
    http::body::{boxed, Body},
};

// pub struct ActixBody<T: MessageBody<Error = Error>>(T);

// impl<T: MessageBody<Error = Error>> From<ActixBody<T>> for Body {
//     fn from(value: ActixBody<T>) -> Self {
//         todo!()
//     }
// }

impl MessageBody for crate::http::body::Body {
    type Error = Error;

    fn size(&self) -> actix_web::body::BodySize {
        match self.size_hint().exact() {
            // If we do know the size, set it.
            Some(size) => actix_web::body::BodySize::Sized(size),
            // We are streaming by default, so we probably don't know the body size
            None => actix_web::body::BodySize::Stream,
        }
    }

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<Bytes, Self::Error>>> {
        self.poll_frame(cx).map(|opt| {
            opt.map(|res| res.map(|e| Frame::into_data(e).expect("Frame should have data")))
        })
    }
}

// impl<T> From<Body> for ActixBody<T>
// where
//     T: MessageBody<Error = Error> + Unpin + Send + 'static,
// {
//     fn from(value: Body) -> Self {
//         // todo!()
//         // ActixBody(value)
//     }
// }

// impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
//     fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
//         Error::new(e)
//     }
// }

// struct ActixBody<T: MessageBody>(T);
//
// #[derive(Debug)]
// struct ActixBoxBody(Mutex<actix_web::body::BoxBody>);

// impl<T> http_body::Body for ActixBody<T>
// where
//     T: MessageBody<Error = Error> + Unpin,
// {
//     type Data = Bytes;
//     type Error = Error;
//     fn poll_frame(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
//         Pin::new(&mut self.get_mut().0)
//             .poll_next(cx)
//             .map(|opt| opt.map(|res| res.map_err(Error::new).map(Frame::data)))
//     }
// }

// impl http_body::Body for ActixBoxBody {
//     type Data = Bytes;
//     type Error = Error;
//     fn poll_frame(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
//         Pin::new(&mut self.get_mut().0)
//             .lock()
//             .unwrap()
//             .as_pin_mut()
//             .poll_next(cx)
//             .map(|opt| opt.map(|res| res.map_err(|e| Error::new(e.to_string())).map(Frame::data)))
//         // .poll_next(cx)
//         // .map(|opt| opt.map(|res| res.map_err(|e| Error::new(e.to_string())).map(Frame::data)))
//     }
// }

// impl From<ActixBoxBody> for Body {
//     fn from(value: ActixBoxBody) -> Self {
//         Self::empty()
//         // Body::new()
//     }
// }

// impl<T> From<ActixBody<T>> for Body
// where
//     T: MessageBody<Error = Error> + Unpin + Send + 'static,
// {
//     fn from(value: ActixBody<T>) -> Self {
//         Body::new(boxed(value))
//     }
// }

// impl<T> From<Body> for ActixBody<T>
// where
//     T: MessageBody<Error = Error> + Unpin + Send + 'static,
// {
//     fn from(value: Body) -> Self {
//         ActixBody(value)
//     }
// }

// impl From<ActixBody> for crate::http::body::Body {
//     fn from(value: ActixBody) -> Self {
//         Body::new(boxed(value))
//     }
// }

// USE BOX BODY

#[cfg(test)]
mod test {

    use super::*;
    use http_body_util::BodyExt;

    #[actix_web::test]
    async fn test_actix_to_anvil_body_conversion() {
        // let actix_body = actix_web::body::Body::from("Hello, World!");
        // let body: Body = ActixBody(actix_body).into();
        // let bytes = body.collect().await.unwrap().to_bytes();
        // assert_eq!(bytes, "Hello, World!");
    }

    // #[tokio::test]
    // async fn test_anvil_to_axum_body_conversion() {
    //     let body = Body::from("Hello, World!".to_string());
    //     let axum_body: axum::body::Body = body.into();
    //
    //     let bytes = axum_body.collect().await.unwrap().to_bytes();
    //
    //     assert_eq!(bytes, "Hello, World!");
    // }
    //
    // #[tokio::test]
    // async fn test_axum_to_anvil_body_conversion() {
    //     let axum_body = axum::body::Body::from("Hello, World!".to_string());
    //     let body: Body = axum_body.into();
    //     let bytes = body.collect().await.unwrap().to_bytes();
    //     assert_eq!(bytes, "Hello, World!");
    // }
}
