use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index(web::Path(()): web::Path<()>) -> impl Responder {
    format!("Hello App Runner!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
