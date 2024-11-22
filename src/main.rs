use actix_web::{web, App, HttpServer};
use backend::routes::auth::auth::{routes_auth};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("Starting server at http://127.0.0.1:4000");
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(routes_auth())
            )
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
