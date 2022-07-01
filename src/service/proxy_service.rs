use std::{sync::Arc, time::Duration};

use futures::future::join_all;
use reqwest::{Client, Proxy};
use serde::Deserialize;

use crate::{db::Db, AppError, async_synchronized, Result};

pub struct ProxyService {
    db: Arc<Db>,
}

impl ProxyService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn check(&self, id: i32) -> Result<String> {
        log::info!("proxy_service: check {}", id);

        #[derive(Deserialize, Debug)]
        struct Response {
            origin: String,
        }

        let proxy = self.db.find_proxy(id).await?.ok_or(AppError::NotFound)?;

        let mut status = "down";
        let client = Client::builder().proxy(Proxy::all(proxy.url())?).timeout(Duration::from_secs(3)).build()?;

        if let Ok(res) = client.get("https://httpbin.org/ip").send().await {
            if let Ok(res) = res.json::<Response>().await {
                if res.origin == proxy.host {
                    status = "ok";
                }
            }
        }
        self.db.set_proxy_status(id, status).await?;
        Ok(status.to_string())
    }

    pub async fn check_next(&self) ->Result<()>{
        async_synchronized!();
        println!("proxy_service.check_next");
        let proxies = self.db.find_proxies_for_check(10).await?;
        join_all(proxies.iter().map(|id|self.check(*id)).collect::<Vec<_>>()).await;
        Ok(())
    }
}
