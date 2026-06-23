---
title: Staging
description: Staging environment installation process
---

# Prerequisites

- A host with virtualization enabled
- Qemu and KVM installed and configured
- Terraform installed with a version >= 1.15.6
- Libvirtd service running
- Host's public SSH key in `~/.ssh/id_ed25519.pub`

# Process

### Install a Debian based cloud image [step]

```bash
cd /var/lib/libvirt/images
sudo wget https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img
```

### Create a backing file [step]

```bash
sudo qemu-img create -f qcow2 -F qcow2 \
  -b /var/lib/libvirt/images/noble-server-cloudimg-amd64.img \
  /var/lib/libvirt/images/ubuntu.qcow2 20G
```

### Clone the repository [step]

```bash
git clone https://github.com/TimotheeRen/KubePods.git
```

### Get in the `provisioning/staging` folder [step]

```bash
cd provisioning/staging
```

### Initialize Terraform [step]

```bash
terraform init
```

### Plan the migration [step]

```bash
terraform plan
```


### Apply the migration [step]

```bash
terraform apply
```

# Verification

### You should have two running VM [step]

```bash
virsh list --all
```
