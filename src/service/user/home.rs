pub mod home {
    use actix_web::{HttpResponse, Responder};

    pub struct HomeService{
        pub username:String,
    }

    pub async fn user_home()->impl Responder{
        HttpResponse::Ok().body("Home")
    }

}