pub mod users{
    use chrono::Utc;
    use sea_orm::prelude::DateTime;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug,Deserialize, Serialize )]
    pub struct UserModel {
        pub id: i32,
        pub image: String,
        pub email: String,
        pub role: String,
        pub verified: bool,
        pub password: String,
        pub username: String,
        pub created_at: DateTime,
    }
    #[derive(Clone, Debug,Deserialize, Serialize )]
    pub struct UserModelToken {
        pub id: i32,
        pub image: String,
        pub email: String,
        pub role: String,
        pub verified: bool,
        pub username: String,
        pub exp :usize
    }
}