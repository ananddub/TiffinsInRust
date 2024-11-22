pub mod auth {
    use actix_web::{HttpResponse, Responder};
    pub async  fn login() -> impl Responder {
       HttpResponse::Ok().body("Hello login!")
    }
    pub async  fn signup() -> impl Responder {
        HttpResponse::Ok().body("Hello signup!")
    }
    pub async  fn forgot() -> impl Responder {
        HttpResponse::Ok().body("Hello forgot!")
    }
    pub async  fn logout() -> impl Responder {
        HttpResponse::Ok().body("Hello logout!")
    }
}
