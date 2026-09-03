# MLHub TypeScript SDK Generator

This tooling generates the service-specific TypeScript SDK packages under `sdks/<service>-ts-sdk` from the OpenAPI documents under `specs/<service>/openapi.json`.

> The Typescript code generator used in the project requires Open API specs to be formatted using version 3.0.3 of the Open API Specification

## Step 1. Generate the specs

From the root of the project, generate every OpenAPI specification or one service specification:

```sh
./manage generate specs
./manage generate-models specs
```

## Step 2. Build, then run and exec into the SDK bulider container

1. `./manage build typescript-sdk-generator`
2. `./manage exec typescript-sdk-generator`

## Step 3. Update the service configs

Increment the version number in every **services/*/config.json** file. The version bump script supports `minor` and `patch` increments:

```sh
npm run bump-version -- patch
npm run bump-version -- minor
```

The same operation is available through the lifecycle management CLI:

```sh
./manage bump typescript-sdk-generator -a patch
./manage bump typescript-sdk-generator -a minor
```

A minor bump resets the patch component to zero. A patch bump increments only the patch component.

## Step 4. Run the spec sdk generation script

Use the lifecycle component to generate all SDKs or an individual service SDK:

```sh
./manage generate typescript-sdk-generator
./manage generate-models typescript-sdk-generator
```

## Step 5. Testing the new SDKs

1. `cd sdks/ts-sdk`
1. `npm install`
1. `npm run build`
1. `npm run test`
1. `npm run e2e`
