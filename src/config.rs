use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub app_name: String,
    pub access_token: String,
    pub database_url: String,
    pub cargo_pkg_version: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .expect("can't parse AppConfig")
            .try_deserialize()
            .expect("can't parse AppConfig")
    }
}
