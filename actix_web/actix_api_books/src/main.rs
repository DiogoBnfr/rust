use actix_web::*;

mod routes;
use routes::ping::*;
use routes::info::*;
use routes::catalog::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api = HttpServer::new(|| {
        App::new()
        .route("/ping", web::get().to(ping))
        .route("/info", web::get().to(info))
        .route("/catalog", web::get().to(catalog))
    });

    let api = api.bind(format!("127.0.0.1:9091"))
        .expect("CONNECTION ERROR!");

    println!("CONNECTED TO: http://localhost:9091");

    api.run().await
}

