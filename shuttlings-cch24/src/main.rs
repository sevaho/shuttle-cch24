use axum::{
    routing::get,
    Router,
    response::Response,
    http::{StatusCode, header::LOCATION},
};

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

async fn seek_302() -> Response {
    let redirect_url = "https://www.youtube.com/watch?v=9Gc4QTqslN4";

    // Build the 302 response
    Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, redirect_url)
        .body(axum::body::Body::empty())
        .unwrap()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/-1/seek", get(seek_302))
        .route("/", get(hello_world));

    Ok(router.into())
}
