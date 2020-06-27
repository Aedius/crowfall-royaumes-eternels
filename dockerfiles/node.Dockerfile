FROM node:14-stretch-slim

RUN apt-get update
RUN apt-get install -y apt-transport-https ca-certificates curl git gnupg

RUN npm install -g npm

RUN npm install -g gatsby-cli
RUN gatsby telemetry --disable

RUN curl -o- -L https://yarnpkg.com/install.sh | bash

WORKDIR /project