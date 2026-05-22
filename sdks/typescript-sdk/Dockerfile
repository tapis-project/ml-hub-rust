FROM node:19.0

RUN apt-get update

RUN apt-get install -y vim git default-jre

RUN mkdir -p /src/sdks/typescript-sdk

# COPY . /src/sdks/typescript-sdk

# COPY ./package.json /src/sdks/typescript-sdk

WORKDIR /src/sdks/typescript-sdk

RUN npm install -g @openapitools/openapi-generator-cli@2.5.2 --save-exact

# RUN npm install

ENTRYPOINT [ "bash" ]
