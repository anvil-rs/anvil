pub struct Request {}

struct Error;

trait FromRequest: Sized {
    // TODO: Convert into IntoResponse?
    type Error: Into<Error>;
    async fn from_request(req: Request) -> Result<Self, Error>;
}
