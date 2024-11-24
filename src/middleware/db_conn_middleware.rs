pub mod db_conn_middleware {
    use actix_web::body::MessageBody;
    use actix_web::dev::{ServiceRequest, ServiceResponse};
    use actix_web::{Error};
    use actix_web::error::{ErrorInternalServerError };
    use actix_web::http::StatusCode;
    use actix_web::middleware::{ Next};
    use crate::connection::dbconection::db_conection::{check_db_status, check_rdb_status};

    pub async fn db_con_middleware(req: ServiceRequest, next: Next<impl MessageBody>, ) -> Result<ServiceResponse
    <impl MessageBody>, Error> {
        if check_rdb_status().await == false{
            println!("Reddis conection failed");
            return Err(ErrorInternalServerError(StatusCode::INTERNAL_SERVER_ERROR));
        }
        if check_db_status().await == false{
            println!("Database conection failed");
            return Err(ErrorInternalServerError(StatusCode::INTERNAL_SERVER_ERROR));
        }
        Ok(next.call(req).await?)
    }
}