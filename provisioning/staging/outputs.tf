output "vm_names" {
  description = "VM names"
  value = [
    for node in local.nodes :
    libvirt_domain.node[node].name
  ]
}

output "ansible_group" {
  description = "Ansible group"
  value       = ansible_group.nodes.name
}

output "ansible_hosts" {
  description = "Ansible hosts"
  value = [
    for node in local.nodes :
    ansible_host.vm[node].name
  ]
}
