use actix_web::{HttpServer, App, web, HttpResponse};

const SERVER_ADDR: &str = "127.0.0.1:8080";

pub fn main() {
    HttpServer::new(|| {
        App::new()
          .route("/", web::get().to(|| HttpResponse::Ok().content_type("text/html; charset=utf-8").body(include_str!("./index.html"))))
    })
      .bind(SERVER_ADDR)
      .unwrap()
      .run()
      .unwrap();
}
