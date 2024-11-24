pub mod db_conection {
    use std::env;
    use std::sync::Mutex;
    use dotenv::dotenv;
    use lazy_static::lazy_static;
    use redis::{Client, RedisError};
    use sea_orm::{Database, DatabaseConnection, DbErr};

    lazy_static! {
    pub static ref RDB: Mutex<Result<redis::Connection, bool>> = Mutex::new(Err(false));
    pub static ref DB: Mutex<Result<DatabaseConnection, bool>> = Mutex::new(Err(false));
}
    pub async  fn db_connection() -> Result<DatabaseConnection,DbErr> {
        dotenv().ok();
        let  conn_string:String  = env::var("DATABASE_URL")
            .unwrap_or("".to_string());
        println!("DATABASE_URL: {}",&conn_string[0..5]);
        let db: DatabaseConnection = Database::connect(&conn_string).await?;
        println!("Database connection established");
        Ok(db)
    }
    pub async fn check_db_status()->bool{
        unsafe {
            let mut rdb_lock =DB.lock().unwrap();
            match *rdb_lock {
                Ok(ref rdb) => {return true}
                _=>{}
            }
            let rdb = match db_connection().await {
                Ok(rdb) => rdb,
                Err(_)=>return false
            };
            *rdb_lock = Ok(rdb);
            println!("Redis Conected Successfully!");
        }
        true
    }
    pub async fn check_rdb_status() ->bool{
        unsafe {
            let mut rdb_lock = RDB.lock().unwrap();
            match *rdb_lock {
                Ok(ref rdb) => {return true}
                _=>{}
            }
            let rdb = match redis_con().await {
                Ok(rdb) => rdb,
                Err(_)=>return false
            };
            *rdb_lock = Ok(rdb);
            println!("Redis Conected Successfully!");
        }
        true
    }
    pub async  fn redis_con()->Result<redis::Connection,RedisError>{
        dotenv().ok();

        let  redis_conn_url:String  = env::var("REDIS_URL")
            .unwrap_or(String::from(""));
        let client =  Client::open(redis_conn_url)?;
        client.get_connection()
    }


}