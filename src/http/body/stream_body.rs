use std::{
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Buf;
use futures_util::Stream;
use http_body::{Body, Frame};
use pin_project_lite::pin_project;

pin_project! {
    /// A body created from a [`Stream`].
    pub struct StreamBody<S> {
        #[pin]
        stream: S,
    }
}

impl<S: Stream> StreamBody<S> {
    /// Create a new `StreamBody`.
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
}

impl<S, D, E> Body for StreamBody<S>
where
    S: Stream<Item = Result<Frame<D>, E>>,
    D: Buf,
{
    type Data = D;
    type Error = E;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        match self.project().stream.poll_next(cx) {
            Poll::Ready(Some(result)) => Poll::Ready(Some(result)),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
