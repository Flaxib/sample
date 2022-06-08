This project is a simple example of usage of a Postgres database in Rust with the Diesel package.

# Environment

Requierements:
 - Rust compiler
 - Docker
 - Node

## Install Diesel CLI

```sh
sudo apt install libpq-dev
cargo install diesel_cli --no-default-features --features "postgres"
```

Documentation links:  
[Tutorial get started with Diesel](https://diesel.rs/guides/getting-started)

## Launch servers

To create the db with `diesel`, one time:
```sh
docker-compose up -d --remove-orphans
diesel setup
```

## Test the project

```sh
cargo run --bin show_posts
cargo run --bin write_post
cargo run --bin publish_post 0
cargo run --bin show_posts
cargo run --bin delete_post Agate

cargo run --bin dump_posts
cargo run --bin seed 10
cargo run --bin dump_posts
cargo run --bin wipe
```

## Stop servers

```sh
docker-compose down
```
# Documentation links

[The package Diesel](https://crates.io/crates/diesel_cli)

[Tutorial get started with Diesel](https://diesel.rs/guides/getting-started)

# Previously

## Launch PostgreSQL server

```sh
docker run --name diesel-demo -p 5432:5432 -e POSTGRES_PASSWORD=toto -e POSTGRES_USER=frozar -e POSTGRES_DB=diesel-demo -d postgres
```


Retrieve the [IP address of the docker postgres container](https://stackoverflow.com/questions/53610385/docker-postgres-and-pgadmin-4-connection-refused#answer-56334518
):
```sh
docker network inspect bridge | node getContainerIP.js diesel-demo
```

[Documentation link](https://dev.to/shree_j/how-to-install-and-run-psql-using-docker-41j2)

## Launch Pgadmin4

```sh
docker run --rm -p 5050:80 -e PGADMIN_DEFAULT_EMAIL=fabien@flaxib.re -e PGADMIN_DEFAULT_PASSWORD=toto dpage/pgadmin4
```

[Documentation link](https://www.pgadmin.org/docs/pgadmin4/development/container_deployment.html)
