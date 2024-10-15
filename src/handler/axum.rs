use std::{future::Future, pin::Pin};

use axum::{
    extract::{
        FromRequest as AxumFromRequest, FromRequestParts as AxumFromRequestParts,
        Request as AxumRequest,
    },
    handler::Handler as AxumHandler,
    response::IntoResponse as AxumIntoResponse,
};

use super::{Handle, Handler};

macro_rules! impl_handler {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<T, S, M, $($ty,)* $last> AxumHandler<(M, $($ty,)* $last,), S> for Handle<T, ($($ty,)* $last,)>
        where
            T: Handler<($($ty,)* $last,)> + Clone,
            T::Output: AxumIntoResponse + Send ,
            T::Future: Future<Output = T::Output> + Send,
            S: Send + Sync + 'static,
            $( $ty: AxumFromRequestParts<S> + Send + 'static + Clone,)*
            $last: AxumFromRequest<S, M> + Send + 'static + Clone,
        {
            type Future = Pin<Box<dyn Future<Output = axum::response::Response> + Send>>;

            fn call(self, req: AxumRequest, state: S) -> Self::Future {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*

                    let req = AxumRequest::from_parts(parts, body);

                    let $last = match $last::from_request(req, state).await {
                        Ok(value) => value,
                        Err(rejection) => return rejection.into_response(),
                    };

                    Handler::call(&self.0, ($($ty,)* $last,)).await.into_response()
                })
            }
        }
    };
}

// Initially implemented for 0 arguments
impl<T, S> AxumHandler<((),), S> for Handle<T, ()>
where
    T: Handler<()>,
    T::Output: AxumIntoResponse + Send,
    T::Future: Future<Output = T::Output> + Send,
    S: Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = axum::response::Response> + Send>>;

    fn call(self, _: AxumRequest, _: S) -> Self::Future {
        Box::pin(async move { Handler::call(&self.0, ()).await.into_response() })
    }
}

impl_handler!([], T1);
impl_handler!([T1], T2);
impl_handler!([T1, T2], T3);
impl_handler!([T1, T2, T3], T4);
impl_handler!([T1, T2, T3, T4], T5);
impl_handler!([T1, T2, T3, T4, T5], T6);
impl_handler!([T1, T2, T3, T4, T5, T6], T7);
impl_handler!([T1, T2, T3, T4, T5, T6, T7], T8);
impl_handler!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
impl_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
impl_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
impl_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
impl_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
impl_handler!(
    [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13],
    T14
);
impl_handler!(
    [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14],
    T15
);
impl_handler!(
    [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15],
    T16
);

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        extract::Query,
        http::{Request, StatusCode},
        routing::get,
    };
    use http_body_util::BodyExt;
    use serde::Deserialize;
    use tower::ServiceExt;

    use super::*;

    async fn handler() -> String {
        "Hello, world!".to_string()
    }

    #[tokio::test]
    async fn basic_handler() {
        let local_handler = Handle::new(handler);

        let app = axum::Router::new().route("/hello", get(local_handler));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, world!");
    }

    async fn handler_with_args(arg: String) -> String {
        format!("Hello, {}!", arg)
    }

    #[tokio::test]
    async fn handle_with_args() {
        let local_handler = Handle::new(handler_with_args);

        let app = axum::Router::new().route("/hello/:name", get(local_handler));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello/world")
                    .body(Body::from("world"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, world!");
    }

    #[derive(Deserialize, Clone)]
    struct Info {
        name: String,
        age: i32,
    }

    async fn handler_with_extractors(info: Query<Info>) -> String {
        let info: Info = info.0;
        format!("Hello, {}! You are {} years old", info.name, info.age)
    }

    #[tokio::test]
    async fn handle_with_extractors() {
        let local_handler = Handle::new(handler_with_extractors);
        let app = axum::Router::new().route("/hello", get(local_handler));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello?name=world&age=42")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, world! You are 42 years old");
    }

    #[derive(Clone)]
    struct Test;

    impl Handler<()> for Test {
        type Output = String;

        type Future = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

        fn call(&self, _: ()) -> Self::Future {
            Box::pin(async move { "Hello World!".to_string() })
        }
    }

    #[tokio::test]
    async fn test_handler_with_custom_handle_impl() {
        let local_handler = Handle::new(Test);
        let app = axum::Router::new().route("/", get(local_handler));
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello World!");
    }
}
