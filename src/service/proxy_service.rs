use bson::oid::ObjectId;
use bson::{doc, Bson};
use chrono::Utc;
use reqwest::{Client, Proxy};
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;

use crate::db::model::ProxyStatus;
use crate::db::Db;
use crate::error::AppError;

pub struct ProxyService {
    db: Arc<Db>,
}

impl ProxyService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn check(&self, id: ObjectId) -> crate::Result<()> {
        let proxy = self.db.proxy.find_by_id(Bson::ObjectId(id)).await?.ok_or(AppError::NotFound)?;
        let client = Client::builder().proxy(Proxy::all(proxy.url())?).timeout(Duration::from_secs(2)).build()?;

        let mut status = ProxyStatus::Down;
        if let Ok(res) = client.get("https://httpbin.org/ip").send().await {
            if let Ok(res) = res.json::<HttpbinIpResponse>().await {
                if res.origin == proxy.host {
                    status = ProxyStatus::Ok;
                }
            }
        }
        let status = bson::to_bson(&status).unwrap();
        self.db.proxy.set_by_id(Bson::ObjectId(id), doc! {"status": status, "checked_at": Utc::now().to_owned()}).await?;

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct HttpbinIpResponse {
    origin: String,
}
