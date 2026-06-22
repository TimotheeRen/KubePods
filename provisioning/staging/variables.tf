variable "nodes_spec" {
  type = object(
    {
      memory = number
      vcpu   = number
  })
  default = {
    memory = 2048
    vcpu   = 2
  }
  validation {
    condition     = var.nodes_spec.memory > 0
    error_message = "Allocated memory can't be negative."
  }
  validation {
    condition     = var.nodes_spec.vcpu > 0
    error_message = "Allocated vcpu can't be negative."
  }
}
