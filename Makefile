#!make

include .env

################ NODE ################

node-docker-build:
	docker build -t mynode:latest -f dockerfiles/node.Dockerfile dockerfiles

node-install: node-docker-build
	docker run -v ${DIR}/node:/project -it mynode:latest /bin/bash -c "yarn install"

# example : make node-cli CMD="npm --version"
node-cli: node-install
	docker run -v ${DIR}/node:/project -it mynode:latest /bin/bash -c "${CMD}"

node-dev: node-install
	docker run -v ${DIR}/node:/project -p${NODE_SERVE_PORT}:3000 -it mynode:latest /bin/bash -c "npm run develop"

node-prod: node-install
	docker run -v ${DIR}/node:/project -it mynode:latest /bin/bash -c "npm run build"


################ RUST ################

rust-build:
	cargo build

################ ALL ################

build: node-build rust-build

run:
	cargo run

