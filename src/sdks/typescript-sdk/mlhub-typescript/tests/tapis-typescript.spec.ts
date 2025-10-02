import 'mocha';
import {
  Authenticator,
  Systems,
  Tenants,
  Files,
  Apps,
  Jobs,
  Streams,
  Workflows,
  Actors,
  Pods,
  Models,
  Notifications,
  Datasets
} from '../src';
import { expect } from 'chai';

describe('tapis-typescript', () => {
  it('should have Authenticator service', () => {
    expect(Authenticator).to.have.property('ClientsApi');
    expect(Authenticator).to.have.property('ProfilesApi');
    expect(Authenticator).to.have.property('TokensApi');
    expect(Authenticator).to.have.property('MetadataApi');
    expect(Authenticator).to.have.property('HealthCheckApi');
    expect(Authenticator).to.have.property('AdminApi');
  });
  it('should have Systems service', () => {
    expect(Systems).to.have.property('CredentialsApi');
    expect(Systems).to.have.property('GeneralApi');
    expect(Systems).to.have.property('PermissionsApi');
    expect(Systems).to.have.property('SystemsApi');
  });
  it('should have Tenants service', () => {
    expect(Tenants).to.have.property('SitesApi');
    expect(Tenants).to.have.property('TenantsApi');
  });
  it('should have Files service', () => {
    expect(Files).to.have.property('ContentApi');
    expect(Files).to.have.property('FileOperationsApi');
    expect(Files).to.have.property('GeneralApi');
    expect(Files).to.have.property('PermissionsApi');
    expect(Files).to.have.property('SharingApi');
    expect(Files).to.have.property('TransfersApi');
    expect(Files).to.have.property('PostItsApi');
  });
  it('should have Apps service', () => {
    expect(Apps).to.have.property('ApplicationsApi');
    expect(Apps).to.have.property('HealthApi');
    expect(Apps).to.have.property('PermissionsApi');
  });
  it('should have Jobs service', () => {
    expect(Jobs).to.have.property('JobsApi');
    expect(Jobs).to.have.property('HealthApi');
    expect(Jobs).to.have.property('SharingApi');
    expect(Jobs).to.have.property('SubscriptionsApi');
  });
  it('should have Streams service', () => {
    expect(Streams).to.have.property('HealthApi');
    expect(Streams).to.have.property('ProjectsApi');
    expect(Streams).to.have.property('SitesApi');
    expect(Streams).to.have.property('InstrumentsApi');
    expect(Streams).to.have.property('VariablesApi');
    expect(Streams).to.have.property('MeasurementsApi');
    expect(Streams).to.have.property('OntologiesApi');
    expect(Streams).to.have.property('ChannelsApi');
    expect(Streams).to.have.property('TemplatesApi');
    expect(Streams).to.have.property('UnitsApi');
    expect(Streams).to.have.property('MeasuredPropertiesApi');
  });
  it('should have Workflows service', () => {
    expect(Workflows).to.have.property('ArchivesApi');
    expect(Workflows).to.have.property('CICDApi');
    expect(Workflows).to.have.property('ETLApi');
    expect(Workflows).to.have.property('GeneralApi');
    expect(Workflows).to.have.property('GroupsApi');
    expect(Workflows).to.have.property('IdentitiesApi');
    expect(Workflows).to.have.property('PipelineArchivesApi');
    expect(Workflows).to.have.property('PipelineRunsApi');
    expect(Workflows).to.have.property('PipelinesApi');
    expect(Workflows).to.have.property('TaskExecutionsApi');
    expect(Workflows).to.have.property('TasksApi');
    expect(Workflows).to.have.property('UsersApi');
    expect(Workflows).to.have.property('SecretsApi');
    expect(Workflows).to.have.property('GroupSecretsApi');
  });
  it('should have Actors service', () => {
    expect(Actors).to.have.property('ActorsApi');
    expect(Actors).to.have.property('MessagesApi');
    expect(Actors).to.have.property('ExecutionsApi');
    expect(Actors).to.have.property('NoncesApi');
    expect(Actors).to.have.property('StateApi');
    expect(Actors).to.have.property('PermissionsApi');
    expect(Actors).to.have.property('WorkersApi');
    expect(Actors).to.have.property('AliasesApi');
    expect(Actors).to.have.property('SearchApi');
  });
  it('should have Pods service', () => {
    expect(Pods).to.have.property('MiscApi');
    expect(Pods).to.have.property('PermissionsApi');
    expect(Pods).to.have.property('PodsApi');
    expect(Pods).to.have.property('SnapshotsApi');
    expect(Pods).to.have.property('VolumesApi');
  });
  it('should have Models service', () => {
    expect(Models).to.have.property('ModelsApi');
  });
  it('should have Notifications service', () => {
    expect(Notifications).to.have.property('EventsApi');
    expect(Notifications).to.have.property('GeneralApi');
    expect(Notifications).to.have.property('SubscriptionsApi');
    expect(Notifications).to.have.property('TestApi');
  });
  it('should have Datasets service', () => {
    expect(Datasets).to.have.property('DatasetsApi');
  });
});
