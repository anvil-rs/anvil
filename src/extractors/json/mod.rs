pub mod rejection;

use bytes::Bytes;
use http::{header, HeaderMap};
use rejection::{JsonRejection, MissingJsonContentType};
use serde::de::DeserializeOwned;

use crate::http::request::{FromRequest, Request};

#[derive(Debug, Clone, Copy, Default)]
pub struct Json<T>(pub T);

// impl<T, S> FromRequest<S> for Json<T>
// where
//     T: DeserializeOwned,
//     S: Send + Sync,
// {
//     type Error = JsonRejection;
//
//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Error> {
//         if json_content_type(req.headers()) {
//             let bytes = Bytes::from_request(req, state).await?;
//             Self::from_bytes(&bytes)
//         } else {
//             Err(MissingJsonContentType.into())
//         }
//     }
// }

fn json_content_type(headers: &HeaderMap) -> bool {
    let content_type = if let Some(content_type) = headers.get(header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };

    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().map_or(false, |name| name == "json"));

    is_json_content_type
}

impl<T> From<T> for Json<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

// impl<T> Json<T>
// where
//     T: DeserializeOwned,
// {
//     /// Construct a `Json<T>` from a byte slice. Most users should prefer to use the `FromRequest` impl
//     /// but special cases may require first extracting a `Request` into `Bytes` then optionally
//     /// constructing a `Json<T>`.
//     pub fn from_bytes(bytes: &[u8]) -> Result<Self, JsonRejection> {
//         let deserializer = &mut serde_json::Deserializer::from_slice(bytes);
//
//         let value = match serde_path_to_error::deserialize(deserializer) {
//             Ok(value) => value,
//             Err(err) => {
//                 let rejection = match err.inner().classify() {
//                     serde_json::error::Category::Data => JsonDataError::from_err(err).into(),
//                     serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
//                         JsonSyntaxError::from_err(err).into()
//                     }
//                     serde_json::error::Category::Io => {
//                         if cfg!(debug_assertions) {
//                             // we don't use `serde_json::from_reader` and instead always buffer
//                             // bodies first, so we shouldn't encounter any IO errors
//                             unreachable!()
//                         } else {
//                             JsonSyntaxError::from_err(err).into()
//                         }
//                     }
//                 };
//                 return Err(rejection);
//             }
//         };
//
//         Ok(Json(value))
//     }
// }
