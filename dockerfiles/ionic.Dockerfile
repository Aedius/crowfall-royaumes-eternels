FROM node:14-stretch-slim

RUN npm install -g @ionic/cli

WORKDIR /project/