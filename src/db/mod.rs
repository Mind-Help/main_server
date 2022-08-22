use std::str::FromStr;

use std::env::var;
use tokio_postgres::{Client, Config, Error, NoTls};

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn new() -> Result<Self, Error> {
        let db_config = Config::from_str(
            var("DATABASE_URL")
                .expect("$DATABASE_URL not found")
                .as_str(),
        )?;

        let (client, connection) = db_config.connect(NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self { client })
    }
}
