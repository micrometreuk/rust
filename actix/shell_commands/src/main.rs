use actix_files::Files;
use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}


async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/alprd", web::post().to(index))
            .service(Files::new("/video", "./static/videos/").index_file("alprVideo1.mp4"))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
