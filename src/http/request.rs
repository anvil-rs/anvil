use std::{convert::Infallible, future::Future};

use async_trait::async_trait;

use crate::http::body::Body;

use super::response::IntoResponse;

pub struct Request<T = Body>(http::Request<T>);

pub trait FromRequest<S>: Sized {
    type Error: IntoResponse;

    fn from_request(
        req: Request,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Error>> + Send;
}

impl<S> FromRequest<S> for Request
where
    S: Send + Sync,
{
    type Error = Infallible;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Error> {
        Ok(req)
    }
}
