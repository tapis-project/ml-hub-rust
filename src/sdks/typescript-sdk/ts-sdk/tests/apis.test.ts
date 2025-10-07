import 'mocha';
import {
  Models,
} from '../src';
import { expect } from 'chai';

describe('tapis-typescript', () => {
  it('should have Models service', () => {
    expect(Models).to.have.property('ArtifactsApi');
    expect(Models).to.have.property('IngestionsApi');
    expect(Models).to.have.property('MetadataApi');
    expect(Models).to.have.property('PlatformsApi');
    expect(Models).to.have.property('PublicationsApi');
  });
});
