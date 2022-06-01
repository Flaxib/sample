# Environment

## To install postgreSQL:

```sh
sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
sudo apt-get update
sudo apt-get -y install postgresql
sudo apt install libpq-dev
cargo install diesel_cli --no-default-features --features "postgres"
```

Install PostgreSQL:  
https://www.postgresql.org/download/linux/ubuntu/

## Install pgadmin4

```sh
curl  -fsSL https://www.pgadmin.org/static/packages_pgadmin_org.pub | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/pgadmin.gpg
sudo sh -c 'echo "deb https://ftp.postgresql.org/pub/pgadmin/pgadmin4/apt/$(lsb_release -cs) pgadmin4 main" > /etc/apt/sources.list.d/pgadmin4.list'
sudo apt update
sudo apt install pgadmin4
```

Install pgadmin4:  
https://www.howtoforge.com/how-to-install-postgresql-and-pgadmin4-on-ubuntu-1804-lts/#install-pgadmin-from-repository

## Start using Diesel



Page of package Diesel:  
https://crates.io/crates/diesel_cli

Tutorial get started with Diesel:  
https://diesel.rs/guides/getting-started