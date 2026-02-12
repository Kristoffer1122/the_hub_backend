## A Webserver written in rust
A webserver written in rust,
all it does is return JSON from a Mariadb or Mysql database.


### Deployment
I recommend using Terraform and Docker to deploy this software
However you can also just run the Dockerfile or use the binary directly, but I will not cover that in this README.


```bash
terraform apply
```

### Use your own database
If you want to use your own database make your own `terrafrom.tfvars` file in the project root.

```toml
db_name = "NAME HERE"
db_port = 3306
db_user = "USERNAME HERE"
db_password = "PASSWORD HERE"
```

However you will have to change the Table struct to follow your table structure.

Example:
```toml
    pub struct Table {
        pub id: i32,
        pub title: String,
        pub genre: String,
        pub utgivelsesdato: Option<chrono::NaiveDate>,
    }
```


### This does not save any of your data
https://www.shera.no/privacy-policy
