use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::get;
use axum::{middleware, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

fn token_is_valid(auth: &str) -> bool {
    if auth == "hello" {
        return true;
    }
    false
}

fn token2_is_valid(auth: &str) -> bool {
    if auth == "world" {
        return true;
    }
    false
}

async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(auth_header) if token_is_valid(auth_header) => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn auth2(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("key")
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(auth_header) if token2_is_valid(auth_header) => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn hello() -> &'static str {
    "Hello"
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/hello", get(hello))
        .route_layer(middleware::from_fn(auth))
        .route_layer(middleware::from_fn(auth2));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
