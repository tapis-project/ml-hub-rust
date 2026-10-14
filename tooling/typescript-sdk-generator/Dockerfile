FROM node:22-alpine3.24

RUN apk add --no-cache \
    bash \
    openjdk17-jre-headless \
    python3

RUN mkdir -p /src/tooling/typescript-sdk-generator

WORKDIR /src/tooling/typescript-sdk-generator

COPY package.json package-lock.json ./

RUN npm ci

RUN npm install --global \
    @openapitools/openapi-generator-cli@2.5.2

ENTRYPOINT ["bash"]