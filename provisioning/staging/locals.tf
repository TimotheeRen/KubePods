locals {
  nodes = ["master_node", "worker_node"]
  addresses = {
    master_node : "192.168.122.10",
    worker_node : "192.168.122.11"
  }
}
