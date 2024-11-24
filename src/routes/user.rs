
pub mod user{
    use actix_web::{web};

    pub fn routes_user() ->actix_web::Scope{
        web::scope("/user")
    }
}