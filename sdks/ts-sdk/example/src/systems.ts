// Load e2e environment variables into process.env
require('dotenv').config({ path: 'example.env' });
import { getToken, checkJsonError } from '../../tapis-typescript/e2e/utils';
const fetch = require('node-fetch');

import { 
  Systems
} from '@tapis/tapis-typescript';

(async function() {
  try {
    // Retrieve an access token
    const token = await getToken();

    // Configure the client to use the retrieved JWT as the "X-Tapis-Token" authentication header
    const configurationParameters: Systems.ConfigurationParameters = {
      basePath: process.env.TEST_TENANT,
      headers: {
        "X-Tapis-Token": token.access_token
      },
      fetchApi: fetch
    }
    const configuration: Systems.Configuration = new Systems.Configuration(configurationParameters);
    const api = new Systems.SystemsApi(configuration);

    // Create a request object
    const systemsRequest: Systems.GetSystemsRequest = {};

    // Send the request
    const systemsResponse: Systems.RespSystems = await api.getSystems(systemsRequest);

    // Print the result
    const systems: Array<Systems.TapisSystem> = systemsResponse.result;
    console.log(systems);
  } catch (error) {
    checkJsonError(error);
  }

}());

export default getToken;