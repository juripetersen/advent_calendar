use actix_cors::Cors;
use actix_web::{App, HttpServer};

pub mod routes;
pub mod winners;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting HTTP Server on localhost:7777");

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).configure(routes::configure)
    })
    .bind(("0.0.0.0", 7777))?
    .run()
    .await
}
