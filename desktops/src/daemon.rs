use std::{collections::BTreeMap, time::Duration};

use sqlx::{Pool, Postgres, Row};
use tokio::time::sleep;
use tokio_stream::StreamExt;
use tonic::transport::Channel;

use crate::daemon::user_external::{
    IncrementChronometerRequest, UsersTicks,
    user_external_service_client::UserExternalServiceClient,
};

pub mod user_external {
    tonic::include_proto!("user_external");
}

pub async fn increment(
    pool: Pool<Postgres>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let users_host = match std::env::var("USERS_HOST") {
        Ok(val) => val,
        Err(_) => "http://0.0.0.0:50051".to_string(),
    };
    let users_channel = Channel::from_shared(users_host)?.connect().await?;
    let mut user_auth_client = UserExternalServiceClient::new(users_channel);

    loop {
        sleep(Duration::from_mins(1)).await;

        let mut usernames: BTreeMap<String, u32> = BTreeMap::new();

        let mut rows = sqlx::query("SELECT username FROM desktops").fetch(&pool);

        while let Some(row) = rows.next().await {
            if let Ok(record) = row {
                let username: String = record.get("username");
                *usernames.entry(username).or_insert(0) += 1;
            }
        }

        let mut users_ticks = Vec::new();
        for (k, v) in usernames.iter() {
            users_ticks.push(UsersTicks {
                username: k.to_string(),
                tick: v.to_owned(),
            });
        }

        let _ = user_auth_client
            .increment_chronometer(IncrementChronometerRequest { users_ticks })
            .await;
    }
}
