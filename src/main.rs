use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize}; // 修正: 正しいインポート

#[derive(Serialize)] // シリアライズ用トレイトを実装
struct MyResponse {
    message: String,
    code: i32,
}

async fn index() -> impl Responder {
    let response = MyResponse {
        message: "Hello, Actix!".to_string(),
        code: 200,
    };
    HttpResponse::Ok().json(response) // JSONレスポンスを返す
}

#[tokio::main] // 修正: 非同期ランタイムのアトリビュート
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
