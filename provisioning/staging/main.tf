resource "libvirt_cloudinit_disk" "init" {
  for_each = toset(local.nodes)
  name     = "${each.value}-init"
  user_data = templatefile("${path.module}/user-data.yaml", {
    ssh_key = trimspace(file(pathexpand("~/.ssh/id_ed25519.pub")))
  })
  meta_data = yamlencode({
    instance-id    = each.value
    local-hostname = each.value
  })
  network_config = templatefile("${path.module}/network-config.yaml", {
    address = local.addresses[each.value]
  })
}

resource "libvirt_volume" "disk" {
  for_each = toset(local.nodes)
  name     = "${each.value}.qcow2"
  pool     = "default"
  capacity = 10737418240
  target = {
    format = {
      type = "qcow2"
    }
  }
  create = {
    content = {
      url = "/var/lib/libvirt/images/ubuntu.qcow2"
    }
  }
}

resource "libvirt_domain" "node" {
  for_each    = toset(local.nodes)
  name        = "${each.value}-vm"
  memory      = var.nodes_spec.memory
  memory_unit = "MiB"
  vcpu        = var.nodes_spec.vcpu
  type        = "kvm"

  os = {
    type         = "hvm"
    type_arch    = "x86_64"
    type_machine = "q35"
  }

  features = {
    acpi = true
  }

  devices = {
    disks = [
      {
        source = {
          volume = {
            volume = libvirt_volume.disk[each.value].name
            pool   = libvirt_volume.disk[each.value].pool
          }
        }
        driver = {
          name = "qemu"
          type = "qcow2"
        }
        target = {
          dev = "vda"
          bus = "virtio"
        }
      },
      {
        device = "cdrom"
        driver = {
          name = "qemu"
          type = "raw"
        }
        source = {
          file = {
            file = libvirt_cloudinit_disk.init[each.value].path
          }
        }
        target = {
          dev = "sda"
          bus = "sata"
        }
      }
    ]
    interfaces = [{
      model  = { type = "virtio" }
      source = { network = { network = "default" } }
    }]
    serials = [{
      type = "pty"
      target = {
        port = 0
      }
    }]
    consoles = [{
      type = "pty"
      target = {
        type = "serial"
        port = 0
      }
    }]
    graphics = [{
      spice = {
        auto_port = true
        listen    = "127.0.0.1"
      }
      videos = [{
        model = {
          type = "virtio"
        }
      }]
    }]
  }
}

resource "ansible_group" "nodes" {
  name = "nodes"
}

resource "ansible_host" "vm" {
  for_each = toset(local.nodes)
  name     = "${each.value}-vm"
  groups   = [ansible_group.nodes.name]
  variables = {
    ansible_host = local.addresses[each.value]
    ansible_user = "ubuntu"
  }
}
