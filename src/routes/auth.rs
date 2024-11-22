pub mod auth{
    use actix_web::{web};
    use crate::service::auth::auth::auth::{forgot, login, signup};

    pub fn routes_auth() ->actix_web::Scope{
        web::scope("/auth")
            .route("/login",web::get().to(login))
            .route("/signup",web::get().to(signup))
            .route("/logout",web::get().to(login))
            .route("/forgot",web::get().to(forgot))
    }
}