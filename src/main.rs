use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 bismillah run http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
