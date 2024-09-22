struct Response {}

trait IntoResponse {
    /// Create a response.
    fn into_response(self) -> Response;
}

impl IntoResponse for () {
    fn into_response(self) -> Response {
        Response {}
    }
}

trait Controller {
    fn index() -> impl IntoResponse {}
    fn show() -> impl IntoResponse {}
    fn store() -> impl IntoResponse {}
    fn create() -> impl IntoResponse {}
    fn edit() -> impl IntoResponse {}
    fn update() -> impl IntoResponse {}
    fn delete() -> impl IntoResponse {}
}

trait Request {}

struct Routes {
    prefix: Option<String>,
    handlers: Vec<Handler>,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
    ANY,
}

struct Handler {
    uri: String,
    method: Method,
}
