use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    dotenv::dotenv().ok();
    config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("can't parse AppConfig")
        .try_deserialize()
        .expect("can't parse AppConfig")
});

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub access_token: String,
    pub database_url: String,
    pub cargo_pkg_version: String,

}

impl AppConfig {
    pub fn get() -> Self {
        CONFIG.clone()
    }
}
