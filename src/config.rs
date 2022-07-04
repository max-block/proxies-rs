use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub app_name: String,
    pub access_token: String,
    pub database_url: String,
    pub app_version: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        config::Config::builder()
            .set_default("app_version", env!("CARGO_PKG_VERSION"))
            .expect("can't set app_version")
            .add_source(config::Environment::default())
            .build()
            .expect("can't parse AppConfig")
            .try_deserialize()
            .expect("can't parse AppConfig")
    }
}
