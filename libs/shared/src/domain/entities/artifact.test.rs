#[cfg(test)]
mod artifact_test {
    use crate::domain::entities::artifact::{Artifact, ArtifactType};

    #[test]
    fn test_touch() {
        let mut artifact = Artifact::new(ArtifactType::Model);
        let initial_last_modified = artifact.last_modified.clone();
        // Call touch to update last_modified
        artifact.touch();
        // Check that last_modified has been updated
        assert_ne!(artifact.last_modified, initial_last_modified);
    }

    #[test]
    fn test_set_path() {
        let mut artifact = Artifact::new(ArtifactType::Model);
        let initial_last_modified = artifact.last_modified.clone();

        // check that the path is initially None
        assert!(artifact.path.is_none());
        // Set the path of the artifact
        artifact.set_path("/path/to/artifact".into());
        // Check that the path has been set
        assert!(artifact.path.is_some());
        // Check that last_modified has been updated
        assert_ne!(artifact.last_modified, initial_last_modified);
    }
}