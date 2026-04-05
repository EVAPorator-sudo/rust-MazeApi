use MazeApi::*;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = app("generate");
    let address: SocketAddr = "0.0.0.0:3080".parse().unwrap();
    let listener: TcpListener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
