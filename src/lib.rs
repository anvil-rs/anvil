//! # Swivel
//! Swivel is a web framework for Rust that aims to be simple, fast, and flexible.
//! ## Ethos
//! - Configuration in code.
//! - User level abstraction should be minimal.
//! - Each component should be interchangeable with __no__ code change.
//! - The library provides the building blocks, not the solutions.
//! - Rely on other's implementations.

pub mod backends;

pub mod routes;

pub mod http;

pub mod extractors;

/// Handlers and Handle
pub mod handler;

pub mod error;
// struct Response {}
//
// trait IntoResponse {
//     /// Create a response.
//     fn into_response(self) -> Response;
// }
//
// impl IntoResponse for () {
//     fn into_response(self) -> Response {
//         Response {}
//     }
// }
//
// trait Controller {
//     fn index() -> impl IntoResponse {}
//     fn show() -> impl IntoResponse {}
//     fn store() -> impl IntoResponse {}
//     fn create() -> impl IntoResponse {}
//     fn edit() -> impl IntoResponse {}
//     fn update() -> impl IntoResponse {}
//     fn delete() -> impl IntoResponse {}
// }
//
// struct Test;
//
// impl Controller for Test {
//     fn index() -> () {
//         ()
//     }
// }
//
// trait Request {}
//
// struct Routes {
//     prefix: Option<String>,
//     handlers: Vec<Handler>,
// }
//
// enum Method {
//     GET,
//     POST,
//     PUT,
//     DELETE,
//     PATCH,
//     OPTIONS,
//     HEAD,
//     CONNECT,
//     TRACE,
//     ANY,
// }
//
// struct Handler {
//     uri: String,
//     method: Method,
// }
