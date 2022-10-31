use axum::{routing::get, Router};
use backend_rs::JSONLoader;
use backend_rs::Loader;
use backend_rs::TaskList;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "welcome" }))
        .route("/tasks", get(get_all_tasks));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_all_tasks() -> TaskList {
    let json_filepath = "/Users/lcrown/temp/tasklist.json";
    let loader = JSONLoader::new(&json_filepath);
    loader.load().unwrap()
}
