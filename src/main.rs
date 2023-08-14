use std::net::{Ipv4Addr, SocketAddr};

use axum::body::Bytes;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};
use tokio::fs::File;

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

async fn serve_video() -> impl IntoResponse {
    let file = match File::open("assets/sample_video.mp4").await {
        Ok(file) => file,
        Err(_) => {
            let response = Html("File not found");
            return Err(response);
        }
    };

    todo!()
}
