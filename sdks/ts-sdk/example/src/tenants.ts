// Load e2e environment variables into process.env
require('dotenv').config({ path: 'example.env' });
import { checkJsonError } from '../../tapis-typescript/e2e/utils';
const fetch = require('node-fetch');

import { Tenants } from '@tapis/tapis-typescript';

// Asynchronous top level function
(async function() {
  try {
    // Create a configuration for the service with the target tenant and the fetch client.
    // In this case, node-fetch.
    const configurationParameters: Tenants.ConfigurationParameters = {
      basePath: process.env.TACC_TENANT,
      fetchApi: fetch
    };
    const configuration: Tenants.Configuration = new Tenants.Configuration(configurationParameters);

    // Create the service with the configuration
    const tenantsApi: Tenants.TenantsApi = new Tenants.TenantsApi(configuration);

    // Create a request structure
    const listTenantsRequest: Tenants.ListTenantsRequest = {};

    // Make the request
    const response: Tenants.RespListTenants = await tenantsApi.listTenants(listTenantsRequest);

    // Extract the result object
    const tenants: Array<Tenants.Tenant> = response.result;
    console.log(tenants);
  } catch (error) {
    checkJsonError(error);
  }

}());
