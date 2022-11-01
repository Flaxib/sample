Documentation link:  
https://fly.io/docs/hands-on/

# Install flyctl

```sh
curl -L https://fly.io/install.sh | sh
```

# Launch an application

A fly application is based on a Docker image/Dockerfile.

This command create a fly.toml which will contain the application configuration:

```sh
flyctl launch --image flyio/hellofly:latest
```

To deploy the application:

```sh
flyctl deploy
```

# Check an application

```sh
flyctl status
```

# Delete an application

```sh
flyctl destroy <application_name>
```

One will find the application name from the `flyctl status` command.
