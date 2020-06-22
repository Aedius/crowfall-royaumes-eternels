#!make

include .env

ionic-docker-build:
	docker build -t myionic:latest ionic

ionic-build: ionic-docker-build
	docker run -v ${DIR}/static/design:/project -it myionic:latest ionic build --prod
