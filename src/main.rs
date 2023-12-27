use actix_files as fs;
use actix_web::{App, HttpServer};
 
mod shared {
    pub mod templates;
} 

mod modules {
    pub mod index;
    pub mod blog;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(fs::Files::new("/styles", "./src/shared/styles").show_files_listing())
        .service(fs::Files::new("/blog/styles", "./src/modules/blog/styles").show_files_listing())
        .service(fs::Files::new("/index/styles", "./src/modules/index/styles").show_files_listing())
        .configure(modules::index::routes::init)
            .configure(modules::blog::routes::init)
    })
    .bind("127.0.0.2:8080")?
    .run()
    .await
}
