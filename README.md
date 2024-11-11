# neilwoodhouse.uk
This repo contains the source code for my personal website. 

The purpose of this site is to host links to, and files for, projects that I'm involved in and to document past work. 

This site is built with Rust, using the [Rocket](https://rocket.rs/) web framework, the [Tera](https://tera.netlify.app/) templating engine,  and containerised using docker and nginx for easy deployment. 


## Installation
Note: this process assumes you have basic experience with docker and docker compose.

1. Pull the git repo.
1. Edit the host port in `docker-compose.yml` to your desired host port. (The default is port 8001)
1. Run `docker compose build` to build the container image.
1. Run `docker compose up` to run the server. 


## Update
### From Git:
1. Pull the changes from the repo
1. Run `docker compose down` to stop the server.
1. Run `docker compose build` to build the updated image.
1. Run `docker compose up` to restart the server.

### Locally
This assumes you have the Rust toolchain installed on your system, and allows you to update the dependencies independently from the repo's current state. 

```sh
cargo update
docker compose down
docker compose build
docker compose up
```
