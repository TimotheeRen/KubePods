from diagrams import Diagram, Cluster
from diagrams.onprem.network import Traefik
from diagrams.programming.framework import React
from diagrams.programming.language import Rust
from diagrams.onprem.database import Postgresql
from diagrams.k8s.others import CRD
from diagrams.k8s.compute import Deploy as Deployment
from diagrams.k8s.network import Ing as Ingress
from diagrams.k8s.network import SVC as Service

graph_attr = {
    "dpi": "200",
}

with Diagram("", show=False, filename="../public/diagrams/overview", graph_attr=graph_attr, direction="LR"):

    with Cluster("Endpoints"):
        api = Rust("API")
        ingress = Traefik("Ingress")
        ingress >> [
            React("Frontend"),
            api
        ]

    with Cluster("Service Layer"):
        desktop_service = Rust("Desktops Service")
        user_service = Rust("Users Service")

    with Cluster("Data Layer"):
        desktop_cr = CRD("Desktop CR")
        desktop_db = Postgresql("Desktops Database")
        user_db = Postgresql("Users Database")

    desktop_service >> [
        desktop_cr,
        desktop_db
    ]

    user_service >> user_db
    api >> [desktop_service, user_service]
    operator = Rust("K8s Operator")
    desktop_cr >> operator

    with Cluster("Desktops"):
        deployment = Deployment("Desktop Deployment")
        ingress = Ingress("Desktop Ingress")
        service = Service("Desktop Service")
        ingress >> service

    operator >> [deployment, ingress]
