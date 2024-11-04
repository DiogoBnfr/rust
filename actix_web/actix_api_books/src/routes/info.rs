use actix_web::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

pub async fn info() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<h1>VERSION: {}<h1><br><h2>AUTHORS: {}<h2>", VERSION, AUTHORS))
}
