pub mod token_service{
    use std::env;
    use std::time::SystemTime;
    use dotenv::dotenv;
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
    use serde::{Deserialize, Serialize};
    use chrono::{Utc, Duration, DateTime};
    use serde::de::DeserializeOwned;
    use serde::de::Unexpected::Str;
    use crate::service::auth::auth::auth::Token;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TokenStruct{
        pub token:String,
        pub exp: DateTime<Utc>,
    }
    pub struct TokenService{
        refresh_token:String,
        access_token:String

    }
    pub fn access_token<T: Serialize>(token:&T ) ->TokenStruct{
        dotenv().ok();
        let secret_key = env::var("JWT_SECRETKEY").unwrap_or("".to_string());
        let token =  match encode(
            &Header::default(),
            &token,
            &EncodingKey::from_secret(secret_key.as_bytes())
        ){
            Ok(token_data) => token_data,
            Err(_) => "".to_string()
        };
        let days = Utc::now() + Duration::days(7);
        TokenStruct{
            token:token,
            exp: days,
        }
    }
    pub fn refresh_token<T: Serialize>(token:&T ) ->TokenStruct{
        dotenv().ok();
        let secret_key = env::var("JWT_SECRETKEY").unwrap_or("".to_string());
        let token =  match encode(
            &Header::default(),
            &token,
            &EncodingKey::from_secret(secret_key.as_bytes())
        ){
            Ok(token_data) => token_data,
            Err(_) => "".to_string()
        };
        let days = Utc::now() + Duration::days(30);
        TokenStruct{
            token:token,
            exp: days,
        }
    }
    pub fn token_decoder<T:DeserializeOwned>(token:&str)->Result<TokenData<T>,String>{
        dotenv().ok();
        let secret_key = env::var("JWT_SECRETKEY").unwrap_or("".to_string());
        match decode(
            &token,
            &DecodingKey::from_secret(secret_key.as_bytes()),
            &Validation::default()
        ){
            Ok(token_data) => Ok(token_data),
            Err(_) => Err("Unable To Decode Token".to_string())
        }

    }

}