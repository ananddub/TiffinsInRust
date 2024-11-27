use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_limitation::Limiter;
use dotenv::dotenv;
use backend::connection::dbconection::db_conection::{check_db_status, check_rdb_status, redis_con, DB, RDB};
use backend::middleware::db_conn_middleware::db_conn_middleware::db_con_middleware;
use backend::middleware::logmiddlware::loginmiddlware::logmiddlware;
use backend::routes::routes_auth::routes_auth::routes_auth;
use backend::routes::routes_user::routes_user::routes_user;

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
    check_db_status().await;
    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(2)
        .burst_size(5)
        .finish()
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(routes_auth())
                    .service(routes_user())
            )
            .wrap(Governor::new(&governor_conf))
            .route("/",web::get().to(home))
            .wrap(actix_web::middleware::from_fn(logmiddlware))
            .wrap(actix_web::middleware::from_fn(db_con_middleware))
        })
        .bind(("0.0.0.0", port))?
        .workers(6)
        .run()
        .await
}


async fn home()-> impl Responder   {
    HttpResponse::Ok().body("Hello From Rust")
}