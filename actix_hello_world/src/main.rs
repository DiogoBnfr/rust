use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("REQUEST::DATA: {}", req_body);
    println!("HTTP::RESPONSE::OK");
    if req_body == "Hello, I'm the user!" {
        HttpResponse::Ok().body("Hello, user!")
    } else {
        HttpResponse::Ok().body("Who are you?")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};

    #[actix_rt::test]
    async fn test_echo() {
        let mut app = test::init_service(
            App::new().service(echo)  // Register the handler
        ).await;

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_payload("Hello, Actix!")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(),http::StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, Actix!");
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
