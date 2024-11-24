pub mod auth_forgot {
    use actix_web::{web, HttpResponse, Responder};
    use serde::{Deserialize, Serialize};
    use validator::Validate;
    #[derive(Deserialize, Serialize, Debug, Validate)]
    pub struct SendOtp {
        #[validate(email)]
        email: String,
    }

    pub async fn auth_forgot(req_body:web::Json<SendOtp>) -> impl Responder {
        HttpResponse::Ok().json(req_body)
    }




}
