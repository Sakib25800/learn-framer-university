# FROM node:latest
# WORKDIR /app
# COPY . .
# CMD ["npm", "run", "dev"]

ARG NODE_VERSION=18

FROM node:${NODE_VERSION}-alpine

WORKDIR /app

COPY package.json package-lock.json /app/

RUN npm ci

COPY . /app

ENTRYPOINT ["npm", "run", "dev"]
