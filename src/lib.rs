//! # Anvil
//! Anvil is a web framework for Rust that aims to be simple, fast, and flexible.
//! ## Ethos
//! - Configuration in code.
//! - User level abstraction should be minimal.
//! - Each component should be interchangeable with __no__ code change.
//! - The library provides the building blocks, not the solutions.
//! - Rely on other's implementations.

#[doc(no_inline)]
pub use async_trait::async_trait;

/// Backends for Anvil.
pub mod adapters;

/// Router and routing
pub mod routes;

/// Core http implementations.
pub mod http;

/// Extractors for requests.
pub mod extractors;

/// Handlers and Handle
pub mod handler;

/// Error handling.
pub mod error;

/// Middleware
pub mod middleware;

// trait Controller {
//     fn index() -> impl IntoResponse {}
//     fn show() -> impl IntoResponse {}
//     fn store() -> impl IntoResponse {}
//     fn create() -> impl IntoResponse {}
//     fn edit() -> impl IntoResponse {}
//     fn update() -> impl IntoResponse {}
//     fn delete() -> impl IntoResponse {}
// }
