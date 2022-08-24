use actix_web::{post, web, App, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    str: String,
    num: i8,
    arr: Vec::<i8>,
}

#[post("/postjson")]
async fn index(info: web::Json<Info>) -> String {
    format!("str: {} num {} arr: {:?}", info.str, info.num, info.arr)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
