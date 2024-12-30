pub mod rejection;

use bytes::Bytes;
use rejection::{BytesRejection, FailedToBufferBody};

use crate::http::request::FromRequest;

// impl<S> FromRequest<S> for Bytes {
//     type Error = BytesRejection;
//
//     async fn from_request(
//         req: crate::http::request::Request,
//         state: &S,
//     ) -> Result<Self, Self::Error> {
//         let bytes = req
//             .0
//             .into_limited_body()
//             .collect()
//             .await
//             .map_err(FailedToBufferBody::from_err)?
//             .to_bytes();
//     }
// }
