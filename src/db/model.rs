use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProxyStatus {
    Unknown,
    Ok,
    Down,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProxyProtocol {
    Socks5,
    Http,
}

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub source: String,
    pub protocol: ProxyProtocol,
    pub status: ProxyStatus,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    #[serde_as(as = "bson::DateTime")]
    pub created_at: DateTime<Utc>,
    #[serde_as(as = "Option<bson::DateTime>")]
    pub checked_at: Option<DateTime<Utc>>,
    #[serde_as(as = "Option<bson::DateTime>")]
    pub last_ok_at: Option<DateTime<Utc>>,
}

impl Proxy {
    pub fn new(source: &Source, host: String) -> Self {
        Self {
            id: None,
            source: source.id.clone(),
            protocol: source.protocol,
            status: ProxyStatus::Unknown,
            username: source.username.clone(),
            password: source.password.clone(),
            host,
            port: source.port,
            created_at: Utc::now(),
            checked_at: None,
            last_ok_at: None,
        }
    }

    pub fn url(&self) -> String {
        let protocol = if self.protocol == ProxyProtocol::Socks5 { "socks5" } else { "http" };
        format!("{}://{}:{}@{}:{}", protocol, self.username, self.password, self.host, self.port)
    }
}

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "_id")]
    pub id: String, // represents name
    pub url: String,
    pub protocol: ProxyProtocol,
    pub username: String,
    pub password: String,
    pub port: u16,

    #[serde_as(as = "bson::DateTime")]
    pub created_at: DateTime<Utc>,

    #[serde_as(as = "Option<bson::DateTime>")]
    pub checked_at: Option<DateTime<Utc>>,
}

impl Source {
    pub fn new(id: String, url: String, protocol: ProxyProtocol, username: String, password: String, port: u16) -> Self {
        Self { id, url, protocol, username, password, port, created_at: Utc::now(), checked_at: None }
    }
}
