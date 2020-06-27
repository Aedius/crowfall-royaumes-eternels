#!make

include .env

################ NODE ################

node-docker-build:
	docker build -t mynode:latest -f dockerfiles/node.Dockerfile dockerfiles

node-build: node-docker-build
	docker run -v ${DIR}/node:/project/crowfall-royaules-eternels -it mynode:latest gatsby new crowfall-royaules-eternels https://github.com/gatsbyjs/gatsby-starter-hello-world

node-dev:node-docker-build
	docker run -v ${DIR}/node:/project -p${NODE_SERVE_PORT}:8000 -it mynode:latest gatsby develop --host=0.0.0.0

################ RUST ################

rust-build:
	cargo build

################ ALL ################

build: node-build rust-build

run:
	cargo run

