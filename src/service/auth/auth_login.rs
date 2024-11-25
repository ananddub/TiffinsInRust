pub mod auth_login {
    use crate::service::token::token_service::{access_token, refresh_token, TokenStruct};
    use actix_web::web::Json;
    use actix_web::{HttpResponse, Responder};
    use dotenv::dotenv;
    use entity::users;
    use sea_orm::{ ColumnTrait, EntityTrait, QueryFilter};
    use serde::{Deserialize, Serialize};
    use std::time::SystemTime;
    use chrono::{Duration, Utc};
    use validator::Validate;
    use entity::users::Model;
    use crate::connection::dbconection::db_conection::{ db_connection };
    use crate::model::users::users::{UserModel, UserModelToken};

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
        refresh_token: String,
        access_token: String,
    }


    pub async fn auth_login(req_body: Json<LoginBody>) -> impl Responder {
        dotenv().ok();
        match req_body.validate() {
            Ok(_) => (),
            Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        }

        let db=match db_connection().await {
            Ok( conn) =>conn,
            Err(e) => {
                println!("Database Error occured ,{}",e.to_string());
                return HttpResponse::InternalServerError().finish(); // Redis connection error
            }
        };

        let mut userdata = match users::Entity::find()
            .filter(users::Column::Email.eq(&req_body.email))
            .one(&db)
            .await
        {
            Ok(user) => match user {
                Some(value) => value,
                _ => return HttpResponse::NotFound().body("User not found".to_string()),
            },
            _ => return HttpResponse::InternalServerError().finish(),
        };

        match bcrypt::verify(&req_body.password, &userdata.password) {
            Ok(e) => {
                if e == false {
                    return HttpResponse::NotAcceptable().finish();
                }
            }
            Err(_) => return HttpResponse::NotAcceptable().finish(),
        }
        userdata.password = String::from("");

        let mut atok = UserModelToken{
             id: userdata.id,
             image: userdata.image.clone(),
             email: userdata.email.clone(),
             role: userdata.role.clone(),
             verified: userdata.verified,
             username: userdata.username.clone(),
             exp :(Utc::now() + Duration::days(7)).timestamp() as usize
        };
        let mut rtok = atok.clone();
        rtok.exp = (Utc::now() + Duration::days(30)).timestamp() as usize;
        let token: Token = Token {
            access_token: access_token::<UserModelToken>(&atok),
            refresh_token: refresh_token::<UserModelToken>(&rtok),
        };
        HttpResponse::Ok().json(serde_json::json!(token))
    }





}
