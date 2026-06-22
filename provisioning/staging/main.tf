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
}

resource "libvirt_volume" "disk" {
  for_each = toset(local.nodes)
  name     = "${each.value}-disk"
  pool     = "default"
  capacity = 21474836480
  create = {
    content = {
      url = libvirt_cloudinit_disk.init[each.key].path
    }
  }
}

resource "libvirt_domain" "master_node" {
  for_each    = toset(local.nodes)
  name        = "${each.value}-vm"
  memory      = var.nodes_spec.memory
  memory_unit = "Mib"
  vcpu        = var.nodes_spec.vcpu
  type        = "kvm"

  os = {
    type         = "hvm"
    type_arch    = "x86_64"
    type_machine = "q35"
  }

  devices = {
    disks = [
      {
        source = {
          file = {
            file = "/var/lib/libvirt/images/ubuntu.img"
          }
        }
        target = {
          dev = "vda"
          bus = "virtio"
        }
      }
    ]
    interfaces = [
      {
        model = {
          type = "virtio"
        }
        source = {
          network = {
            network = "default"
          }
        }
      }
    ]
  }
}
