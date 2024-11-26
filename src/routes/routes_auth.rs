pub mod routes_auth{
    use actix_web::{web};
    use crate::service::auth::auth_forgot_send_otp::auth_forgot_send_otp::auth_forgot_send;
    use crate::service::auth::auth_jwt_genrator::auth_jwt_genrator::{auth_access_token, auth_refresh_token};
    use crate::service::auth::auth_login::auth_login::auth_login;
    use crate::service::auth::auth_logout::auth_logout::auth_logout;
    use crate::service::auth::auth_send_otp::auth_send_otp::auth_send_otp;
    use crate::service::auth::auth_signup::auth_signup::auth_signup;
    use crate::service::auth::auth_verify_forgot_otp::auth_verify_forgot_otp::{ auth_forgot_verify_otp};
    use crate::service::auth::auth_verify_otp::auth_verify_otp::auth_verify_otp;

    pub fn routes_auth() ->actix_web::Scope{
        web::scope("/auth")
            .route("/login",web::post().to(auth_login))
            .route("/signup",web::post().to(auth_signup))
            .route("/logout",web::post().to(auth_logout))
            .route("/send_otp",web::post().to(auth_send_otp))
            .route("/verify_otp",web::post().to(auth_verify_otp))
            .route("/forgot_send",web::post().to(auth_forgot_send))
            .route("/forgot_verify",web::post().to(auth_forgot_verify_otp))
            .route("/accestoken",web::post().to(auth_access_token))
            .route("/refreshtoken",web::post().to(auth_refresh_token))
    }
}