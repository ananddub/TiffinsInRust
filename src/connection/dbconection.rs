pub mod db_conection {
    use std::env;
    use dotenv::dotenv;
    use sea_orm::{Database, DatabaseConnection, DbErr};
    pub async  fn db_conection() -> Result<DatabaseConnection,DbErr> {
        dotenv().ok();
        let  conn_string:String  = env::var("DATABASE_URL")
            .unwrap_or("".to_string());
        println!("DATABASE_URL: {}",&conn_string[0..5]);
        let db: DatabaseConnection = Database::connect(&conn_string).await?;
        println!("Database connection established");
        Ok(db)
    }
    pub async  fn redis_con()->redis::Connection{
        dotenv().ok();
        let  redis_conn_url:String  = env::var("REDIS_URL")
            .unwrap_or(String::from(""));
        let client = redis::Client::open(redis_conn_url)
            .expect("Redis connection failed to open")
        .get_connection()
        .expect("Redis connection failed to connect");
            ;


        println!("Redis connection established");
         client

    }

}