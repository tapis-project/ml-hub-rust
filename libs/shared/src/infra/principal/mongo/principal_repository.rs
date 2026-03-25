use crate::application::inputs::principal::FindByFederatedIdentity;
use crate::infra::identity::mongo::documents::FederatedIdentity;
use crate::infra::principal::mongo::documents::Principal;
use crate::application::ports;
use crate::domain::entities;
use mongodb::{
    bson::doc,
    Client,
    Collection,
};
use futures::stream::TryStreamExt;

pub const FEDERATED_IDENTITY_COLLECTION: &str = "FEDERATED_IDENTITIES";
pub const PRINCIPAL_COLLECTION: &str = "FEDERATED_IDENTITIES";

pub struct PrincipalRepository {
    federated_identity_read_collection: Collection<FederatedIdentity>,
    federated_identity_write_collection: Collection<FederatedIdentity>,
    principal_read_collection: Collection<Principal>,
    principal_write_collection: Collection<Principal>,
}

impl PrincipalRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        let db = client.database(&db_name);

        Self {
            federated_identity_write_collection: db.collection(FEDERATED_IDENTITY_COLLECTION),
            federated_identity_read_collection: db.collection(FEDERATED_IDENTITY_COLLECTION),
            principal_write_collection: db.collection(PRINCIPAL_COLLECTION),
            principal_read_collection: db.collection(PRINCIPAL_COLLECTION),
        }
    }
}

#[async_trait::async_trait]
impl ports::principal::PrincipalRepository for PrincipalRepository {
    async fn save(&self, principal: &entities::principal::Principal) -> Result<(), ports::principal::PrincipalRepositoryError> {
        // let mut identity_docs = Vec::with_capacity(principal.identites().len());
        // for identity in principal.identites() {
        //     identity_docs.push(FederatedIdentity::from(identity.clone()))
        // }

        // let mut principal_doc = Principal::from(principal.clone());

        // let result = self.write_collection.insert_one(&document)
        //     .await
        //     .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        // document._id = result.inserted_id.as_object_id();

        Ok(())
    }

    async fn find_by_identity(&self, input: &FindByFederatedIdentity) -> Result<Option<entities::principal::Principal>, ports::principal::PrincipalRepositoryError> {
        let mut filter = doc! {};

        // if let Some(iss) = input.issuer.clone() {
        //     filter.insert("issuer", iss);
        // }

        // if let Some(sub) = input.subject.clone() {
        //     filter.insert("sub", sub);
        // }

        // if let Some(tenant_id) = input.tenant_id.clone() {
        //     filter.insert("tenant_id", tenant_id);
        // }
        
        // let mut cursor = self.read_collection.find(filter)
        //     .await
        //     .map_err(|err| ApplicationError::RepoError(err.to_string()))?;

        // let maybe_federated_identity = cursor.try_next().await.map_err(|err| ApplicationError::RepoError(err.to_string()))?;
        
        // match maybe_federated_identity {
        //     Some(m) => Ok(Some(entities::identity::FederatedIdentity::from(m))),
        //     None => Ok(None)
        // }
        Ok(None)
    }
}