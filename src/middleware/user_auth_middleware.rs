pub mod auth_middleware {
    use actix_web::body::MessageBody;
    use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
    use actix_web::{Error};
    use actix_web::error::{ErrorGone, ErrorUnauthorized};
    use actix_web::http::StatusCode;
    use actix_web::middleware::{ Next};
    use chrono::Utc;
    use regex::Regex;
    use entity::users::Model;
    use crate::model::users::users::UserModelToken;
    use crate::service::token::token_service::token_decoder;

    pub async fn user_auth_middleware(req: ServiceRequest, next: Next<impl MessageBody>, ) -> Result<ServiceResponse
    <impl MessageBody>, Error> {
        let token = req.headers().get("Authorization");

        match token {
            Some(token) => {
                let re = Regex::new(r"^Bearer").unwrap();
                if re.is_match(token.to_str().unwrap()) {
                    let token = token.to_str().unwrap().replace("Bearer ", "");
                    let value =  match token_decoder::<UserModelToken>(&token){
                            Ok(t)=>t,
                            Err(e)=>{
                                println!("{:?}", e);
                                return Err(ErrorUnauthorized(StatusCode::UNAUTHORIZED))
                        }
                    };
                    let curdate = Utc::now().timestamp() as usize;
                    if value.exp < curdate {
                        return Err(ErrorGone(StatusCode::GONE))
                    }
                    println!("value {:?}",value);
                    Ok(next.call(req).await?)
                }else{
                     Err(ErrorUnauthorized(StatusCode::UNAUTHORIZED))
                }
            }
            None => {
                Err(ErrorUnauthorized(StatusCode::UNAUTHORIZED))
            }
        }
    }
}