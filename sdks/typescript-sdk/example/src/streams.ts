
// Load e2e environment variables into process.env
require('dotenv').config({ path: 'example.env' });
const fetch = require('node-fetch');
import { getToken, checkJsonError } from '../../tapis-typescript/e2e/utils';
import { Authenticator, Streams } from '@tapis/tapis-typescript';

const projectId = process.env.TEST_PROJECT_ID;
const siteId = process.env.TEST_SITE_ID;
const instId = process.env.TEST_INSTRUMENT_ID;

(async function() {
    try {
        const token: Authenticator.NewAccessTokenResponse = await getToken();
        // Configure the client to use the retrieved JWT as the "X-Tapis-Token" authentication header
        const configurationParameters: Streams.ConfigurationParameters = {
            basePath: process.env.TEST_TENANT,
            headers: {
                "X-Tapis-Token": token.access_token
            },
            fetchApi: fetch
        }
        
        const configuration: Streams.Configuration = new Streams.Configuration(configurationParameters);
        //APIs
        const healthApi: Streams.HealthApi = new Streams.HealthApi(configuration);
        const projectApi: Streams.ProjectsApi = new Streams.ProjectsApi(configuration);
        const siteApi: Streams.SitesApi = new Streams.SitesApi(configuration);
        const instrumentApi: Streams.InstrumentsApi = new Streams.InstrumentsApi(configuration);
        const variableApi: Streams.VariablesApi = new Streams.VariablesApi(configuration);
        const measurementApi: Streams.MeasurementsApi = new Streams.MeasurementsApi(configuration);
        const ontologyApi: Streams.OntologiesApi = new Streams.OntologiesApi(configuration);
        const channelApi: Streams.ChannelsApi = new Streams.ChannelsApi(configuration);
        const templateApi: Streams.TemplatesApi = new Streams.TemplatesApi(configuration);
        const unitApi: Streams.UnitsApi = new Streams.UnitsApi(configuration);
        const measuredPropertyApi: Streams.MeasuredPropertiesApi = new Streams.MeasuredPropertiesApi(configuration);


        //projects
        console.log("List Projects\n");

        const listProjectParams: Streams.ListProjectsRequest = {};

        const projectResponse: Streams.RespListProjects = await projectApi.listProjects(listProjectParams);
        const projects: Array<Streams.Project> = projectResponse.result;
        //console.log(projects);

        //sites
        console.log("\n\nList Sites\n");

        const listSiteParams: Streams.ListSitesRequest = {
            projectId: projectId,
        };

        const siteResponse: Streams.RespListSites = await siteApi.listSites(listSiteParams);
        const sites: Array<Streams.Site> = siteResponse.result;
        console.log(sites);

        //instruments
        console.log("\n\nList Instruments\n");

        const listInstrumentParams: Streams.ListInstrumentsRequest = {
            projectId: projectId,
            siteId: siteId,
        };

        const instrumentResponse: Streams.RespListInstruments = await instrumentApi.listInstruments(listInstrumentParams);
        const instruments: Array<Streams.Instrument> = instrumentResponse.result;
        console.log(instruments);

        //variables
        console.log("\n\nList Variables\n");

        const listVariableParams: Streams.ListVariablesRequest = {
            projectId: projectId,
            siteId: siteId,
            instId: instId
        };

        const variableResponse: Streams.RespListVariables = await variableApi.listVariables(listVariableParams);
        const variables: Array<Streams.Variable> = variableResponse.result;
        console.log(variables);

        //measurements
        //list
        console.log("\n\nList Measurements\n");

        let listMeasurementParams: Streams.ListMeasurementsRequest = {
            projectId: projectId,
            siteId: siteId,
            instId: instId,
            startDate: "2021-01-01T00:00:00Z",
            endDate: "2025-12-30T22:19:25Z",
            format: "json"
        };

        const measurementResponse: Streams.RespListMeasurements = await measurementApi.listMeasurements(listMeasurementParams);
        const measurements: Streams.Measurements = measurementResponse.result;
        console.log(measurements);

        //download
        console.log("\n\nDownload Measurements by Instrument Only\n");

        let downloadMeasurementParams: Streams.DownloadMeasurementsRequest = {
            instId: instId,
        };

        const measurementDownloadResponse: Streams.RespDownloadMeasurements = await measurementApi.downloadMeasurements(downloadMeasurementParams);
        const measurementsDownload: Streams.Measurements = measurementDownloadResponse.result;
        console.log(measurementsDownload);
    }
    catch(error) {
        checkJsonError(error);
    }
  }());




