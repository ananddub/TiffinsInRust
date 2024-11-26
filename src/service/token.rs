pub mod token_service{
    use std::env;
    use dotenv::dotenv;
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
    use serde::{Deserialize, Deserializer, Serialize};
    use chrono::{Utc, Duration, DateTime};
    use serde::de::DeserializeOwned;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TokenStruct{
        pub token:String,
        pub exp: DateTime<Utc>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims <T>{
        pub sub:T, // Subject (e.g., user ID)
        pub exp: usize, // Expiration time (as a timestamp)
    }

    pub fn access_token<T: Serialize>(token:&T) ->String{
        dotenv().ok();
        let secret_key = env::var("JWT_SECRETKEY").unwrap_or("".to_string());
        let days = Utc::now() + Duration::days(7);
        let token = encode(
            &Header::default(),
            &token,
            &EncodingKey::from_secret(secret_key.as_bytes())
        ).unwrap_or_else(|_| "".to_string());
        token
    }
    pub fn refresh_token<T: Serialize>(token:&T ) ->String{
        dotenv().ok();
        let secret_key = env::var("JWT_SECRETKEY").unwrap_or("".to_string());
        let token = encode(
            &Header::default(),
            &token,
            &EncodingKey::from_secret(secret_key.as_bytes())
        ).unwrap_or_else(|_| "".to_string());

         token
    }
    pub fn token_decoder<T:DeserializeOwned>(token:&str)->Result<T,String>{
        dotenv().ok();
        let secret_key = env::var("JWT_SECRETKEY").unwrap_or("".to_string());
        println!("token:{:?}",&token[0..4]);
        match decode::<T>(
            &token,
            &DecodingKey::from_secret(secret_key.as_bytes()),
            &Validation::default()
        ){
            Ok(token_data) => Ok(token_data.claims),
            Err(e) => Err(format!("Unable To Decode Token {e}"))
        }

    }

}