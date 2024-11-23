pub mod db_conection {
    use std::env;
    use dotenv::dotenv;
    use sea_orm::{Database, DatabaseConnection, DbErr};
    pub async  fn dbconection() -> Result<DatabaseConnection,DbErr> {
        dotenv().ok();
        let  conn_string:String  = env::var("DATABASE_URL")
            .unwrap_or("".to_string());
        println!("DATABASE_URL: {}",&conn_string[0..5]);
        let db: DatabaseConnection = Database::connect(&conn_string).await?;
        println!("Database connection established");
        Ok(db)
    }
    pub async  fn redisconnection()->Result<redis::Connection,redis::RedisError>{
        dotenv().ok();
        let  conn_string:String  = env::var("REDIS_URL")
            .unwrap_or(String::from(""));
        let client = redis::Client::open(conn_string)?;
        let  con = client.get_connection()?;
        println!("db connection established");
        Ok(con)
    }

}