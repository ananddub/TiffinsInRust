pub mod auth {
    use crate::service::mail::Mail::{mail, otp_html};
    use crate::service::token::token_service::{access_token, refresh_token, TokenStruct};
    use actix_web::web::Json;
    use actix_web::{HttpResponse, Responder};
    use bcrypt::{hash, DEFAULT_COST};
    use chrono::Utc;
    use dotenv::dotenv;
    use entity::users;
    use lettre::Transport;
    use redis_macros::{FromRedisValue, ToRedisArgs};
    use sea_orm::ActiveValue::Set;
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
    use serde::{Deserialize, Serialize};
    use std::time::SystemTime;
    use actix_redis_client::ActixRedisClientError::RedisError;
    use futures::FutureExt;
    use rand::Rng;
    use redis::{Commands, Connection};
    use validator::Validate;
    use crate::connection::dbconection::db_conection::{check_rdb_status, db_connection, redis_con, RDB};

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
        let db = match db_connection().await {
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
        let db = match db_connection().await {
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
        let db = db_connection().await.unwrap();
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

    #[derive(Serialize, Deserialize,FromRedisValue, ToRedisArgs, Debug)]
    pub struct RediSession {
        otp: String,
        count: u8,
    }

    pub async fn send_otp(req_body: Json<SendOtp>) -> impl Responder {
        match req_body.validate() {
            Ok(_) => (),
            Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        }
        let db = match db_connection().await {
            Ok(db) => db,
            Err(e) => {
                println!("db Error occured ,{}",e.to_string());
                return HttpResponse::InternalServerError().finish()
            },
        };
        match users::Entity::find().
            filter(users::Column::Email.eq(&req_body.email))
            .one(&db).await {
            Ok(e) => match e {
                Some(_) => {}
                _ => return HttpResponse::NotFound().finish(),
            }
            Err(e) => {
                println!("db Error occured ,{}",e.to_string());
                return HttpResponse::InternalServerError().finish()
            },
        }

        fn generate_otp() -> String {
            (0..6)
                .map(|_| rand::thread_rng().gen_range(0..10).to_string())
                .collect::<String>()
        }

        let otpstr = generate_otp();
        let rdbbool = check_rdb_status().await;
        if rdbbool==false{
            println!("redis Connection Error");
            return HttpResponse::InternalServerError().finish();
        };
        let mut rdb_lock = RDB.lock().unwrap();
        let redis_conn=match *rdb_lock {
            Ok(ref mut conn) =>conn,
            Err(e) => {
                println!("redis_conn Error occured ,{}",e.to_string());
                return HttpResponse::InternalServerError().finish(); // Redis connection error
            }
        };

        let emailotp = "otp-".to_string() + &req_body.email;


        let session_result:RediSession = match redis_conn.get(&emailotp){
            Ok(s) => s,
            Err(_)=>{
                RediSession {
                    otp: otpstr.clone(),
                    count: 1,
                }
            }
        };
        if session_result.count>5{
            return HttpResponse::TooManyRequests().body("Too many requests!");
        }
        let otpvalue = RediSession {
            otp: otpstr.clone(),
            count: session_result.count+1,
        };

        let strvalue = serde_json::to_string(&otpvalue).unwrap_or(String::new());
        match redis::cmd("SETEX").arg(emailotp).arg(60*5).arg(strvalue).query::<()>(redis_conn) {
            Ok(_)=>{},
            Err(e)=>return HttpResponse::InternalServerError().body(
                format!("Could not set OTP {}",e.to_string())),
        }
        // Send OTP email
        match mail(&req_body.email, &otp_html(&otpstr, &req_body.email)).await {
            Ok(_) => HttpResponse::Ok().body("Success!"),
            Err(e) =>  HttpResponse::InternalServerError().body(format!("Failed to send OTP email {} ",e.to_string())),
        }

    }
}
