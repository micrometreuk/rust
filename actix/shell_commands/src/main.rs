use actix_web::middleware::Logger;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| 
        App::new()
        .service(hello)
        .service(echo)
        .wrap(Logger::default()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
