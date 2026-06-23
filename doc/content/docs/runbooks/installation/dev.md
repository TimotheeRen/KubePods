---
title: Dev
description: Dev environment installation process
---

## Prerequisites
- A Kubernetes cluster running with k3d
- The Kubectl and Helm CLI installed on the host

## Process

### With Make

1. Clone the repository

```bash
git clone https://github.com/TimotheeRen/KubePods.git
```

2. Start the cluster

```bash
make dev
```

It will automatically create a cluster, then install and configure the flux-operator in it.

### Manually

1. Clone the repository

```bash
git clone https://github.com/TimotheeRen/KubePods.git
```

2. Create the cluster with K3d

``` bash
k3d cluster create --config k3d/dev-env.yaml \
        --port "8080:30080@loadbalancer" \
        --port "8443:30443@loadbalancer"
```

3. Install the flux-operator chart

```bash
helm install flux-operator oci://ghcr.io/controlplaneio-fluxcd/charts/flux-operator \
	  --namespace flux-system \
	  --create-namespace
```

4. Apply Flux's dev configuration file

```bash
kubectl apply -f flux-dev.yaml
```

# Verification

1. Check if the Kubernetes cluster is running:

```bash
kubectl cluster-info
```

2. If you have the Flux CLI installed, check for any reconciliation error:

```bash
flux get all
```

Make sure everything is marked as "Ready".
