use crate::http::body::Body;

pub type Request<T = Body> = http::Request<T>;
