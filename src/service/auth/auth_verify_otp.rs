pub mod auth_verify_otp{
    use actix_web::{HttpResponse, Responder};
    use crate::connection::dbconection::db_conection::{check_db_status, check_rdb_status, RDB};

    pub async fn auth_verify_otp() ->impl Responder{

        let mut rdb_lock = match RDB.lock(){
            Ok(rdb_lock) => rdb_lock,
            Err(_) => return HttpResponse::InternalServerError().finish()
        };

        let mut rdb_conn = match *rdb_lock{
            Ok(ref mut rdb) => rdb,
            Err(e) => {
                println!("rdb_lock error {}",e.to_string());
                return HttpResponse::InternalServerError().finish();
            }
        };


        HttpResponse::Ok().finish()
    }
}