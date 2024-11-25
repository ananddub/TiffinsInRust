pub mod auth_verify_otp {
    use actix_web::{web, HttpResponse, Responder};
    use redis::Commands;
    use entity::users;
    use redis_macros::{FromRedisValue, ToRedisArgs};
    use sea_orm::ActiveValue::Set;
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
    use serde::{Deserialize, Serialize};
    use validator::Validate;
    use entity::users::ActiveModel;
    use crate::connection::dbconection::db_conection::{check_db_status, check_rdb_status, db_connection, RDB};
    use crate::service::auth::auth_send_otp::auth_send_otp::RedisOtp;

    #[derive(Deserialize, Serialize,ToRedisArgs,FromRedisValue, Debug, Validate)]
    pub struct OtpVerriyStruct {
        #[validate(email)]
        pub email: String,
        #[validate(length(min = 6, max = 6))]
        pub otp: String,
    }

    pub async fn auth_verify_otp(req_body: web::Json<OtpVerriyStruct>) -> impl Responder {
        

        match req_body.validate() {
            Ok(_) => (),
            Err(e) => return HttpResponse::BadRequest().body(format!("Validation error: {}", e.to_string())),
        }

        let db = match db_connection().await{
            Ok(c) => c,
            Err(e) => return HttpResponse::InternalServerError().body(format!("db connection error: {}", e)),
        };
        let mut userdata = match users::Entity::find()
            .filter(users::Column::Email.eq(&req_body.email))
            .one(&db)
            .await
        {
            Ok(user) => match user {
                Some(value) => value,
                _ => return HttpResponse::NotFound().body("User not found"),
            },
            _ => return HttpResponse::InternalServerError().finish(),
        };
        if userdata.verified==true{
            return HttpResponse::AlreadyReported().body(format!("already verified"));
        }
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

        

        let otp_email = format!("otp-{}", req_body.email);
        let get_otp: RedisOtp = match rdb_conn.get(&otp_email) {
            Ok(rdb) => rdb,
            Err(_) => return HttpResponse::Gone().body("OTP expired or not found"),
        };

        

        if get_otp.otp == req_body.otp {
            let mut  usernewvlaue:ActiveModel = userdata.into();
            usernewvlaue.verified=Set(true);
            match usernewvlaue.update(&db).await
            {
                Ok(user) =>  { },
                _ => return HttpResponse::InternalServerError().finish(),
            };

            match redis::cmd("DEL").arg(&otp_email).query::<()>(rdb_conn) {
                Ok(_) => (),
                Err(e) => println!("Failed to delete OTP: {}", e.to_string()), 
            };
            HttpResponse::Ok().body("OTP verified successfully")
        } else {
            HttpResponse::Unauthorized().body("Invalid OTP or expired")
        }
    }
}