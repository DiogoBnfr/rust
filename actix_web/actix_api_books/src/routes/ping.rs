use actix_web::*;

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("ROUTE: ping")
}
