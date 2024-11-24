pub mod loginmiddlware {
    use std::time::Instant;
    use actix_web::body::MessageBody;
    use actix_web::dev::{ServiceRequest, ServiceResponse};
    use actix_web::Error;
    use actix_web::middleware::{ Next};
    pub async fn logmiddlware( req: ServiceRequest, next: Next<impl MessageBody>, ) -> Result<ServiceResponse <impl MessageBody>, Error> {
        let now = Instant::now();
    let routes = req.path().clone().to_string();
    let res = next.call(req).await?;
        let elapsed = now.elapsed();
        println!(
            "[LOG] Route: '{}', Time Elapsed: {} ms",
            routes,
            elapsed.as_millis()
        );
    Ok(res)
    }
}