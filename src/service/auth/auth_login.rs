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
    use rand::Rng;
    use validator::Validate;
    use entity::users::Model;
    use crate::connection::dbconection::db_conection::{db_connection, RDB};
    use crate::model::users::users::{UserModel, UserModelToken};
    use crate::service::auth::auth_send_otp::auth_send_otp::RedisOtp;
    use crate::service::mail::Mail::{mail, otp_html};

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
        if userdata.verified == false{
            fn generate_otp() -> String {
                (0..6)
                    .map(|_| rand::thread_rng().gen_range(0..10).to_string())
                    .collect::<String>()
            }
            let otpvalue = RedisOtp {
                otp: generate_otp(),
                count: 0,
            };
            let emailotp = format!("otp-{}",req_body.email);
            let strvalue = serde_json::to_string(&otpvalue).unwrap_or(String::new());
            let mut rdb_lock = match RDB.lock() {
                Ok(rdb_lock) => rdb_lock,
                Err(_) => return HttpResponse::InternalServerError().body("Failed to acquire Redis lock"),
            };

            let mut rdb_conn = match *rdb_lock {
                Ok(ref mut rdb) => rdb,
                Err(e) => {
                    println!("Redis lock error: {}", e.to_string());
                    return HttpResponse::InternalServerError().body("Failed to get Redis connection");
                }
            };
            match redis::cmd("SETEX").arg(emailotp).arg(60*5).arg(strvalue).query::<()>(rdb_conn) {
                Ok(_)=>{},
                Err(e)=>return HttpResponse::InternalServerError().body(
                    format!("Could not set OTP {}",e.to_string())),
            }
            // Send OTP email
            match mail(&req_body.email, &otp_html(&otpvalue.otp, &req_body.email)).await {
                Ok(_) => {},
                Err(e) =>  return HttpResponse::InternalServerError().body(format!("Failed to send OTP email {} ",e.to_string())),
            };
        }
        HttpResponse::Ok().json(serde_json::json!(token))
    }





}
