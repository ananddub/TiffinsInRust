use actix_web::{web, App, HttpServer};
use backend::routes::auth::auth::routes_auth;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string()) // Default port agar `PORT` set na ho
        .parse()
        .expect("PORT must be a valid u16 integer");

    println!("Starting server at http://127.0.0.1:{}", port);

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(routes_auth())
            )
    })
        .bind(("127.0.0.1", port))? // Dynamic port binding
        .run()
        .await
}
