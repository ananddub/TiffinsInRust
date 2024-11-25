pub mod routes_auth{
    use actix_web::{web};
    use crate::service::auth::auth_forgot::auth_forgot::auth_forgot;
    use crate::service::auth::auth_login::auth_login::auth_login;
    use crate::service::auth::auth_logout::auth_logout::auth_logout;
    use crate::service::auth::auth_send_otp::auth_send_otp::auth_send_otp;
    use crate::service::auth::auth_signup::auth_signup::auth_signup;
    use crate::service::auth::auth_verify_otp::auth_verify_otp::auth_verify_otp;

    pub fn routes_auth() ->actix_web::Scope{
        web::scope("/auth")
            .route("/login",web::post().to(auth_login))
            .route("/signup",web::post().to(auth_signup))
            .route("/logout",web::post().to(auth_logout))
            .route("/forgot",web::post().to(auth_forgot))
            .route("/send_otp",web::post().to(auth_send_otp))
            .route("/verify_otp",web::post().to(auth_verify_otp))
    }
}