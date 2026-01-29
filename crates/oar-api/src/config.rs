use std::env;

pub struct Config {
    pub database_url: String,
    pub api_port: u16,
    pub api_host: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok(); 

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            api_port: env::var("API_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("API_PORT must be a number"),
            api_host: env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        }
    }
}