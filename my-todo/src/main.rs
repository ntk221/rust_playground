mod repositories;
mod handlers;

use crate::repositories::{TodoRepository, TodoRepositoryForMemory};
use handlers::create_todo;

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::{
    env,
    sync::{Arc},
};


#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
    env::set_var("RUST_LOG", &log_level);
    tracing_subscriber::fmt::init();

    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // std::convert::Fromトレイトを実装している. Fromは，引数にとった値を自身の型に変換する
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>))
        .layer(Extension(Arc::new(repository))) // axumアプリケーション内でrepositoryを共有する
}

async fn root() -> &'static str {
    "Hello, World!"
}


#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        assert_eq!(bytes, "Hello, World!");
    }

}