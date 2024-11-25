
pub mod routes_user{
    use actix_web::{web, Error};
    use actix_web::body::MessageBody;
    use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
    use crate::middleware::user_auth_middleware::auth_middleware::user_auth_middleware;
    use crate::service::user::home::home::user_home;

    pub fn routes_user()->actix_web::Scope<impl ServiceFactory<ServiceRequest, Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
        InitError = ()>>{
           web::scope("/user")
            .route("/home",web::get().to(user_home))
               .wrap(actix_web::middleware::from_fn(user_auth_middleware))
    }

}