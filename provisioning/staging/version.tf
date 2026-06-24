terraform {
  required_version = ">= 1.15.6"
  required_providers {
    libvirt = {
      source  = "dmacvicar/libvirt"
      version = "~> 0.9.8"
    }
    ansible = {
      source  = "ansible/ansible"
      version = "~> 1.4"
    }
  }
}

provider "libvirt" {
  uri = "qemu:///system"
}
