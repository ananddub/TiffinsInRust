use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use backend::routes::auth::auth::routes_auth;
use std::env;
use dotenv::dotenv;
use backend::connection::dbconection::db_conection::{check_rdb_status, redis_con, DB, RDB};
use backend::middleware::logmiddlware::loginmiddlware::log;
use actix_web::middleware::{self, Next};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string()) // Default port agar `PORT` set na ho
        .parse()
        .expect("PORT must be a valid u16 integer");

        println!("Starting server at http://0.0.0.0:{}", port);
    check_rdb_status().await;
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(routes_auth())
                    .wrap(actix_web::middleware::from_fn(log))
            )
            .route("/",web::get().to(home))
        })
        .bind(("0.0.0.0", port))?

        .workers(6)
        .run()
        .await
}


async fn home()-> impl Responder   {
    HttpResponse::Ok().body("Hello From Rust")
}