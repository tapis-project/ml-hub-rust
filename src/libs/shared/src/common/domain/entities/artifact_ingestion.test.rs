#[cfg(test)]
mod artifact_ingestion_test {
    use uuid::Uuid;
    use crate::common::domain::entities::artifact_ingestion::{ArtifactIngestion, ArtifactIngestionStatus};

    #[test]
    fn test_touch() {
        let test_id = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")
            .expect("Failed to parse UUID");

        let mut ingestion = ArtifactIngestion::new(test_id, "test_path".into(), None);
        let initial_last_modified = ingestion.last_modified.clone();
        // Call touch to update last_modified
        ingestion.touch();
        // Check that last_modified has been updated
        assert_ne!(ingestion.last_modified, initial_last_modified);
    }

    #[test]
    fn positive_test_status_transition() {
        let test_id = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")
            .expect("Failed to parse UUID");

        // Create a new ArtifactIngestion instance with initial status which is Submitted
        let mut ingestion = ArtifactIngestion::new(test_id, "test_path".into(), None);
        assert!(matches!(ingestion.status, ArtifactIngestionStatus::Submitted));

        // Try to change status to Pending
        // This should succeed
        let result = ingestion.change_status(ArtifactIngestionStatus::Pending);
        assert!(result.is_ok());
        assert!(matches!(ingestion.status, ArtifactIngestionStatus::Pending));
    }

    #[test]
    fn negative_test_status_transition() {
        let test_id = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")
            .expect("Failed to parse UUID");

        // Create a new ArtifactIngestion instance with initial status which is Submitted
        let mut ingestion = ArtifactIngestion::new(test_id, "test_path".into(), None);
        assert!(matches!(ingestion.status, ArtifactIngestionStatus::Submitted));

        // Try to change status to Downloaded directly
        // This should fail because we cannot transition from Submitted to Downloaded directly
        // The status stays as Submitted
        let result = ingestion.change_status(ArtifactIngestionStatus::Downloaded);
        assert!(result.is_err());
        assert!(matches!(ingestion.status, ArtifactIngestionStatus::Submitted));
    }

    #[test]
    fn positive_test_set_artifact_path() {
        let test_id = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")
            .expect("Failed to parse UUID");

        let mut ingestion = ArtifactIngestion::new(test_id, "test_path".into(), None);

        // Set the status to a valid state before setting the artifact path
        ingestion.change_status(ArtifactIngestionStatus::Pending)
            .and_then(|_| ingestion.change_status(ArtifactIngestionStatus::Downloading))
            .and_then(|_| ingestion.change_status(ArtifactIngestionStatus::Downloaded))
            .expect("Failed during status transitions");
        let result = ingestion.set_artifact_path("new_artifact_path".into());
        // Check that the artifact_path has been set correctly
        assert!(result.is_ok());
        assert!(ingestion.artifact_path.is_some());
    }

    #[test]
    fn negative_test_set_artifact_path() {
        let test_id = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")
            .expect("Failed to parse UUID");

        let mut ingestion = ArtifactIngestion::new(test_id, "test_path".into(), None);

        // Try to set the artifact path without a valid status
        let result = ingestion.set_artifact_path("new_artifact_path".into());
        // Check that the artifact_path has not been set
        assert!(result.is_err());
        assert!(ingestion.artifact_path.is_none());
    }
}