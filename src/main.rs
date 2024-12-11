use axum::{routing::get, Router};
mod data_process;

#[tokio::main]
async fn main() {

    // build application with a single route
    let app = Router::new().route("/", get(data_process::request_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    }
