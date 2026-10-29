use std::collections::HashSet;

use tokio::sync::Mutex;

use super::{MigrationGroup, MigrationGroupRunner, run_all_migration_groups};

#[test]
fn migration_catalog_preserves_historical_ids_and_order() {
    let expected = vec![
        (
            MigrationGroup::Principals,
            ids(&["CreatePrincipalIdTenantIdIndexUniqueMigration"]),
        ),
        (
            MigrationGroup::FederatedIdentities,
            ids(&[
                "CreateIssuerSubjectIndexUniqueMigration",
                "CreateIssuerSubjectPrincipalIdIndexUniqueMigration",
                "CorrectFederatedIdentityIssuerIndexesMigration",
            ]),
        ),
        (
            MigrationGroup::Models,
            ids(&[
                "CreateModelAuthorNameIndexMigration",
                "CreateTaskTypesIndexMigration",
                "CreateArtifactIdIndexMigration",
                "RenameModelMetadataKeywordsToTagsMigration",
                "CreateModelAggregateIndexesMigration",
                "CreateExternalModelAggregateIndexesMigration",
                "CreateDeploymentOptionIndexesMigration",
            ]),
        ),
        (
            MigrationGroup::Deployments,
            ids(&[
                "CreateHpcClusterIndexesMigration",
                "CreateModelDeploymentReconciliationIndexMigration",
            ]),
        ),
        (
            MigrationGroup::Endpoints,
            ids(&[
                "CreateEndpointIndexesMigration",
                "AddEndpointOwnerMigration",
            ]),
        ),
    ];

    let actual = MigrationGroup::ORDERED
        .into_iter()
        .map(|group| {
            let ids = group
                .migrations()
                .into_iter()
                .map(|migration| migration.get_id().to_owned())
                .collect::<Vec<_>>();

            (group, ids)
        })
        .collect::<Vec<_>>();

    assert_eq!(actual, expected);
}

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[test]
fn migration_ids_are_globally_unique() {
    let ids = MigrationGroup::ORDERED
        .into_iter()
        .flat_map(MigrationGroup::migrations)
        .map(|migration| migration.get_id().to_owned())
        .collect::<Vec<_>>();

    let unique_ids = ids.iter().collect::<HashSet<_>>();

    assert_eq!(unique_ids.len(), ids.len());
}

struct RecordingMigrationGroupRunner {
    calls: Mutex<Vec<MigrationGroup>>,
    fail_on: Option<MigrationGroup>,
}

#[async_trait::async_trait]
impl MigrationGroupRunner for RecordingMigrationGroupRunner {
    async fn run(&self, group: MigrationGroup) -> anyhow::Result<()> {
        self.calls.lock().await.push(group);

        if self.fail_on == Some(group) {
            anyhow::bail!("Migration group failed");
        }

        Ok(())
    }
}

#[tokio::test]
async fn migration_groups_stop_after_the_first_failure() {
    let runner = RecordingMigrationGroupRunner {
        calls: Mutex::new(Vec::new()),
        fail_on: Some(MigrationGroup::Models),
    };

    let result = run_all_migration_groups(&runner).await;

    let calls = runner.calls.lock().await.clone();

    assert!(result.is_err());
    assert_eq!(
        calls,
        vec![
            MigrationGroup::Principals,
            MigrationGroup::FederatedIdentities,
            MigrationGroup::Models,
        ]
    );
}
