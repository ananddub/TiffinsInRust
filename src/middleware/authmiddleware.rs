pub mod auth_middleware {
    use actix_web::body::MessageBody;
    use actix_web::dev::{ServiceRequest, ServiceResponse};
    use actix_web::{Error};
    use actix_web::error::ErrorUnauthorized;
    use actix_web::http::StatusCode;
    use actix_web::middleware::{ Next};
    use regex::Regex;

    pub async fn auth_middleware(req: ServiceRequest, next: Next<impl MessageBody>, ) -> Result<ServiceResponse
    <impl MessageBody>, Error> {
        let token = req.headers().get("Authorization");

         match token {
            Some(token) => {
                let re = Regex::new(r"^Bearer").unwrap();
                if re.is_match(token.to_str().unwrap()) {
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