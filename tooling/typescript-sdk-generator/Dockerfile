FROM node:19.0

RUN apt-get update

RUN apt-get install -y vim git default-jre

RUN mkdir -p /src/tooling/typescript-sdk-generator

WORKDIR /src/tooling/typescript-sdk-generator

COPY package.json package-lock.json ./

RUN npm ci

RUN npm install -g @openapitools/openapi-generator-cli@2.5.2 --save-exact

ENTRYPOINT [ "bash" ]
