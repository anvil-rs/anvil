async fn index() {
    Ok("Hello, World!")
}

fn routes() -> Vec<rocket::Route> {
    routes![index]
}
