use std::future::Future;

pub struct Extractor<T>(pub T);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Path<T>(pub T);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Query<T>(pub T);

#[derive(Debug)]
pub struct Json<T>(pub T);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Form<T>(pub T);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Header<T>(pub T);

/// The extension extractor is different from the others.
/// We need to implement it differently from the others, as it is impossible to map from a foreign
/// type's extension into our own extension type.
/// Therefore we must implement extension (or state/data) for each framework.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Extensions<T>(pub T);

// Now we need a generic-enough value extractor.
// Axum Handler === Actix Responder In actix, we deal with responders on a user-level.

// For Method, might be a good idea to create bindings to http::Method. Then implement Actix's
// FromRequst trait. Same here is true for Uri.
// FACT CHECK: Axum's stuff is just a re-export of http crate. And actix is identicle to the http
// create implementation. Would be nice to just be able to convert between the two.

// Path - Extracts the request's path. Including variables.
// Query - Extracts the query string from the request.
// JSON - Extracts and deserializes JSON from the request's body.
// Form - Extracts and deserializes form data from the request's body.
// Headers - Extracts headers from the request.
// Data - For accessing pieces of application state. Could also be called state. In axum this is
//      essentially the Extension struct.
// HttpRequest - HttpRequest is itself an extractor, in case you need access to other parts of the request.
// String - You can convert a request's payload to a String. An example is available in the rustdoc.
// Bytes - You can convert a request's payload into Bytes. An example is available in the rustdoc.
// Payload - Low-level payload extractor primarily for building other extractors. An example is available in the rustdoc.

// For some sort of General implementation of extractors for models, we can leverage the fact that
// a PATH extractor with #[Deserialize] can be used to extract only a single param, even if
// multiple params are there. This means that we can extract a struct with the param name. This
// means that we can create an extractor for a model using the models name as the field that needs
// to be extracted. Then we can create the abstraction, and
