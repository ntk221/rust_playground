use axum::{
    routing::{get, post},
    Router, Json,
    http::StatusCode,
    response::IntoResponse
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::env;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
    env::set_var("RUST_LOG", &log_level);
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // std::convert::Fromトレイトを実装している. Fromは，引数にとった値を自身の型に変換する
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser> ) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct CreateUser {
    username: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    username: String
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        assert_eq!(bytes, "Hello, World!");
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"username":"foo"}"#))
            .unwrap();
        let res = create_app().oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body)
            .expect("failed to deserialize body");
        assert_eq!(
            user, 
            User {
                id: 1337,
                username: "foo".to_owned()
            }
        );
    }
}