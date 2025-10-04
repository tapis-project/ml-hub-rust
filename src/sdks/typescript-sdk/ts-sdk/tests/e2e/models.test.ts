// Load e2e environment variables into process.env
require('dotenv').config({ path: 'e2e.env' });

import 'mocha';
import { before } from 'mocha';
import { 
  Models
} from '../../src';
import { expect } from 'chai';
import fetch from 'cross-fetch';
import { CreateModelMetadataRequest, DiscoverModelsByPlatformRequest, GetModelByPlatformRequest, ListModelsByPlatformRequest } from '@mlhub/models-ts-sdk';

//   basePath: process.env.TEST_TENANT,
const basePath = 'https://dev.develop.tapis.io/v3/mlhub';

//////////////////////////////////////////////////////////
/*                      ExternalModels                  */
//////////////////////////////////////////////////////////

let externalModelsApi: Models.ExternalModelsApi;

let externalModels: Array<object>
let externalModel: object | undefined;

describe('Models e2e tests', async () => {
  before(async () => {
    // Tenants configuration for all tests
    const configurationParameters: Models.ConfigurationParameters = {
      basePath,
      headers: {},
      fetchApi: fetch
    }
    const configuration: Models.Configuration = new Models.Configuration(configurationParameters);
    
    externalModelsApi = new Models.ExternalModelsApi(configuration);
  });

  it("should fetch a list of models from huggingface", async () => {
    let request: ListModelsByPlatformRequest = {
      platform: "huggingface"
    };

    try {
        let response = await externalModelsApi.listModelsByPlatform(request)
        externalModels = response.result as Array<{[key: string]: any}>;
        expect(response.result).to.be.an("array");
    } catch (e) {
      expect.fail(`Test failed because an exception was thrown: ${e}`)
    }
  });

  it("should get the metadata for the first model returned from huggingface", async () => {
    externalModel = externalModels.pop();
    expect(externalModel).to.be.not.undefined;

    let request: GetModelByPlatformRequest = {
      platform: Models.Platform.Huggingface,
      modelId: (externalModel! as unknown as any)["id"]
    };

    try {
        let response = await externalModelsApi.getModelByPlatform(request)
        console.log({res: response.result})
        // expect(response.result).to.be.an("object");
    } catch (e) {
      console.log({e})
      expect.fail(`Test failed because an exception was thrown: ${e}`)
    }
  });

  it("should discover models on HuggingFace", async () => {
    let request: DiscoverModelsByPlatformRequest = {
      platform: Models.Platform.Patra,
      discoveryCriteria: {
        confidence_threshold: null,
        criteria: [
          {
            name: "Find image detection models"
          }
        ]
      }
    };

    try {
        let response = await externalModelsApi.discoverModelsByPlatform(request)
        console.log({res: response.result})
        // expect(response.result).to.be.an("object");
    } catch (e) {
      console.log({e})
      expect.fail(`Test failed because an exception was thrown: ${e}`)
    }
  });
});

//////////////////////////////////////////////////////////
/*                      Platforms                       */
//////////////////////////////////////////////////////////

let platformsApi: Models.PlatformsApi;

describe('Platforms e2e tests', async () => {
  before(async () => {
    const configurationParameters: Models.ConfigurationParameters = {
      basePath,
      headers: {},
      fetchApi: fetch
    }
    const configuration: Models.Configuration = new Models.Configuration(configurationParameters);
    
    platformsApi = new Models.PlatformsApi(configuration);
  });

  it("should fetch the list of available platforms on MLHub", async () => {
    try {
        let response = await platformsApi.listPlatforms()
        expect(response.result).to.be.an("array");
    } catch (e) {
      expect.fail(`Test failed because an exception was thrown: ${e}`)
    }
  });
});

//////////////////////////////////////////////////////////
/*                      Artifacts                       */
//////////////////////////////////////////////////////////

let artifactsApi: Models.ArtifactsApi;

describe('Artifacts e2e tests', async () => {
  before(async () => {
    // Tenants configuration for all tests
    const configurationParameters: Models.ConfigurationParameters = {
      basePath,
      headers: {},
      fetchApi: fetch
    }
    const configuration: Models.Configuration = new Models.Configuration(configurationParameters);
    
    artifactsApi = new Models.ArtifactsApi(configuration);
  });

  it("list all available model artifacts for a user", async () => {
    try {
        let response = await artifactsApi.listModelArtifacts()
        expect(response.result).to.be.an("array");
    } catch (e) {
      expect.fail(`Test failed because an exception was thrown: ${e}`)
    }
  });
});

//////////////////////////////////////////////////////////
/*                      Ingestions                      */
//////////////////////////////////////////////////////////

let ingestionsApi: Models.IngestionsApi;

describe('Ingestions e2e tests', async () => {
  before(async () => {
    // Tenants configuration for all tests
    const configurationParameters: Models.ConfigurationParameters = {
      basePath,
      headers: {},
      fetchApi: fetch
    }
    const configuration: Models.Configuration = new Models.Configuration(configurationParameters);
    
    ingestionsApi = new Models.IngestionsApi(configuration);
  });

  it("should fetch a list of all available ingestions for a user", async () => {
    try {
        let response = await ingestionsApi.listModelIngestions()
        expect(response.result).to.be.an("array");
    } catch (e) {
      console.log({e})
      expect.fail(`Test failed because an exception was thrown: ${e}`)
    }
  });
});

// //////////////////////////////////////////////////////////
// /*                      ModelMetadata                   */
// //////////////////////////////////////////////////////////

// let modelMetadataApi: Models.MetadataApi;

// describe('ModelMetadata e2e tests', async () => {
//   before(async () => {
//     // Tenants configuration for all tests
//     const configurationParameters: Models.ConfigurationParameters = {
//       basePath,
//       headers: {},
//       fetchApi: fetch
//     }
//     const configuration: Models.Configuration = new Models.Configuration(configurationParameters);
    
//     modelMetadataApi = new Models.MetadataApi(configuration);
//   });

//   let request: CreateModelMetadataRequest = {
//     artifactId: "",
//     modelMetadata: {}
//   };

//   it("should create a model metadata entry for an artifact", async () => {
//     try {
//         let response = await modelMetadataApi.createModelMetadata(request)
//         expect(response.result).to.be.an("object");
//     } catch (e) {
//       expect.fail(`Test failed because an exception was thrown: ${e}`)
//     }
//   });
// });

//////////////////////////////////////////////////////////
/*                      Publications                    */
//////////////////////////////////////////////////////////

let publicationsApi: Models.PublicationsApi;

describe('Publications e2e tests', async () => {
  before(async () => {
    // Tenants configuration for all tests
    const configurationParameters: Models.ConfigurationParameters = {
      basePath,
      headers: {},
      fetchApi: fetch
    }
    const configuration: Models.Configuration = new Models.Configuration(configurationParameters);
    
    publicationsApi = new Models.PublicationsApi(configuration);
  });

  it("should fetch a list of all available model publications for a user", async () => {
    try {
        let response = await publicationsApi.listModelPublications()
        expect(response.result).to.be.an("array");
    } catch (e) {
      console.log({e})
      expect.fail(`Test failed because an exception was thrown: ${e}`)
    }
  });
});