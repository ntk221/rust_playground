mod repositories;
mod handlers;

use crate::repositories::{TodoRepository, TodoRepositoryForMemory};
use handlers::{all_todos, create_todo, delete_todo, find_todo, update_todo};

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
        .route("/", get(root)
        )
        .route("/todos", 
            post(create_todo::<T>)
                    .get(all_todos::<T>)
        )
        .route("/todos/:id",
            get(find_todo::<T>)
                    .patch(update_todo::<T>)
                    .delete(delete_todo::<T>
            )
        )
        .layer(Extension(Arc::new(repository))) // axumアプリケーション内でrepositoryを共有する
}

async fn root() -> &'static str {
    "Hello, World!"
}


#[cfg(test)]
mod test {
    use super::*;
    use axum::response::Response;
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
    };
    use crate::repositories::{CreateTodo, Todo};
    use tower::ServiceExt;


    // test用のhelper
    fn build_todo_req_with_json(path: &str, method: Method, json_body: String) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json_body))
            .unwrap()
    }

    fn build_todo_req_with_empty(method: Method, path: &str) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .body(Body::empty())
            .unwrap()
    }

    async fn res_to_todo(res: Response) -> Todo {
            let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
            let body: String = String::from_utf8(bytes.to_vec()).unwrap();
            let todo: Todo = serde_json::from_str(&body)
                .expect(&format!("cannot convert Todo instance. body:{}", body));
            todo
        }

    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        assert_eq!(bytes, "Hello, World!");
    }

    #[tokio::test]
    async fn should_created_todo() {
        let expected = Todo::new(1, "should_return_created_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        let req = build_todo_req_with_json("/todos", Method::POST, r#"{"text": "should_return_created_todo" }"#.to_string());
        let res = create_app(repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(todo, expected);
    }

    #[tokio::test]
    async fn should_find_todo() {
        let expected = Todo::new(1, "should_find_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("should_find_todo".to_string()));
        let req = build_todo_req_with_empty(Method::GET, "/todos/1");
        let res = create_app(repository).oneshot(req).await.unwrap();
        println!("{:?}", res);
        let todo = res_to_todo(res).await;
        assert_eq!(todo, expected);
    }

    #[tokio::test]
    async fn should_get_all_todos() {
        let expected = Todo::new(1, "should_get_all_todos".to_string());
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("should_get_all_todos".to_string()));

        let req = build_todo_req_with_empty(Method::GET, "/todos");
        let res = create_app(repository).oneshot(req).await.unwrap();
        // TodoのVecを取得したいので，res_to_todoを使えない
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Vec<Todo> = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instance. body:{}", body));
        assert_eq!(vec![expected], todo);
    }

    #[tokio::test]
    async fn should_update_todo() {
        let expected = Todo::new(1, "should_update_todo".to_string());

        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("should_update_todo".to_string()));
        let req = build_todo_req_with_json(
            "/todos/1", 
            Method::PATCH, 
            r#"{"text": "should_update_todo",
            "completed": false
            }"#.to_string()
        );

        let res = create_app(repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(todo, expected);
    }

    #[tokio::test]
    async fn should_delete_todo() {
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("should_delete_todo".to_string()));
        let req = build_todo_req_with_empty(Method::DELETE, "/todos/1");

        let res = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT)
    }

}