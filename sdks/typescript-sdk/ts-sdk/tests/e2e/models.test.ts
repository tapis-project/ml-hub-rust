// Load e2e environment variables into process.env
require('dotenv').config({ path: 'e2e.env' });

import 'mocha';
import { before } from 'mocha';
import { 
  Models
} from '../../src';
import { expect } from 'chai';
import fetch from 'cross-fetch';
import { CreateModelMetadataRequest, DiscoverModelsRequest, Task, DiscoverModelsByPlatformRequest, GetModelByPlatformRequest, ListModelsByPlatformRequest } from '@mlhub/models-ts-sdk';

//   basePath: process.env.TEST_TENANT,
const basePath = process.env.TEST_BASE_URL;

//////////////////////////////////////////////////////////
/*                      Platforms                       */
//////////////////////////////////////////////////////////

let externalModelsApi: Models.PlatformsApi;

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
    
    externalModelsApi = new Models.PlatformsApi(configuration);
  });

  it("should fetch a list of models from huggingface", async () => {
    let request: ListModelsByPlatformRequest = {
      platform: Models.Platform.HuggingFace
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
      platform: Models.Platform.HuggingFace,
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

  it("should discover models on Patra", async () => {
    let request: DiscoverModelsByPlatformRequest = {
      platform: Models.Platform.Patra,
      discoveryCriteria: {
        confidence_threshold: null,
        prompt: "Find image detection models",
        criteria: []
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

// //////////////////////////////////////////////////////////
// /*                      Models                          */
// //////////////////////////////////////////////////////////

let modelsApi: Models.ModelsApi;

describe('Models e2e tests', async () => {
  before(async () => {
    // Tenants configuration for all tests
    const configurationParameters: Models.ConfigurationParameters = {
      basePath,
      headers: {},
      fetchApi: fetch
    }
    const configuration: Models.Configuration = new Models.Configuration(configurationParameters);
    
    modelsApi = new Models.ModelsApi(configuration);
  });

  // let request: CreateModelMetadataRequest = {
  //   modelMetadata: {}
  // };

  // it("should create a model metadata entry", async () => {
  //   try {
  //       let response = await modelsApi.createModelMetadata(request)
  //       expect(response.result).to.be.an("object");
  //   } catch (e) {
  //     expect.fail(`Test failed because an exception was thrown: ${e}`)
  //   }
  // });

  let request: DiscoverModelsRequest = {
    limit: 2,
    includeCount: true,
    discoveryCriteria: {
      criteria: [
        {
          "libraries": ["transformers"],
          "task_types": [ Task.TextGeneration ]
        }
      ]
    }
  };

  it("should discover 2 models that can perform the text-generation task", async () => {
    try {
        let response = await modelsApi.discoverModels(request)
        expect(response.result).to.be.an("array");
        expect(response.result.length).to.be.eq(2);
    } catch (e) {
      expect.fail(`Test failed because an exception was thrown: ${e}`)
    }
  });
});

// //////////////////////////////////////////////////////////
// /*                      Models                          */
// //////////////////////////////////////////////////////////

let tasksApi: Models.TasksApi;

describe('Tasks e2e tests', async () => {
  before(async () => {
    // Tenants configuration for all tests
    const configurationParameters: Models.ConfigurationParameters = {
      basePath,
      headers: {},
      fetchApi: fetch
    }
    const configuration: Models.Configuration = new Models.Configuration(configurationParameters);
    
    tasksApi = new Models.TasksApi(configuration);
  });

  it("should list all task types for MLHub models", async () => {
    try {
        let response = await tasksApi.listTasks()
        expect(response.result).to.be.an("array");
        expect(response.result).to.contain(Task.FillMask)
    } catch (e) {
      expect.fail(`Test failed because an exception was thrown: ${e}`)
    }
  });
});