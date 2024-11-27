pub mod auth_signup{
    use std::fmt::format;
    use crate::service::token::token_service::{access_token, refresh_token, TokenStruct};
    use actix_web::web::Json;
    use actix_web::{HttpResponse, Responder};
    use bcrypt::{hash, DEFAULT_COST};
    use chrono::Utc;
    use entity::users;
    use sea_orm::ActiveValue::Set;
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
    use serde::{Deserialize, Serialize};
    use std::time::SystemTime;
    use rand::Rng;
    use validator::Validate;
    use crate::connection::dbconection::db_conection::{check_db_status, clone_db_conection, db_connection, DB, RDB};
    use crate::service::auth::auth_send_otp::auth_send_otp::RedisOtp;
    use crate::service::mail::Mail::{mail, otp_html};

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
        #[validate(length(min = 10, max = 10))] // Password length validation
        pub mob:String
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


    pub async fn auth_signup(req_body: Json<SignupBody>) -> impl Responder {
        match req_body.validate() {
            Ok(_) => (),
            Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        }

        let db =  match db_connection().await {
            Ok(db) =>db,
            Err(e) => {
                println!("Database Error occured ,{}",e.to_string());
                return HttpResponse::InternalServerError().finish(); // Redis connection error
            }
        };

        match users::Entity::find()
            .filter(users::Column::Email.eq(&req_body.email))
            .into_json()
            .one(&db)
            .await
        {
            Ok(user) => match user {
                Some(_) => return HttpResponse::AlreadyReported().finish(),
                _ => (),
            },
            _ => return HttpResponse::InternalServerError().finish(),
        };

        let hashed_password = hash(&req_body.password, DEFAULT_COST).unwrap();
        let strimage = &req_body.image;

        let image = match strimage {
            Some(t) => t.clone(),
            _ => "".to_string(),
        };

        let user_modal = users::ActiveModel {
            image: Set(image),
            username: Set(req_body.username.clone()),
            email: Set(req_body.email.clone()),
            mob:Set(req_body.mob.clone()),
            role: Set("user".to_string()),
            verified: Set(false),
            password: Set(hashed_password),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };
        println!("create user called ");
        match user_modal.insert(&db).await {
            Ok(_) => HttpResponse::Created().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        };
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
            Ok(_) => HttpResponse::Ok().body("Success!"),
            Err(e) =>  HttpResponse::InternalServerError().body(format!("Failed to send OTP email {} ",e.to_string())),
        }
    }


}
