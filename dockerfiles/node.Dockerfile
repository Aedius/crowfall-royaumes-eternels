FROM node:14

RUN apt-get update
RUN apt-get install -y apt-transport-https ca-certificates curl git gnupg

RUN npm install -g npm

RUN curl -o- -L https://yarnpkg.com/install.sh | bash

WORKDIR /project