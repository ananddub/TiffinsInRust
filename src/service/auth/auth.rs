pub mod auth {
    use crate::connection::dbconection::db_conection::{db_conection, redis_con};
    use crate::service::mail::Mail::{mail, otp_html};
    use crate::service::token::token_service::{access_token, refresh_token, TokenStruct};
    use actix_web::web::Json;
    use actix_web::{HttpResponse, Responder};
    use bcrypt::{hash, DEFAULT_COST};
    use chrono::Utc;
    use dotenv::dotenv;
    use entity::users;
    use lettre::Transport;
    use migration::extension::sqlite::SqliteBinOper::Match;
    use rand::Rng;
    use redis::{Commands, FromRedisValue, ToRedisArgs};
    use sea_orm::ActiveValue::Set;
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
    use serde::{Deserialize, Serialize};
    use std::time::SystemTime;
    use log::debug;
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

    pub async fn login(req_body: Json<LoginBody>) -> impl Responder {
        dotenv().ok();
        match req_body.validate() {
            Ok(_) => (),
            Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        }
        let db = match db_conection().await {
            Ok(db) => db,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };
        let mut userdata = match users::Entity::find()
            .filter(users::Column::Email.eq(&req_body.email))
            .one(&db)
            .await
        {
            Ok(user) => match user {
                Some(value) => value,
                _ => return HttpResponse::NotFound().finish(),
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
        let token: Token = Token {
            access_token: access_token(&userdata),
            refresh_token: refresh_token(&userdata),
        };

        HttpResponse::Ok().json(serde_json::json!(token))
    }

    pub async fn signup(req_body: Json<SignupBody>) -> impl Responder {
        match req_body.validate() {
            Ok(_) => (),
            Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        }
        let db = match db_conection().await {
            Ok(db) => db,
            Err(_) => return HttpResponse::InternalServerError().finish(),
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
            role: Set("user".to_string()),
            verified: Set(false),
            password: Set(hashed_password),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        println!("create user called ");
        let db = db_conection().await.unwrap();
        match user_modal.insert(&db).await {
            Ok(_) => HttpResponse::Created().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    pub async fn forgot() -> impl Responder {
        HttpResponse::Ok().body("Hello forgot!")
    }

    pub async fn logout() -> impl Responder {
        HttpResponse::Ok().body("Hello logout!")
    }

    #[derive(Deserialize, Serialize, Debug, Validate)]
    pub struct SendOtp {
        #[validate(email)]
        email: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RediSession {
        otp: String,
        count: u8,
    }

    pub async fn send_otp(req_body: Json<SendOtp>) -> impl Responder {
        fn generate_otp() -> String {
            (0..6)
                .map(|_| rand::thread_rng().gen_range(0..10).to_string())
                .collect::<String>()
        }

        let otpstr = generate_otp();

        // Establish Redis connection
        let mut redis_conn: redis::Connection = redis_con().await;
        let emailotp = "otp-".to_string() + &req_body.email;
        // Try to get the current OTP session from Redis

        let session_result: RediSession = match   redis::cmd("GET")
            .arg(&emailotp)
            .query::<String>(&mut redis_conn){
            Ok(sessionreddis) =>  match serde_json::from_str(&sessionreddis) {
                Ok(sessionreddis) => sessionreddis,
                Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            },
            Err(_)=>{
                RediSession {
                    otp: otpstr.clone(),
                    count: 0,
                }
            }
        };
        if(session_result.count>5){
            return HttpResponse::TooManyRequests().body("Too many requests!");
        }
        let optvalue = RediSession {
            otp: otpstr.clone(),
            count: session_result.count+1,
        };

        let serialized_session = serde_json::to_string(&optvalue).unwrap_or("".to_string());

        match redis::cmd("SETEX")
            .arg(&emailotp)
            .arg(300)
            .arg(serialized_session)
            .query::<()>(&mut redis_conn){
            Ok(_)=>{},
            Err(_)=>return HttpResponse::InternalServerError().finish(),
        }

        // Send OTP email
        match mail(&req_body.email, &otp_html(&otpstr, &req_body.email)).await {
            Ok(_) => {}
            Err(_) => return HttpResponse::InternalServerError().body("Failed to send OTP email"),
        }

        HttpResponse::Ok().finish()
    }
}
