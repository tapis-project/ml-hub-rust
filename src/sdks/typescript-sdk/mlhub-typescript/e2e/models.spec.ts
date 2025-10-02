// Load e2e environment variables into process.env
require('dotenv').config({ path: 'e2e.env' });

import 'mocha';
import { before } from 'mocha';
import { 
  Models
} from '../src';
import { getToken, checkJsonError } from './utils';
import { expect } from 'chai';
import fetch from 'cross-fetch';

let modelsApi: Models.ModelsApi;

describe('Models e2e tests', async () => {

  before(async () => {
    const token = await getToken();
    // Tenants configuration for all tests
    const configurationParameters: Models.ConfigurationParameters = {
    //   basePath: process.env.TEST_TENANT,
      basePath: 'https://hub.pods.tacc.tapis.io',
      headers: {
        "X-TAPIS-TOKEN": token.access_token!
      },
      fetchApi: fetch
    }
    const configuration: Models.Configuration = new Models.Configuration(configurationParameters);
    
    modelsApi = new Models.ModelsApi(configuration);
  });

  it("should fetch a list of models from huggingface", async () => {
    
  });
});