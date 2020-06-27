#!make

include .env

################ NODE ################

node-docker-build:
	docker build -t mynode:latest -f dockerfiles/node.Dockerfile dockerfiles

node-dev: node-docker-build
	docker run -v ${DIR}/node:/project -p${NODE_SERVE_PORT}:8000 -it mynode:latest /bin/bash -c "yarn install && gatsby develop --host=0.0.0.0"

node-prod: node-docker-build
	docker run -v ${DIR}/node:/project -it mynode:latest /bin/bash -c "yarn install && gatsby build"


################ RUST ################

rust-build:
	cargo build

################ ALL ################

build: node-build rust-build

run:
	cargo run

