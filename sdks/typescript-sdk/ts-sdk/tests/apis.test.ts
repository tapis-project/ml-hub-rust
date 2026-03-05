import 'mocha';
import {
  Models,
} from '../src';
import { expect } from 'chai';

describe('tapis-typescript', () => {
  it('should have APIs', () => {
    expect(Models).to.have.property('ArtifactsApi');
    expect(Models).to.have.property('PublicationsApi');
    expect(Models).to.have.property('IngestionsApi');
    expect(Models).to.have.property('ModelsApi');
    expect(Models).to.have.property('PlatformsApi');
    expect(Models).to.have.property('TasksApi');
  });
});
