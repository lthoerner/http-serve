use std::net::{Ipv4Addr, SocketAddr};

use axum::body::Bytes;
use axum::response::Html;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(foo))
        .route("/sample_video.mp4", get(serve_video));

    let socket = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn foo() -> Html<&'static str> {
    Html(include_str!("../assets/foo.html"))
}

async fn serve_video() -> Bytes {
    Bytes::from_static(include_bytes!("../assets/sample_video.mp4"))
}
