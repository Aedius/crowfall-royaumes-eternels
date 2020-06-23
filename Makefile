#!make

include .env

ionic-docker-build:
	docker build -t myionic:latest -f dockerfiles/ionic.Dockerfile dockerfiles

ionic-build: ionic-docker-build
	docker run -v ${DIR}/ionic:/project -it myionic:latest ionic build --engine=browser --prod

rust-build:
	cargo build

build: ionic-build
	make rust-build

run:
	cargo run