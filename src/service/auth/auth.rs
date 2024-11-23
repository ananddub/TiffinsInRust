

pub mod auth {
    use std::time::SystemTime;
    use actix_web::{HttpResponse, Responder};
    use actix_web::web::{Json};
    use bcrypt::{hash, DEFAULT_COST};
    use chrono::Utc;
    use dotenv::dotenv;
    use sea_orm::ActiveValue::Set;
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Value};
    use entity::users;
    use crate::connection::dbconection::db_conection::dbconection;

    use serde::{Deserialize, Serialize};
    use validator::{Validate};
    use entity::prelude::Users;
    use crate::service::token::token_service::{access_token, refresh_token, TokenStruct};

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
    pub struct LoginBody{
        #[validate(email)]
        email:String,
        #[validate(length(min = 6))]
        password:String,
        exp:Option<SystemTime>
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct Token{
        refresh_token:TokenStruct,
        access_token:TokenStruct
    }

    pub async  fn login(req_body:Json<LoginBody>) -> impl Responder {
        dotenv().ok();
        match req_body.validate(){
            Ok(_)=>(),
            Err(e)=>{
                return HttpResponse::BadRequest().body(e.to_string())
            }
        }
        let  db = match dbconection().await{
            Ok(db) => db,
            Err(_)=>return HttpResponse::Forbidden().finish()
        };
        let mut userdata = match users::Entity::find()
            .filter(users::Column::Email.eq(&req_body.email))
            .one(&db)
            .await{
            Ok(user) =>match user {
                Some(value)=>value,
                _=>return HttpResponse::NotFound().finish()
            },
            _=>return HttpResponse::Forbidden().finish()
        };
        match bcrypt::verify(&req_body.password,&userdata.password){
            Ok(e)=>{
                if e==false {
                    return HttpResponse::NotAcceptable().finish();
                }
            }
            Err(_)=>return HttpResponse::NotAcceptable().finish()
        }
        userdata.password=String::from("");
        let token:Token = Token{
            access_token:access_token(&userdata),
            refresh_token:refresh_token(&userdata)
        };

       HttpResponse::Ok().json(serde_json::json!(token))
    }


    pub async  fn signup(req_body: Json<SignupBody>) -> impl Responder {
        match req_body.validate(){
            Ok(_)=>(),
            Err(e)=>{
               return HttpResponse::BadRequest().body(e.to_string())
            }
        }
        let  db = match dbconection().await{
            Ok(db) => db,
            Err(_)=>return HttpResponse::Forbidden().finish()
        };

        match users::Entity::find()
            .filter(users::Column::Email.eq(&req_body.email))
            .into_json()
            .one(&db)
            .await{
            Ok(user) =>match user {
                Some(_)=>return HttpResponse::AlreadyReported().finish(),
                _=>()
            },
            _=>return HttpResponse::Forbidden().finish()
        };



        let hashed_password = hash(&req_body.password, DEFAULT_COST).unwrap();
        let strimage = &req_body.image;

        let image = match strimage {
            Some(t)=>t.clone(),
            _=>"".to_string()
        };

        let user_modal = users::ActiveModel{
            image: Set(image),
            username:Set(req_body.username.clone()),
            email:Set(req_body.email.clone()),
            role:Set("user".to_string()),
            verified:Set(false),
            password:Set(hashed_password),
            created_at:Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        println!("create user called ");
        let db = dbconection().await.unwrap();
        match user_modal.insert(&db).await{
            Ok(_) =>  HttpResponse::Created().finish(),
            Err(_) =>  HttpResponse::InternalServerError().finish()
        }
    }


    pub async  fn forgot() -> impl Responder {
        HttpResponse::Ok().body("Hello forgot!")
    }

    pub async  fn logout() -> impl Responder {
        HttpResponse::Ok().body("Hello logout!")
    }

    pub async fn resend_otp()->impl Responder{
        HttpResponse::Ok().body("Hello Otp")
    }
}
