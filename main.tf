variable "container_name" {
  description = "Name of our Rust backend"
  type        = string
  default     = "Rust_backend"
}

terraform {
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 3.0.1"
    }
  }
}

provider "docker" {}

resource "docker_image" "app" {
  name = "rust_backend:latest"
  build {
    context    = path.module
    dockerfile = "Dockerfile"
  }
  keep_locally = false
}

resource "docker_container" "app" {
  image    = docker_image.app.image_id
  name     = var.container_name
  must_run = true
  env = [
    "DB_USER=${var.db_user}",
    "DB_PASSWORD=${var.db_password}",
    "DB_PORT=${var.db_port}",
    "DB_NAME=${var.db_name}"
  ]
  ports {
    internal = 7878
    external = 7878
  }

  # Allow container to reach your host machine's MariaDB
  host {
    host = "host.docker.internal"
    ip   = "host-gateway"
  }
}
