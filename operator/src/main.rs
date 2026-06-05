use futures::StreamExt;
use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{
            Container, ContainerPort, PodSpec, PodTemplateSpec, ResourceRequirements, Service,
            ServicePort, ServiceSpec,
        },
    },
    apimachinery::pkg::{
        api::resource::Quantity, apis::meta::v1::LabelSelector, util::intstr::IntOrString,
    },
};
use std::{collections::BTreeMap, sync::Arc, time::Duration, vec};

use kube::{
    Api, Client, Error,
    api::{ObjectMeta, Patch, PatchParams},
    runtime::{Controller, controller::Action, watcher::Config},
};
use operator::Desktop;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let desktops: Api<Desktop> = Api::namespaced(client.clone(), "desktops");
    let config = Config::default();
    let context = Arc::new(ContextData { client });
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

struct ContextData {
    client: Client,
}

async fn reconcile(desktop: Arc<Desktop>, context: Arc<ContextData>) -> Result<Action, Error> {
    let client = context.client.clone();

    let mut limits = BTreeMap::new();
    limits.insert("memory".to_string(), Quantity(desktop.spec.max_ram.clone()));
    limits.insert("cpu".to_string(), Quantity(desktop.spec.max_cpu.clone()));

    let mut labels = BTreeMap::new();
    labels.insert("app".to_string(), desktop.spec.id.clone());

    let deployment = Deployment {
        metadata: ObjectMeta {
            name: Some(desktop.spec.id.clone()),
            namespace: Some("desktops".to_string()),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            selector: LabelSelector {
                match_labels: Some(labels.clone()),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(labels.clone()),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: desktop.spec.name.clone(),
                        image: Some(desktop.spec.image.clone()),
                        resources: Some(ResourceRequirements {
                            limits: Some(limits),
                            ..Default::default()
                        }),
                        ports: Some(vec![ContainerPort {
                            name: Some("http".to_string()),
                            container_port: 3000,
                            host_port: Some(3000),
                            protocol: Some("TCP".to_string()),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    };
    let deployments: Api<Deployment> = Api::namespaced(client.clone(), "desktops");
    match deployments
        .patch(
            &desktop.spec.id.clone(),
            &PatchParams::apply("deployment-manager").force(),
            &Patch::Apply(deployment),
        )
        .await
    {
        Ok(_) => println!("Sucessfully patched deployment"),
        Err(e) => println!("An error occured when patching a deployment: {}", e),
    };

    let service = Service {
        metadata: ObjectMeta {
            name: Some(desktop.spec.id.clone()),
            namespace: Some("desktops".to_string()),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            selector: Some(labels),
            ports: Some(vec![ServicePort {
                protocol: Some("TCP".to_string()),
                port: 3000,
                target_port: Some(IntOrString::Int(3000)),
                ..Default::default()
            }]),
            type_: Some("ClusterIP".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    let services: Api<Service> = Api::namespaced(client, "desktops");
    match services
        .patch(
            &desktop.spec.id.clone(),
            &PatchParams::apply("service-manager"),
            &Patch::Apply(service),
        )
        .await
    {
        Ok(_) => println!("Sucessfully patched service"),
        Err(e) => println!("An error occured when patching a deployment: {}", e),
    };
    Ok(Action::requeue(Duration::from_secs(300)))
}

fn error_policy(_desktop: Arc<Desktop>, _error: &Error, _context: Arc<ContextData>) -> Action {
    Action::requeue(Duration::from_secs(60))
}
