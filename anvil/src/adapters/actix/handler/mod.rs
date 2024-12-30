use std::future::Future;

use actix_web::{Handler as ActixHandler, Responder};

use crate::handler::{Handle, Handler};

impl<F, Args> ActixHandler<Args> for Handle<F, Args>
where
    F: Handler<Args> + Sized + Send + Sync + 'static,
    F::Output: Responder + Sized + 'static,
    Args: Clone + Send + Sync + 'static,
{
    type Output = F::Output;
    type Future = std::pin::Pin<Box<dyn Future<Output = Self::Output> + Send + 'static>>;

    fn call(&self, args: Args) -> Self::Future {
        let handler = self.0.clone();
        Box::pin(async move { handler.call(args).await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::StatusCode,
        test,
        web::{get, Query},
        App,
    };
    use serde::Deserialize;

    async fn handler() -> String {
        "Hello, world!".to_string()
    }

    #[actix_web::test]
    async fn test_handler() {
        let local_handler = Handle::new(handler);
        let app = App::new().route("/", get().to(local_handler));
        let app = test::init_service(app).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
        let body = test::read_body(res).await;
        assert_eq!(body, "Hello, world!");
    }

    async fn handler_with_args(arg: String) -> String {
        format!("Hello, {}!", arg)
    }

    #[actix_web::test]
    async fn test_handler_with_args() {
        let local_handler = Handle::new(handler_with_args);
        let app = App::new().route("/test/{arg}", get().to(local_handler));
        let app = test::init_service(app).await;
        let req = test::TestRequest::get()
            .uri("/test/world")
            .set_payload("world")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
        let body = test::read_body(res).await;
        assert_eq!(body, "Hello, world!");
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

    #[actix_web::test]
    async fn test_handler_with_extractors() {
        let local_handler = Handle::new(handler_with_extractors);
        let app = App::new().route("/test", get().to(local_handler));
        let app = test::init_service(app).await;
        let req = test::TestRequest::get()
            .uri("/test?name=world&age=42")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
        let body = test::read_body(res).await;
        assert_eq!(body, "Hello, world! You are 42 years old");
    }

    #[derive(Clone)]
    struct Test;

    impl Handler<()> for Test {
        type Output = String;

        async fn call(&self, _: ()) -> Self::Output {
            "Hello World!".to_string()
        }
    }

    #[actix_web::test]
    async fn test_handler_with_custom_handle_impl() {
        let local_handler: Handle<Test, ()> = Handle::new(Test);
        let app = App::new().route("/", get().to(local_handler));
        let app = test::init_service(app).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
        let body = test::read_body(res).await;
        assert_eq!(body, "Hello World!");
    }

    impl Handler<String> for Test {
        type Output = String;
        async fn call(&self, arg: String) -> Self::Output {
            format!("Hello, {}!", arg)
        }
    }

    #[actix_web::test]
    async fn test_handler_with_custom_handle_impl_with_args() {
        let local_handler: Handle<Test, String> = Handle::new(Test);
        let app = App::new().route("/{arg}", get().to(local_handler));
        let app = test::init_service(app).await;
        let req = test::TestRequest::get()
            .uri("/world")
            .set_payload("world")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
        let body = test::read_body(res).await;
        assert_eq!(body, "Hello, world!");
    }
}
