use futures::StreamExt;
use std::{sync::Arc, time::Duration};

use kube::{
    Api, Client, Error,
    runtime::{Controller, controller::Action, watcher::Config},
};
use operator::Desktop;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let desktops: Api<Desktop> = Api::namespaced(client, "desktops");
    let config = Config::default();
    let context = Arc::new(ContextData {});
    Controller::new(desktops.clone(), config.clone())
        .owns(desktops, config)
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => println!("reconciled: {:?}", o),
                Err(e) => println!("reconciliation failed: {}", e),
            }
        })
        .await;
    Ok(())
}

struct ContextData {}

async fn reconcile(desktop: Arc<Desktop>, context: Arc<ContextData>) -> Result<Action, Error> {
    Ok(Action::requeue(Duration::from_secs(300)))
}

fn error_policy(desktop: Arc<Desktop>, error: &Error, context: Arc<ContextData>) -> Action {
    Action::requeue(Duration::from_secs(60))
}
