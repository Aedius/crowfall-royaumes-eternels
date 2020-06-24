#!make

include .env

################ IONIC ################

ionic-docker-build:
	docker build -t myionic:latest -f dockerfiles/ionic.Dockerfile dockerfiles

ionic-build: ionic-docker-build
	docker run -v ${DIR}/ionic:/project -it myionic:latest ionic build --engine=browser --prod

ionic-serve:
	docker run -v ${DIR}/ionic:/project -p8080:8100 -it myionic:latest ionic serve

################ RUST ################

rust-build:
	cargo build

all-build: ionic-build rust-build

################ ALL ################

run:
	cargo run

