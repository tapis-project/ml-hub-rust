use crate::application::{errors::ApplicationError, inputs::identity::FilterInput};
use crate::infra::identity::mongo::documents::FederatedIdentity;
use crate::application;
use crate::domain::entities;
use mongodb::{
    bson::doc,
    Database,
    Collection,
};
use futures::stream::TryStreamExt;

pub const FEDERATED_IDENTITY_COLLECTION: &str = "FEDERATED_IDENTITIES";

pub struct FederatedidentityRepository {
    read_collection: Collection<FederatedIdentity>,
    write_collection: Collection<FederatedIdentity>
}

impl FederatedidentityRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            write_collection: db.collection(FEDERATED_IDENTITY_COLLECTION),
            read_collection: db.collection(FEDERATED_IDENTITY_COLLECTION)
        }
    }
}

#[async_trait::async_trait]
impl application::ports::identity::FederatedIdentityRepository for FederatedidentityRepository {
    async fn save(&self, identity: &entities::identity::FederatedIdentity) -> Result<(), ApplicationError> {
        let mut document = FederatedIdentity::from(identity.clone());

        let result = self.write_collection.insert_one(&document)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn find(&self, input: &FilterInput) -> Result<Option<entities::identity::FederatedIdentity>, ApplicationError> {
        let mut filter = doc! {};

        if let Some(iss) = input.issuer.clone() {
            filter.insert("issuer", iss);
        }

        if let Some(sub) = input.subject.clone() {
            filter.insert("sub", sub);
        }

        if let Some(tenant_id) = input.tenant_id.clone() {
            filter.insert("tenant_id", tenant_id);
        }
        
        let mut cursor = self.read_collection.find(filter)
            .await
            .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        let maybe_federated_identity = cursor.try_next().await.map_err(|err| ApplicationError::RepoError(err.to_string()))?;
        
        match maybe_federated_identity {
            Some(m) => Ok(Some(entities::identity::FederatedIdentity::from(m))),
            None => Ok(None)
        }
    }
}