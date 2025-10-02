// Load e2e environment variables into process.env
require('dotenv').config({ path: 'e2e.env' });
import fetch from 'cross-fetch';
import { 
  Authenticator
} from '../src';

export const checkJsonError = async (error) => {
  if (error.json) {
    // Catch any errors thrown by API calls
    // Asynchronously wait for the error to be translated
    const errorBody = await error.json();
    console.error(errorBody);
    // Rethrow the error
    throw errorBody;
  } else {
    throw error;
  }
}

export const getToken = async (): Promise<Authenticator.NewAccessTokenResponse> => {
  const configurationParameters: Authenticator.ConfigurationParameters = {
    basePath: process.env.TEST_TENANT,
    fetchApi: fetch
  }
  const configuration: Authenticator.Configuration = new Authenticator.Configuration(configurationParameters);
  const api: Authenticator.TokensApi = new Authenticator.TokensApi(configuration);
  const createTokenRequest: Authenticator.CreateTokenRequest = {
    reqCreateToken: {
      username: process.env.TEST_USER,
      password: process.env.TEST_PASSWORD,
      grant_type: "password"
    }
  }
  const response: Authenticator.RespCreateToken = await api.createToken(createTokenRequest);
  return response.result.access_token;
}