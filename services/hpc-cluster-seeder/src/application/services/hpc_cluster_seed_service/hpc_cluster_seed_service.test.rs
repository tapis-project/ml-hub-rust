use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use shared::domain::entities::hpc_cluster::{DataCenter, HpcCluster, NewHpcClusterProps};

use super::*;

struct TestSource {
    loaded: Arc<Mutex<bool>>,
    props: Vec<NewHpcClusterProps>,
}

impl HpcClusterSeedSource for TestSource {
    fn load(&self) -> Result<Vec<NewHpcClusterProps>, HpcClusterSeedError> {
        let mut loaded = self
            .loaded
            .lock()
            .map_err(|error| HpcClusterSeedError::InvalidConfiguration(error.to_string()))?;

        *loaded = true;

        Ok(self.props.clone())
    }
}

struct TestRepository {
    collection_exists: bool,
    seeded: Arc<Mutex<Vec<HpcCluster>>>,
}

#[async_trait]
impl HpcClusterSeedRepository for TestRepository {
    async fn collection_exists(&self) -> Result<bool, HpcClusterSeedError> {
        Ok(self.collection_exists)
    }

    async fn seed(&self, hpc_clusters: &[HpcCluster]) -> Result<(), HpcClusterSeedError> {
        let mut seeded = self
            .seeded
            .lock()
            .map_err(|error| HpcClusterSeedError::Persistence(error.to_string()))?;

        seeded.extend_from_slice(hpc_clusters);

        Ok(())
    }
}

fn cluster_props() -> NewHpcClusterProps {
    NewHpcClusterProps {
        enabled: true,
        name: "Vista".into(),
        description: None,
        host: "vista.tacc.utexas.edu".into(),
        port: 22,
        documentation_url: None,
        data_center: DataCenter::Tacc,
        queues: Vec::new(),
    }
}

#[tokio::test]
async fn skips_without_loading_configuration_when_collection_exists()
-> Result<(), Box<dyn std::error::Error>> {
    let loaded = Arc::new(Mutex::new(false));
    let seeded = Arc::new(Mutex::new(Vec::new()));
    let service = HpcClusterSeedService::new(
        Arc::new(TestSource {
            loaded: loaded.clone(),
            props: vec![cluster_props()],
        }),
        Arc::new(TestRepository {
            collection_exists: true,
            seeded,
        }),
    );

    let outcome = service.seed().await?;

    assert_eq!(outcome, HpcClusterSeedOutcome::SkippedCollectionExists);
    assert!(!*loaded.lock().map_err(|error| error.to_string())?);

    Ok(())
}

#[tokio::test]
async fn creates_uuid_v7_identities_before_seeding() -> Result<(), Box<dyn std::error::Error>> {
    let loaded = Arc::new(Mutex::new(false));
    let seeded = Arc::new(Mutex::new(Vec::new()));
    let service = HpcClusterSeedService::new(
        Arc::new(TestSource {
            loaded,
            props: vec![cluster_props()],
        }),
        Arc::new(TestRepository {
            collection_exists: false,
            seeded: seeded.clone(),
        }),
    );

    let outcome = service.seed().await?;
    let seeded = seeded.lock().map_err(|error| error.to_string())?;

    assert_eq!(
        outcome,
        HpcClusterSeedOutcome::Seeded {
            hpc_cluster_count: 1,
            queue_count: 0,
        }
    );
    assert_eq!(seeded[0].id().as_uuid().get_version_num(), 7);

    Ok(())
}

#[tokio::test]
async fn rejects_empty_configuration() {
    let service = HpcClusterSeedService::new(
        Arc::new(TestSource {
            loaded: Arc::new(Mutex::new(false)),
            props: Vec::new(),
        }),
        Arc::new(TestRepository {
            collection_exists: false,
            seeded: Arc::new(Mutex::new(Vec::new())),
        }),
    );

    let result = service.seed().await;

    assert!(matches!(
        result,
        Err(HpcClusterSeedError::InvalidConfiguration(_))
    ));
}
