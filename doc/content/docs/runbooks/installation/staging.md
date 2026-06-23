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

1. Install a Debian based cloud image

```bash
cd /var/lib/libvirt/images
sudo wget https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img
```

2. Create a backing file

```bash
sudo qemu-img create -f qcow2 -F qcow2 \
  -b /var/lib/libvirt/images/noble-server-cloudimg-amd64.img \
  /var/lib/libvirt/images/ubuntu.qcow2 20G
```

3. Clone the repository

```bash
git clone https://github.com/TimotheeRen/KubePods.git
```

4. Get in the `provisioning/staging` folder

```bash
cd provisioning/staging
```

5. Initialize Terraform

```bash
terraform init
```

6. Plan the migration

```bash
terraform plan
```


7. Apply the migration

```bash
terraform apply
```

# Verification

1. You should have two running VM

```bash
virsh list --all
```
