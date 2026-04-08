use std::time::Duration;

use crate::application::inputs::principal::FindByFederatedIdentity;
use crate::infra::identity::mongo::documents::{FederatedIdentity, FEDERATED_IDENTITY_COLLECTION};
use crate::infra::common::mongo::is_duplicate_key_error;
use crate::infra::principal::mongo::documents::{Principal, PRINCIPAL_COLLECTION};
use crate::application::ports;
use crate::application::ports::principal::PrincipalRepositoryError;
use crate::domain::entities;
use mongodb::{
    bson::{doc, to_bson, to_document},
    error::{TRANSIENT_TRANSACTION_ERROR, Error},
    options::{UpdateModifications, ReadConcern, UpdateOneModel, WriteConcern, WriteModel},
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
    pub fn new(client: Client, db_name: String) -> Self { 
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
            .max_commit_time(Duration::from_millis(2000))
            .read_concern(ReadConcern::majority())
            .write_concern(WriteConcern::majority())
            .await
            .map_err(|err| PrincipalRepositoryError::PersistenceError {
                retriable: true,
                message: err.to_string(),
            })?;

        // Convert the Principal into MongoDB documents
        let principal_doc = Principal::from(principal.clone());
        
        // Save the Principal
        let _ = match self.principal_write_collection.insert_one(principal_doc).session(&mut session).await {
            Ok(x) => x,
            Err(err) => return Err(PrincipalRepositoryError::from(err))
        };
        
        // Convert the Principal's federated identity into a MongoDB document
        let identity_doc = FederatedIdentity::from((principal.active_identity().clone(), principal.id.clone()));
        
        let filter = doc! {
            "issuer": &identity_doc.issuer,
            "subject": &identity_doc.subject,
            "principal_id": &identity_doc.principal_id.clone(),
        };

        // Create an update or insert model for the identity
        let mut insert_doc = to_document(&identity_doc)
            .map_err(|err| PrincipalRepositoryError::ProgrammingError(err.to_string()))?;
        
        insert_doc.remove("metadata");
        insert_doc.remove("last_seen");

        let update = UpdateModifications::Document(doc! {
            "$set": {
                "last_seen": &identity_doc.last_seen,
                "metadata": to_bson(&identity_doc.metadata)
                    .map_err(|err| PrincipalRepositoryError::ProgrammingError(err.to_string()))?,
            },
            "$setOnInsert": insert_doc,
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

        // Run the bulk write with the session
        let _ = match self.client.bulk_write(vec![identity_write_model]).session(&mut session).await {
            Ok(x) => x,
            Err(err) => return Err(PrincipalRepositoryError::from(err))
        };
        
        match session.commit_transaction().await {
            Ok(_) => Ok(()),
            Err(err) => {
                if let Err(e) = session.abort_transaction().await {
                    return Err(PrincipalRepositoryError::from(e))
                }

                return Err(PrincipalRepositoryError::from(err))
            }
        }
    }

    async fn find_by_identity(&self, input: &FindByFederatedIdentity) -> Result<Option<entities::principal::Principal>, PrincipalRepositoryError> {
        let ident_filter = doc! {
            "issuer": input.identity.issuer.clone(),
            "subject": input.identity.subject.clone(),
            "tenant_id": input.identity.tenant_id.clone(),
        };
        
        let mut identity_cursor = match self.federated_identity_read_collection.find(ident_filter).await{
            Ok(c) => Ok(c),
            Err(err) => Err(PrincipalRepositoryError::PersistenceError {
                retriable: false,
                message: err.to_string(),
            })
        }?;

        let maybe_federated_identity_doc = match identity_cursor.try_next().await {
            Ok(f) => Ok(f),
            Err(err) => Err(PrincipalRepositoryError::PersistenceError {
                retriable: false,
                message: err.to_string(),
            })
        }?;

        let federated_identity_doc = match maybe_federated_identity_doc {
            Some(f) => f,
            None => return Ok(None)
        };

        let principal_filter = doc! {
            "id": &federated_identity_doc.principal_id
        };
        
        let mut principal_cursor = match self.principal_read_collection.find(principal_filter).await{
            Ok(c) => Ok(c),
            Err(err) => Err(PrincipalRepositoryError::PersistenceError {
                retriable: false,
                message: err.to_string(),
            })
        }?;

        let maybe_principal_doc = match principal_cursor.try_next().await {
            Ok(f) => Ok(f),
            Err(err) => Err(PrincipalRepositoryError::PersistenceError {
                retriable: false,
                message: err.to_string(),
            })
        }?;

        Ok(match maybe_principal_doc {
            Some(p) => Some(entities::principal::Principal::try_from((p, federated_identity_doc))?),
            None => None
        })
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