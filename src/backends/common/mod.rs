pub trait IntoResponse<R> {
    /// Create a response.
    fn into_response(self) -> R;
}
