// Load e2e environment variables into process.env
require('dotenv').config({ path: 'example.env' });
import { getToken, checkJsonError } from '../../tapis-typescript/e2e/utils';
const fetch = require('node-fetch');

import { 
  Apps
} from '@tapis/tapis-typescript';

(async function() {
  // Retrieve an access token
  const token = await getToken();

  // Configure the client to use the retrieved JWT as the "X-Tapis-Token" authentication header
  const configurationParameters: Apps.ConfigurationParameters = {
    basePath: process.env.TEST_TENANT,
    headers: {
      "X-Tapis-Token": token.access_token
    },
    fetchApi: fetch
  }
  const configuration: Apps.Configuration = new Apps.Configuration(configurationParameters);

  try {
    const api: Apps.ApplicationsApi = new Apps.ApplicationsApi(configuration);
    const getAppsRequest: Apps.GetAppsRequest = {};
    const response: Apps.RespApps = await api.getApps(getAppsRequest);
    const apps: Array<Apps.TapisApp> = response.result
    console.log(apps);
  } catch (error) {
    checkJsonError(error);
  }
}());

export default getToken;