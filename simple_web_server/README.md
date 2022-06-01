# Server

Ici le côté server de l'application `flaxib-scolaire`. Le but de cette partie serveur est :
 - de répondre à des requêtes de données des applications clientes : bus, parents et school,
 - de pouvoir calculer l'itinéraire du bus à interval régulier dans le temps (par exemple, tous les jours à 3h du mat).

## Prérequis

Ce projet utilise `rust` et son gestionnaire de paquet/projet `cargo`. Pour installer `cargo`, vous pouvez suivre les instructions de [cette page](https://doc.rust-lang.org/cargo/getting-started/installation.html).

En résumé, si vous êtes sous linux, dans un terminal tapez :
```shell
curl https://sh.rustup.rs -sSf | sh
```

## Executer le serveur

Pour lancer le serveur, si vous êtes sous linux, dans un terminal tapez :
```shell
cargo run
```

Une fois le serveur lancer, vous pouvez vérifier qu'il fonction en visitant l'adresse http://127.0.0.1:8000/