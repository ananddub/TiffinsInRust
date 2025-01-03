pub mod auth_forgot_send_otp {
    use crate::service::mail::Mail::{forgot_password_html, mail, otp_html};
    use crate::service::token::token_service::{ TokenStruct};
    use actix_web::web::Json;
    use actix_web::{HttpResponse, Responder};
    use entity::users;
    use redis_macros::{FromRedisValue, ToRedisArgs};
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
    use serde::{Deserialize, Serialize};
    use rand::Rng;
    use redis::{Commands};
    use validator::Validate;
    use crate::connection::dbconection::db_conection::{check_db_status, check_rdb_status, clone_db_conection, db_connection, DB, RDB};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Token {
        refresh_token: TokenStruct,
        access_token: TokenStruct,
    }

    #[derive(Deserialize, Serialize, Debug, Validate)]
    pub struct SendOtp {
        #[validate(email)]
        email: String,
    }

    #[derive(Serialize, Deserialize,FromRedisValue, ToRedisArgs, Debug)]
    pub struct RedisOtp {
        pub otp: String,
        pub count: u8,
    }

    pub async fn auth_forgot_send(req_body: Json<SendOtp>) -> impl Responder {
        match req_body.validate() {
            Ok(_) => (),
            Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        }

        let db =  match db_connection().await  {
            Ok(db) =>db,
            Err(e) => {
                println!("Database Error occured ,{}",e.to_string());
                return HttpResponse::InternalServerError().finish(); // Redis connection error
            }
        };

        match users::Entity::find().
            filter(users::Column::Email.eq(&req_body.email))
            .one(&db).await {
            Ok(e) => match e {
                Some(e) => {
                    if e.verified == true {
                        return HttpResponse::AlreadyReported().body("already verified")
                    }
                }
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
        let mut rdb_lock =match  RDB.lock(){
            Ok(rdb) => rdb,
            Err(e) =>return HttpResponse::InternalServerError().body(e.to_string()),
        };
        let redis_conn=match *rdb_lock {
            Ok(ref mut conn) =>conn,
            Err(e) => {
                println!("redis_conn Error occured ,{}",e.to_string());
                return HttpResponse::InternalServerError().finish(); // Redis connection error
            }
        };
        let emailotp = "forgot-otp-".to_string() + &req_body.email;
        let session_result: RedisOtp = match redis_conn.get(&emailotp){
            Ok(s) => s,
            Err(_)=>{
                RedisOtp {
                    otp: otpstr.clone(),
                    count: 1,
                }
            }
        };
        if session_result.count>=5{
            return HttpResponse::TooManyRequests().body("Too many requests!");
        }
        let otpvalue = RedisOtp {
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
        match mail(&req_body.email, &forgot_password_html(&otpstr, &req_body.email)).await {
            Ok(_) => HttpResponse::Ok().body("Success!"),
            Err(e) =>  HttpResponse::InternalServerError().body(format!("Failed to send OTP email {} ",e.to_string())),
        }
    }
}