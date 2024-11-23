pub mod auth{
    use actix_web::{web};
    use crate::service::auth::auth::auth::{forgot, login, logout, resend_otp, signup};

    pub fn routes_auth() ->actix_web::Scope{
        web::scope("/auth")
            .route("/login",web::post().to(login))
            .route("/signup",web::post().to(signup))
            .route("/logout",web::post().to(logout))
            .route("/forgot",web::post().to(forgot))
            .route("/resendotp",web::post().to(resend_otp))
    }
}