use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive( Serialize, Deserialize)]
struct MyObj {
    //uuid: String,
    //results: Value,
    results: Vec<Value>,
}

async fn index(item: web::Json<MyObj>) -> HttpResponse {

    //println!("{:?}", item.results);
    println!("{:?}", item.results.get(0));
    HttpResponse::Ok().json(item.0) // <- send response
}

#[post("/alprd1")]
async fn alprd1(req_body: String) -> impl Responder {
    //let uuid = &req_body[541..578];
    //let plate = &req_body[214..244];
    //println!("{}", uuid);
    //println!("slice = {}", slice);
    println!("Handling POST request: {req_body}");
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/images", "static/images/").show_files_listing())
            .service(Files::new("/video", "./static/videos/").index_file("alprVideo1.mp4"))
            .service(web::resource("/alprd2").route(web::post().to(index)))
            .service(alprd1)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
