use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("src/index.html")?)
}

fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
      .bind("127.0.0.1:8080")
      .unwrap()
      .run()
      .unwrap();
}
