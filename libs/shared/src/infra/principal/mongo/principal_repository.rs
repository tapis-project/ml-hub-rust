use crate::application::inputs::principal::FindByFederatedIdentity;
use crate::infra::identity::mongo::documents::{FederatedIdentity, FEDERATED_IDENTITY_COLLECTION};
use crate::infra::common::mongo::is_duplicate_key_error;
use crate::infra::principal::mongo::documents::{Principal, PRINCIPAL_COLLECTION};
use crate::application::ports;
use crate::application::ports::principal::PrincipalRepositoryError;
use crate::domain::entities;
use mongodb::{
    bson::{doc, to_bson},
    error::{TRANSIENT_TRANSACTION_ERROR, Error},
    options::{UpdateModifications, InsertOneModel, ReadConcern, UpdateOneModel, WriteConcern, WriteModel},
    Client,
    Collection
};
use futures::stream::TryStreamExt;

type FederatedIdentityReadCollection = Collection<FederatedIdentity>;
type FederatedIdentityWriteCollection = Collection<FederatedIdentity>;
type PrincipalReadCollection = Collection<Principal>;
type PrincipalWriteCollection = Collection<Principal>;

pub struct PrincipalRepository {
    client: Client,
    federated_identity_read_collection: FederatedIdentityReadCollection,
    federated_identity_write_collection: FederatedIdentityWriteCollection,
    principal_read_collection: PrincipalReadCollection,
    principal_write_collection: PrincipalWriteCollection,
}

impl PrincipalRepository {
    pub fn new(client: &Client, db_name: String) -> Self { 
        let db = client.database(&db_name);

        Self {
            client: client.clone(),
            federated_identity_write_collection: db.collection(FEDERATED_IDENTITY_COLLECTION),
            federated_identity_read_collection: db.collection(FEDERATED_IDENTITY_COLLECTION),
            principal_write_collection: db.collection(PRINCIPAL_COLLECTION),
            principal_read_collection: db.collection(PRINCIPAL_COLLECTION),
        }
    }
}

#[async_trait::async_trait]
impl ports::principal::PrincipalRepository for PrincipalRepository {
    async fn save(&self, principal: &entities::principal::Principal) -> Result<(), PrincipalRepositoryError> {
        // Start a session
        let mut session = match self.client.start_session().await {
            Ok(s) => Ok(s),
            Err(err) => Err(PrincipalRepositoryError::PersistenceError {
                retriable: true,
                message: err.to_string(),
            })
        }?;

        // Start a transaction
        session.start_transaction()
            .read_concern(ReadConcern::majority())
            .write_concern(WriteConcern::majority())
            .await
            .map_err(|err| PrincipalRepositoryError::PersistenceError {
                retriable: true,
                message: err.to_string(),
            })?;

        
        // Convert the Principal's identities into MongoDB documents and create
        // insert models for the bulk write operation
        let mut identity_write_models = Vec::with_capacity(principal.identites().len());
        for identity in principal.identites() {
            let identity_doc = FederatedIdentity::from((identity.clone(), principal.id.clone()));
            
            let filter = doc! {
                "issuer": &identity_doc.issuer,
                "subject": &identity_doc.subject,
                "principal_id": &identity_doc.principal_id,
            };

            let update = UpdateModifications::Document(doc! {
                // Updates the document with the fields below if found...
                "$set": {
                    "last_seen": &identity_doc.last_seen,
                    "metadata": to_bson(&identity_doc.metadata)
                        .map_err(|err| PrincipalRepositoryError::ProgrammingError(err.to_string()))?, // Should never happen
                },
                // Creates a new FederatedIdentity if not
                "$setOnInsert": to_bson(&identity_doc)
                    .map_err(|err| PrincipalRepositoryError::ProgrammingError(err.to_string()))?,
            });

            // Build the write model
            let identity_write_model = WriteModel::UpdateOne(
                UpdateOneModel::builder()
                    .namespace(self.federated_identity_write_collection.namespace())
                    .filter(filter)
                    .update(update)
                    .array_filters(None)
                    .collation(None)
                    .hint(None)
                    .sort(None)
                    .upsert(true)
                    .build()
            );

            identity_write_models.push(identity_write_model);
        }

        // Run the bulk write with the session
        let _ = match self.client.bulk_write(identity_write_models).session(&mut session).await {
            Ok(x) => x,
            Err(err) => return Err(PrincipalRepositoryError::from(err))
        };

        // Convert the Principal into MongoDB documents
        let principal_doc = Principal::from(principal.clone());

        let _ = match self.principal_write_collection.insert_one(principal_doc).session(&mut session).await {
            Ok(x) => x,
            Err(err) => return Err(PrincipalRepositoryError::from(err))
        };
        
        session.commit_transaction()
            .await
            .map_err(|err| PrincipalRepositoryError::from(err))
    }

    async fn find_by_identity(&self, input: &FindByFederatedIdentity) -> Result<Option<entities::principal::Principal>, PrincipalRepositoryError> {
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

impl From<Error> for PrincipalRepositoryError {
    fn from(value: Error) -> Self {
        if is_duplicate_key_error(&value) {
            return PrincipalRepositoryError::PrincipalAlreadyExists
        }

        if value.contains_label(TRANSIENT_TRANSACTION_ERROR) {
            return PrincipalRepositoryError::PersistenceError { retriable: true, message: value.to_string() }
        }

        PrincipalRepositoryError::PersistenceError { retriable: false, message: value.to_string() }
    }
}