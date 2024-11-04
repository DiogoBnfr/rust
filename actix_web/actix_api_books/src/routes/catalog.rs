use actix_web::HttpResponse;

pub async fn catalog() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(
            r#"
            "books": [
            {"id":"101", "title": "C++ Primer Plus", "author": "Stephen Prata"},
            {"id":"102", "title": "Mastering Emacs", "author": "Mickey Petersen"},
            {"id":"103", "title": "The C Programming Language", "author": "Brian W. Kernighan"}
            ]"#,
        )
}
