variable "db_password" {
  description = "MySQL password"
  type        = string
  sensitive   = true
}

variable "db_user" {
  description = "MySQL user"
  type        = string
  default     = "user"
}

variable "db_name" {
  description = "MySQL database name"
  type        = string
  default     = "the_hub"
}

variable "db_port" {
  description = "MySQL database port"
  type        = number
  default     = 3306
}
