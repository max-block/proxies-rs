use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;

use itertools::Itertools;

use crate::db::{CreateProxy, Db};
use crate::{AppError, Result};

pub struct SourceService {
    db: Arc<Db>,
}

impl SourceService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn check(&self, id: i32) -> Result<Vec<CreateProxy>> {
        let source = self.db.find_source(id).await?.ok_or(AppError::NotFound)?;
        let res = reqwest::get(&source.link).await?.text().await?;
        let ip4_addresses: Vec<String> = parse_ip4_addresses(&res).into_iter().unique().collect();
        let proxies: Vec<CreateProxy> = ip4_addresses
            .iter()
            .map(|host| CreateProxy {
                protocol: source.protocol.clone(),
                host: host.to_string(),
                username: source.username.clone(),
                password: source.password.clone(),
                port: source.port,
            })
            .collect_vec();

        for p in &proxies {
            self.db.create_proxy(p).await?;
        }
        self.db.set_checked_at_for_source(id).await?;
        Ok(proxies)
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
