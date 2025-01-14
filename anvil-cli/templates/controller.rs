async fn index() {}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("{{name|snakecase}}")
        .get("/", index)
}

