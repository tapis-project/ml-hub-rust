#[cfg(test)]
mod artifact_publication_test {
    use uuid::Uuid;
    use crate::domain::entities::artifact_publication::{
        ArtifactPublication,
        ArtifactPublicationStatus
    };

    #[test]
    fn test_initializes_correctly() {
        let artifact_id = Uuid::new_v4();
        let publication = ArtifactPublication::new(
            artifact_id,
            "platform".into(),
            "platform-artifact-id".into()
        );
        
        assert!(publication.artifact_id == artifact_id);
        assert!(publication.attempts == 0);
        assert!(publication.platform == String::from("platform"));
        assert!(publication.platform_artifact_id == String::from("platform-artifact-id"));
        assert!(publication.created_at.into_inner() == publication.last_modified.into_inner());
        assert!(publication.status == ArtifactPublicationStatus::Submitted)
    }

    #[test]
    fn test_set_status_and_touch() {
        let mut publication = ArtifactPublication::new(
            Uuid::new_v4(),
            "platform".into(),
            "platform-artifact-id".into()
        );

        let last_modified_before = publication.last_modified.clone();

        let result = publication.set_status(&ArtifactPublicationStatus::Pending)
            .map(|p| {
                // The status should be updated
                assert!(p.status == ArtifactPublicationStatus::Pending);
                // Setting the status should update last_modified via the private
                // touch function.
                assert!(p.last_modified.into_inner() > last_modified_before.into_inner());
            });
        
        // Valid state transition must not produce error
        assert!(result.is_err() != true)
    }

    #[test]
    fn test_valid_transitions() {
        let mut publication = ArtifactPublication::new(
            Uuid::new_v4(),
            "platform".into(),
            "platform-artifact-id".into()
        );

        let maybe_publication = publication.set_status(&ArtifactPublicationStatus::Pending)
            .and_then(|p| {
                let publication = p.set_status(&ArtifactPublicationStatus::Extracting);
                assert!(!publication.is_err());
                publication
            })
            .and_then(|p| {
                let publication = p.set_status(&ArtifactPublicationStatus::Extracted);
                assert!(!publication.is_err());
                publication
            })
            .and_then(|p| {
                let publication = p.set_status(&ArtifactPublicationStatus::PublishingMetadata);
                assert!(!publication.is_err());
                publication
            })
            .and_then(|p| {
                let publication = p.set_status(&ArtifactPublicationStatus::PublishedMetadata);
                assert!(!publication.is_err());
                publication
            })
            .and_then(|p| {
                let publication = p.set_status(&ArtifactPublicationStatus::PublishingArtifact);
                assert!(!publication.is_err());
                publication
            })
            .and_then(|p| {
                let publication = p.set_status(&ArtifactPublicationStatus::PublishedArtifact);
                assert!(!publication.is_err());
                publication
            })
            .and_then(|p| {
                let publication = p.set_status(&ArtifactPublicationStatus::Finished);
                assert!(!publication.is_err());
                publication
            });
        
        assert!(!maybe_publication.is_err());
    }

    #[test]
    fn test_invalid_transitions() {
        let mut publication = ArtifactPublication::new(
            Uuid::new_v4(),
            "platform".into(),
            "platform-artifact-id".into()
        );

        let maybe_publication = publication.set_status(&ArtifactPublicationStatus::Finished);
        assert!(maybe_publication.is_err())
    }
}