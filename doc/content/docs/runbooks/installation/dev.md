---
title: Dev
description: Dev environment installation process
---

# Prerequisites
- A Kubernetes cluster running with k3d
- The Kubectl and Helm CLI installed on the host

# Process

### With Make

### Clone the repository [step]

```bash
git clone https://github.com/TimotheeRen/KubePods.git
```

### Start the cluster [step]

```bash
make dev
```

It will automatically create a cluster, then install and configure the flux-operator in it.

### Manually

### Clone the repository [step]

```bash
git clone https://github.com/TimotheeRen/KubePods.git
```

### Create the cluster with K3d [step]

``` bash
k3d cluster create --config k3d/dev-env.yaml \
        --port "8080:30080@loadbalancer" \
        --port "8443:30443@loadbalancer"
```

### Install the flux-operator chart [step]

```bash
helm install flux-operator oci://ghcr.io/controlplaneio-fluxcd/charts/flux-operator \
	  --namespace flux-system \
	  --create-namespace
```

### Apply Flux's dev configuration file [step]

```bash
kubectl apply -f flux-dev.yaml
```

# Verification

### Check if the Kubernetes cluster is running: [step]

```bash
kubectl cluster-info
```

### If you have the Flux CLI installed, check for any reconciliation error: [step]

```bash
flux get all
```

Make sure everything is marked as "Ready".
