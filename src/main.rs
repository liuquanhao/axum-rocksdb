mod errors;
mod handlers;
mod models;
mod kv;
mod server;

use handlers::{create_todo, delete_todo, get_todo, update_todo};
use models::todo_repo::TodoRepo;
use kv::RocksDB;
use std::env;

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(num_cpus::get())
        .build()
        .unwrap();
    rt.block_on(serve());
}

async fn serve() {
    let file_path = env::var("ROCKSDB_FILE").expect("需要ROCKSDB_FILE环境变量设置rocksdb库文件地址");
    let kv_db = RocksDB::new(&file_path).await;
    let todo_repo = TodoRepo::new(kv_db).await.to_dyn();

    let router = Router::new()
        .route("/todos/", post(create_todo))
        .route(
            "/todos/:id",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .layer(Extension(todo_repo));

    server::builder()
        .serve(router.into_make_service())
        .await
        .unwrap();
}