# MLHub Typescript SDK

This is the official Typescript SDK for the MLHub suite of APIs generated from the Open API Specification of each of the APIs.

> The Typescript code generator used in the project requires Open API specs to be formatted using version 3.0.3 of the Open API Specification

## Step 1. Generate the specs

From the root of the project, run `./manage generate-spec <service>`. This will generate the latest openapi spec for that service in its respective `specs` directory.

Ex: `./manage generate-spec models`

## Step 2. Build, then run and exec into the SDK bulider container

1. `./manage build typescript-sdk`
2. `./manage exec typescript-sdk`

## Step 3. Update the service configs

Increment the version number in the **services/config.json** file for each service you want to update

## Step 4. Run the spec sdk generation script

Run `./generate_all` to generate the typescript sdk for all services.
Run `./generate <service>` to generate the typescript for a single service. Ex: `./generate models`

## Step 5. Testing the new SDKs

1. `cd` into the **ts-sdk** directory
1. Update dependencies in the **ts-sdk/package.json** to file references for each service's gen directory. Ex. `"@mlhub/models-ts-sdk": "0.1.1"` **->** `"@mlhubmodels-ts-sdk": "file://../gen/models"`,
1. `npm install`
1. `npm run build`
1. `npm run test`
1. `npm run e2e`