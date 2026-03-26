use crate::application::inputs::principal::FindByFederatedIdentity;
use crate::infra::identity::mongo::documents::FederatedIdentity;
use crate::infra::principal::mongo::documents::Principal;
use crate::application::ports;
use crate::domain::entities;
use mongodb::{
    bson::doc, error::{Error, TRANSIENT_TRANSACTION_ERROR, UNKNOWN_TRANSACTION_COMMIT_RESULT}, options::{ReadConcern, WriteConcern}, Client, ClientSession, Collection
};
use futures::stream::TryStreamExt;

pub const FEDERATED_IDENTITY_COLLECTION: &str = "FEDERATED_IDENTITIES";
pub const PRINCIPAL_COLLECTION: &str = "FEDERATED_IDENTITIES";

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

    async fn save_principal_transaction(
        principal: &Principal,
        identities: &Vec<FederatedIdentity>,
        principal_write_coll: &PrincipalWriteCollection,
        fed_ident_write_coll: &FederatedIdentityWriteCollection,
        session: &mut ClientSession,
    ) -> Result<(), Error> {
        fed_ident_write_coll.insert_many(identities).session(&mut *session).await?;
        principal_write_coll.insert_one(principal).session(&mut *session).await?;
        
        session.commit_transaction().await
    }
}

#[async_trait::async_trait]
impl ports::principal::PrincipalRepository for PrincipalRepository {
    async fn save(&self, principal: &entities::principal::Principal) -> Result<(), ports::principal::PrincipalRepositoryError> {
        // Convert the Principal's identities into MongoDB documents
        let mut identity_docs = Vec::with_capacity(principal.identites().len());
        for identity in principal.identites() {
            identity_docs.push(FederatedIdentity::from((identity.clone(), principal.id.clone())))
        }

        // Convert the Principal into MongoDB documents
        let principal_doc = Principal::from(principal.clone());

        // Start a session
        let mut session = match self.client.start_session().await {
            Ok(s) => Ok(s),
            Err(err) => Err(ports::principal::PrincipalRepositoryError::FailedToStartSession(err.to_string()))
        }?;

        // Start a transaction
        session.start_transaction()
            .read_concern(ReadConcern::majority())
            .write_concern(WriteConcern::majority())
            .await
            .map_err(|err| ports::principal::PrincipalRepositoryError::FailedToStartTransaction(err.to_string()))?;


        // Run and retry the transaction on transient errors
        Self::save_principal_transaction(
            &principal_doc,
            &identity_docs,
            &self.principal_write_collection,
            &self.federated_identity_read_collection,
            &mut session,
        )
            .await
            .map_err(|err| ports::principal::PrincipalRepositoryError::FailedToSavePrincipal(err.to_string()))?;

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