use actix_web::{App, HttpServer};
use std::io;
// router
mod model;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().configure(model::cfg_route))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
