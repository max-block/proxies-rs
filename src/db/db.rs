use mongodb::options::ClientOptions;
use mongodb::Client;

use crate::db::model::{Proxy, Source};
use crate::db::util::MongoCollection;
use crate::error::AppError;

pub struct Db {
    pub source: MongoCollection<Source>,
    pub proxy: MongoCollection<Proxy>,
}

impl Db {
    pub async fn new(database_url: &str) -> crate::Result<Self> {
        let client = Client::with_uri_str(database_url).await?;
        let database_name = ClientOptions::parse(database_url).await?.default_database.ok_or(AppError::EmptyDatabase)?;
        let database = client.database(&database_name);

        Ok(Self {
            source: MongoCollection::new(database.collection::<Source>("source")),
            proxy: MongoCollection::new(database.collection::<Proxy>("proxy")),
        })
    }
}
