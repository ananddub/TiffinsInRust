use serde::{Deserialize, Serialize};
use validator::Validate;

pub mod auth_jwt_genrator{
    use actix_web::{HttpResponse, Responder};
    use chrono::{Duration, Utc};
    use sea_orm::{ ColumnTrait, EntityTrait, QueryFilter};
    use serde::Deserialize;
    use entity::users;
    use crate::connection::dbconection::db_conection::db_connection;
    use crate::model::users::users::UserModelToken;
    use crate::service::token::token_service::{access_token, refresh_token, token_decoder};

    #[derive(Deserialize, Debug)]
    pub struct JwtToken{
        pub token: String
    }

    pub async fn auth_access_token(req_body:actix_web::web::Json<JwtToken>)->impl Responder{
        let db=match db_connection().await {
            Ok( conn) =>conn,
            Err(e) => {
                println!("Database Error occured ,{}",e.to_string());
                return HttpResponse::InternalServerError().finish(); // Redis connection error
            }
        };
        let value =  match token_decoder::<UserModelToken>(&req_body.token){
            Ok(t)=>t,
            Err(e)=>{
                println!("{:?}", e);
                return HttpResponse::BadGateway().finish()
            }
        };
        let mut userdata = match users::Entity::find()
            .filter(users::Column::Email.eq(&value.email))
            .one(&db)
            .await
        {
            Ok(user) => match user {
                Some(value) => value,
                _ => return HttpResponse::NotFound().body("User not found".to_string()),
            },
            _ => return HttpResponse::InternalServerError().finish(),
        };
        let atok = UserModelToken{
            id: userdata.id,
            image: userdata.image.clone(),
            email: userdata.email.clone(),
            role: userdata.role.clone(),
            verified: userdata.verified,
            username: userdata.username.clone(),
            exp :(Utc::now() + Duration::days(7)).timestamp() as usize
        };
        HttpResponse::Ok().body(access_token(&atok))
    }
    pub async fn auth_refresh_token(req_body:actix_web::web::Json<JwtToken>)->impl Responder{
        let db=match db_connection().await {
            Ok( conn) =>conn,
            Err(e) => {
                println!("Database Error occured ,{}",e.to_string());
                return HttpResponse::InternalServerError().finish(); // Redis connection error
            }
        };
        let value =  match token_decoder::<UserModelToken>(&req_body.token){
            Ok(t)=>t,
            Err(e)=>{
                println!("{:?}", e);
                return HttpResponse::InternalServerError().finish()
            }
        };
        let mut userdata = match users::Entity::find()
            .filter(users::Column::Email.eq(&value.email))
            .one(&db)
            .await
        {
            Ok(user) => match user {
                Some(value) => value,
                _ => return HttpResponse::NotFound().body("User not found".to_string()),
            },
            _ => return HttpResponse::InternalServerError().finish(),
        };
        let atok = UserModelToken{
            id: userdata.id,
            image: userdata.image.clone(),
            email: userdata.email.clone(),
            role: userdata.role.clone(),
            verified: userdata.verified,
            username: userdata.username.clone(),
            exp :(Utc::now() + Duration::days(30)).timestamp() as usize
        };
        HttpResponse::Ok().body(refresh_token(&atok))
    }
}