pub mod auth_logout {
    use crate::service::token::token_service::{access_token, refresh_token, TokenStruct};
    use actix_web::{HttpResponse, Responder};
    use serde::{Deserialize, Serialize};
    use std::time::SystemTime;
    use validator::Validate;

    #[derive(Deserialize, Serialize, Debug, Validate)]
    pub struct SignupBody {
        #[validate(length(min = 1))] // Optional field must have at least 1 character if present
        pub image: Option<String>,
        #[validate(length(min = 1))] // Ensure non-empty string
        pub username: String,
        #[validate(length(min = 6, max = 32))] // Password length validation
        pub password: String,
        #[validate(email)] // Email validation
        pub email: String,
    }

    #[derive(Deserialize, Serialize, Debug, Validate)]
    pub struct LoginBody {
        #[validate(email)]
        email: String,
        #[validate(length(min = 6))]
        password: String,
        exp: Option<SystemTime>,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct Token {
        refresh_token: TokenStruct,
        access_token: TokenStruct,
    }


    pub async fn auth_logout() -> impl Responder {
        HttpResponse::Ok().body("Hello logout!")
    }




}
