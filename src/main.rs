use axum::{extract::Path, routing::get, Json, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/test", get(test))
        .route("/user/:user_id", get(user));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test() -> String {
    "This is a test function".to_string()
}

async fn user(Path(user_id): Path<u32>) -> Json<Value> {
    Json(json!({
        "user_id": user_id,
        "hello": "world",
    }))
}
