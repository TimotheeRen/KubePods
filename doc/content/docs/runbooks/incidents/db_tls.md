---
title: Databases certificate error
description: Can't reach the databases because of expired certificates
---

# Symptoms
- Error when trying to reach a database: "error dialing backend: tls [...]"

# Prerequisites
- A K3d cluster running KubePods
- Databases up and running

# Process

### Find cluster's master node container id [step]

```bash
docker ps | grep server
```

### Rotate cluster's certificates [step]

```bash
docker exec -it [id] k3s certificate rotate
```

### Reload master node container [step]

```bash
docker restart [id]
```

# Verification

### Try to reach the database again [step]

```bash
	kubectl port-forward svc/[db-svc] 5433:5432
```
