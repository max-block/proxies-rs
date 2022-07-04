use crate::util::utc_delta_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Result};

pub struct Db {
    pool: PgPool,
}

// models

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub username: String,
    pub password: String,
    pub port: i32,
    pub protocol: String,
    pub created_at: DateTime<Utc>,
    pub checked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    pub id: i32,
    pub status: String,
    pub protocol: String,
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: i32,
    pub created_at: DateTime<Utc>,
    pub checked_at: Option<DateTime<Utc>>,
    pub last_ok_at: Option<DateTime<Utc>>,
}

impl Proxy {
    pub fn url(&self) -> String {
        format!("{}://{}:{}@{}:{}", self.protocol, self.username, self.password, self.host, self.port)
    }
}

// params

#[derive(Debug, Deserialize)]
pub struct CreateSource {
    pub name: String,
    pub link: String,
    pub username: String,
    pub password: String,
    pub port: i32,
    pub protocol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProxy {
    pub protocol: String,
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: i32,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new().max_connections(5).connect(database_url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Db { pool })
    }

    pub async fn create_source(&self, params: CreateSource) -> Result<i32> {
        let res = sqlx::query!(
            r#"
            insert into source (name, link, username, password, port, protocol) 
            values ($1, $2, $3, $4, $5, $6) returning id"#,
            params.name,
            params.link,
            params.username,
            params.password,
            params.port,
            params.protocol
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(res.id)
    }

    pub async fn find_sources(&self) -> Result<Vec<Source>> {
        sqlx::query_as!(Source, "select * from source order by id").fetch_all(&self.pool).await
    }

    pub async fn find_sources_for_check(&self, limit: i64) -> Result<Vec<i32>> {
        sqlx::query_scalar!(
            "select id from source where checked_at is null or checked_at < $1 order by checked_at nulls first limit $2",
            utc_delta_seconds(-600),
            limit
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_source(&self, id: i32) -> Result<Option<Source>> {
        sqlx::query_as!(Source, "select * from source where id=$1", id).fetch_optional(&self.pool).await
    }

    pub async fn set_checked_at_for_source(&self, id: i32) -> Result<u64> {
        Ok(sqlx::query!("update source set checked_at=now() where id=$1", id).execute(&self.pool).await?.rows_affected())
    }

    pub async fn find_proxies(&self) -> Result<Vec<Proxy>> {
        sqlx::query_as!(Proxy, "select * from proxy order by id").fetch_all(&self.pool).await
    }

    pub async fn find_proxy(&self, id: i32) -> Result<Option<Proxy>> {
        sqlx::query_as!(Proxy, "select * from proxy where id=$1", id).fetch_optional(&self.pool).await
    }

    pub async fn find_proxies_for_check(&self, limit: i64) -> Result<Vec<i32>> {
        sqlx::query_scalar!(
            "select id from proxy where checked_at is null or checked_at < $1 order by checked_at nulls first limit $2",
            utc_delta_seconds(-60),
            limit
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_live_proxies(&self) -> Result<Vec<Proxy>> {
        sqlx::query_as!(Proxy, "select * from proxy where status = $1 and last_ok_at >= $2", "ok", utc_delta_seconds(-5 * 60))
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_proxy(&self, params: &CreateProxy) -> Result<Option<i32>> {
        let res = sqlx::query!(
            r#"
            insert into proxy (protocol, host, username, password, port)
            values ($1, $2, $3, $4, $5) on conflict do nothing returning id"#,
            params.protocol,
            params.host,
            params.username,
            params.password,
            params.port
        )
        .fetch_optional(&self.pool)
        .await?;
        if let Some(res) = res {
            Ok(Some(res.id))
        } else {
            Ok(None)
        }
    }

    pub async fn set_proxy_status(&self, id: i32, status: &str) -> Result<u64> {
        let mut query = String::from("update proxy set status=$1, checked_at=now()");
        if status == "ok" {
            query.push_str(", last_ok_at=now() ");
        }
        query.push_str(" where id=$2");
        Ok(sqlx::query(&query).bind(status).bind(id).execute(&self.pool).await?.rows_affected())
    }
}
