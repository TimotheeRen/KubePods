use std::{collections::BTreeMap, time::Duration};

use sqlx::{Pool, Postgres, Row};
use tokio::time::sleep;
use tokio_stream::StreamExt;

pub async fn increment(pool: Pool<Postgres>) {
    loop {
        sleep(Duration::from_mins(1)).await;

        let mut usernames: BTreeMap<String, u8> = BTreeMap::new();

        let mut rows = sqlx::query("SELECT username FROM desktops").fetch(&pool);

        while let Some(row) = rows.next().await {
            if let Ok(record) = row {
                let username: String = record.get("username");
                *usernames.entry(username).or_insert(0) += 1;
            }
        }
        println!("{:?}", usernames)
    }
}
