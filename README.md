# neilwoodhouse.uk
This repo contains the source code for my personal website. 

The purpose of this site is to host links to, and files for, projects that I'm involved in and document past work. 

This site is built with Rust, using the [Rocket](https://rocket.rs/) web framework, the [Tera](https://tera.netlify.app/) templating engine,  and containerised using docker and nginx for easy deployment. 


## Installation
Note: this process assumes you have basic experience with docker and Rust and have both installed on your system. 

1. Pull the git repo
1. Run `cargo build -r` to compile the project source to a release version. 
1. Run `docker compose build` to build a container from the compiled app.
1. Edit the host port in `docker-compose.yml` to your desired host port. 
1. Run `docker compose up` to run the server. 