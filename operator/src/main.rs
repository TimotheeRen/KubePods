use futures::StreamExt;
use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{
            Container, ContainerPort, PodSpec, PodTemplateSpec, ResourceRequirements, Service,
            ServicePort, ServiceSpec,
        },
        networking::v1::{
            HTTPIngressPath, HTTPIngressRuleValue, Ingress, IngressBackend, IngressRule,
            IngressServiceBackend, IngressSpec, ServiceBackendPort,
        },
    },
    apimachinery::pkg::{
        api::resource::Quantity, apis::meta::v1::LabelSelector, util::intstr::IntOrString,
    },
};
use std::{collections::BTreeMap, sync::Arc, time::Duration, vec};

use kube::{
    Api, Client, Error, Resource,
    api::{ObjectMeta, Patch, PatchParams},
    runtime::{Controller, controller::Action, watcher::Config},
};
use operator::Desktop;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let desktops: Api<Desktop> = Api::namespaced(client.clone(), "desktops");
    let config = Config::default();

    let deployments: Api<Deployment> = Api::namespaced(client.clone(), "desktops");
    let services: Api<Service> = Api::namespaced(client.clone(), "desktops");
    let ingresses: Api<Ingress> = Api::namespaced(client.clone(), "desktops");
    let context = Arc::new(ContextData {
        deployments: deployments.clone(),
        services: services.clone(),
        ingresses: ingresses.clone(),
    });

    Controller::new(desktops.clone(), config.clone())
        .owns(deployments, config.clone())
        .owns(services, config)
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => println!("reconciled: {:?}", o),
                Err(kube::runtime::controller::Error::ObjectNotFound(_)) => {
                    println!("Object already deleted from store, skipping.")
                }
                Err(e) => println!("reconciliation failed: {}", e),
            }
        })
        .await;
    Ok(())
}

struct ContextData {
    deployments: Api<Deployment>,
    services: Api<Service>,
    ingresses: Api<Ingress>,
}

async fn reconcile(desktop: Arc<Desktop>, context: Arc<ContextData>) -> Result<Action, Error> {
    if desktop.metadata.deletion_timestamp.is_some() {
        println!("Object {} is being deleted", desktop.spec.id);
        return Ok(Action::await_change());
    }

    let mut limits = BTreeMap::new();
    limits.insert("memory".to_string(), Quantity(desktop.spec.max_ram.clone()));
    limits.insert("cpu".to_string(), Quantity(desktop.spec.max_cpu.clone()));

    let mut labels = BTreeMap::new();
    labels.insert("app".to_string(), desktop.spec.id.clone());

    let mut annotations = BTreeMap::new();
    annotations.insert(
        "traefik.ingress.kubernetes.io/router.middlewares".to_string(),
        "default-desktop-chain@kubernetescrd".to_string(),
    );

    let distribution = desktop.spec.distribtion.clone();
    let mut desktop_environment = desktop.spec.desktop_environment.clone();

    if distribution == "alpine" && desktop_environment == "xfce" {
        desktop_environment = "latest".to_string();
    }

    let deployment = Deployment {
        metadata: ObjectMeta {
            name: Some(desktop.spec.id.clone()),
            namespace: Some("desktops".to_string()),
            owner_references: Some(vec![desktop.controller_owner_ref(&()).unwrap()]),
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
                        image: Some(format!(
                            "linuxserver/webtop:{distribution}-{desktop_environment}"
                        )),
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
    match context
        .deployments
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
            owner_references: Some(vec![desktop.controller_owner_ref(&()).unwrap()]),
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
    match context
        .services
        .patch(
            &desktop.spec.id.clone(),
            &PatchParams::apply("service-manager"),
            &Patch::Apply(service),
        )
        .await
    {
        Ok(_) => println!("Sucessfully patched service"),
        Err(e) => println!("An error occured when patching a service: {}", e),
    };

    let ingress = Ingress {
        metadata: ObjectMeta {
            name: Some(desktop.spec.id.clone()),
            namespace: Some("desktops".to_string()),
            owner_references: Some(vec![desktop.controller_owner_ref(&()).unwrap()]),
            annotations: Some(annotations),
            ..Default::default()
        },
        spec: Some(IngressSpec {
            ingress_class_name: Some("traefik".to_string()),
            rules: Some(vec![IngressRule {
                http: Some(HTTPIngressRuleValue {
                    paths: vec![HTTPIngressPath {
                        path: Some(format!("/desktop/{}", desktop.spec.id.clone())),
                        path_type: "Prefix".to_string(),
                        backend: IngressBackend {
                            service: Some(IngressServiceBackend {
                                name: desktop.spec.id.clone(),
                                port: Some(ServiceBackendPort {
                                    number: Some(3000),
                                    ..Default::default()
                                }),
                            }),
                            ..Default::default()
                        },
                    }],
                }),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    };
    match context
        .ingresses
        .patch(
            &desktop.spec.id.clone(),
            &PatchParams::apply("ingress-manager"),
            &Patch::Apply(ingress),
        )
        .await
    {
        Ok(_) => println!("Sucessfully patched ingress"),
        Err(e) => println!("An error occured when patching an ingress: {}", e),
    };
    Ok(Action::requeue(Duration::from_secs(300)))
}

fn error_policy(_desktop: Arc<Desktop>, _error: &Error, _context: Arc<ContextData>) -> Action {
    Action::requeue(Duration::from_secs(60))
}
