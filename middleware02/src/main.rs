use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{middleware, Extension, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::cell::OnceCell;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

const USER_COOKIE_NAME: &'static str = "hello";
type SessionId = u64;
type User = u64;

#[derive(Clone)]
struct AuthState(Option<(SessionId, Arc<User>)>, PgPool);

impl AuthState {
    pub async fn get_user(&self) -> Option<String> {
        Some("0".to_string())
    }
}

// GET /home HTTP/1.1
// Host: example.com
// Cookie: name=first; name=second;
async fn auth(mut req: Request, next: Next, database: PgPool) -> axum::response::Response {
    let key_pair_opt = req
        .headers()
        .get("Cookie")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.split_once(';').map(|(left, _)| left).unwrap_or(value))
        .and_then(|kv| kv.split_once('='));

    let auth_state = if let Some((key, value)) = key_pair_opt {
        if key != USER_COOKIE_NAME {
            None
        } else if let Ok(value) = value.parse::<u64>() {
            Some(value)
        } else {
            None
        }
    } else {
        None
    };

    req.extensions_mut().insert(AuthState(
        auth_state.map(|v| (v, Arc::new(0))),
        database,
    ));

    next.run(req).await
}

async fn me(
    Extension(current_user): Extension<AuthState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Some(user) = current_user.get_user().await {
        Ok(user)
    } else {
        Err("Not logged in")
    }
}

#[tokio::main]
async fn main() {
    let db_url = env!("DATABASE_URL");
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .unwrap();

    let router = Router::new().layer(middleware::from_fn(move |req, next| {
        auth(req, next, db_pool.clone())
    }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
