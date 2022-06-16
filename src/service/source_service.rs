use crate::db::model::Proxy;
use crate::db::Db;
use crate::error::AppError;
use chrono::Utc;
use itertools::Itertools;
use mongodb::bson::{doc, Bson};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;

pub struct SourceService {
    db: Arc<Db>,
}

impl SourceService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn check(&self, id: String) -> crate::Result<bool> {
        let source = self.db.source.find_by_id(Bson::String(id.clone())).await?.ok_or(AppError::NotFound)?;
        let res = reqwest::get(&source.url).await?.text().await?;
        let ip4_addresses: Vec<String> = parse_ip4_addresses(&res).into_iter().unique().collect();

        for address in ip4_addresses {
            if !self.db.proxy.exists(doc! {"host": address.clone()}).await? {
                self.db.proxy.insert(&Proxy::new(&source, address)).await?;
            }
        }
        self.db.source.set_by_id(Bson::String(id), doc! {"checked_at": Utc::now()}).await?;

        Ok(true)
    }
}

fn parse_ip4_addresses(data: &str) -> Vec<String> {
    let mut result = vec![];
    for word in data.split_whitespace() {
        if Ipv4Addr::from_str(word).is_ok() {
            result.push(word.into());
        }
    }
    result
}
