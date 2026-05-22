import 'mocha';
import {
  Models,
  Deployments,
} from '../src';
import { expect } from 'chai';

describe('mlhub/models-ts-sdk', () => {
  it('should have APIs', () => {
    expect(Models).to.have.property('ArtifactsApi');
    expect(Models).to.have.property('PublicationsApi');
    expect(Models).to.have.property('IngestionsApi');
    expect(Models).to.have.property('ModelsApi');
    expect(Models).to.have.property('PlatformsApi');
    expect(Models).to.have.property('TasksApi');
  });
});

describe('mlhub/deployments-ts-sdk', () => {
  it('should have APIs', () => {
    expect(Deployments).to.have.property('DeploymentsApi');
    expect(Deployments).to.have.property('StrategiesApi');
  });
});
