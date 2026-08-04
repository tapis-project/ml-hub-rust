use async_trait::async_trait;
use mongodb::Client;
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::ports::deployment_argument::{
    DeploymentArgumentRepository, DeploymentArgumentRepositoryError,
};
use crate::application::ports::errors::CommonRepositoryError;
use crate::domain::entities::deployment::argument::{Argument, ArgumentData};
use crate::infra::persistence::mongo::database::DEPLOYMENT_ARGUMENT_COLLECTION;
use crate::shared_kernel::security::value_objects::{KeyId, Nonce};
use crate::shared_kernel::security::{EncryptionEnvelope, EncryptionEnvelopeMetadata};
use crate::shared_kernel::value_objects::Base64EncodedString;

/// MongoDB persistence model. `deployment_id` is stored as a string so this
/// repository does not depend on a particular BSON UUID representation.
#[derive(Debug, Serialize, Deserialize)]
struct DeploymentArgumentsDocument {
    #[serde(rename = "_id")]
    deployment_id: String,
    arguments: Vec<StoredArgument>,
}

/// An array is used instead of a BSON document keyed by `parameter_name`.
/// That means parameter names containing `.` or `$` remain safe to persist.
#[derive(Debug, Serialize, Deserialize)]
struct StoredArgument {
    parameter_name: String,
    #[serde(flatten)]
    payload: MongoArgumentPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum MongoArgumentPayload {
    PlainText {
        value: String,
    },
    Encrypted {
        payload_base64: String,
        key_id: String,
        nonce: String,
    },
}

pub struct MongoDeploymentArgumentRepository {
    read_collection: Collection<DeploymentArgumentsDocument>,
    write_collection: Collection<DeploymentArgumentsDocument>
}

impl MongoDeploymentArgumentRepository {
    pub fn new(client: &Client, db_name: &str) -> Self {
        let db = client.database(&db_name);

        Self {
            write_collection: db.collection(DEPLOYMENT_ARGUMENT_COLLECTION),
            read_collection: db.collection(DEPLOYMENT_ARGUMENT_COLLECTION)
        }
    }

    fn internal_error(context: &str, error: impl std::fmt::Display) -> CommonRepositoryError {
        let repository_error = CommonRepositoryError::new_internal();
        log::error!("[{}] {}: {}", repository_error.error_id(), context, error);
        repository_error
    }

    fn to_stored_argument(
        argument: &Argument,
    ) -> Result<StoredArgument, DeploymentArgumentRepositoryError> {
        let payload = match argument.data() {
            ArgumentData::PlainText(value) => MongoArgumentPayload::PlainText {
                value: value.clone(),
            },
            ArgumentData::Encrypted(envelope) => {
                let EncryptionEnvelopeMetadata::AesGcm { key_id, nonce } = envelope.metadata();

                let nonce = Base64EncodedString::try_from(nonce.clone())
                    .map_err(|error| {
                        Self::internal_error("Could not encode encrypted argument nonce", error)
                    })?
                    .into_inner()
                    .into();

                MongoArgumentPayload::Encrypted {
                    payload_base64: envelope.payload().into_inner().to_string(),
                    key_id: key_id.to_string(),
                    nonce,
                }
            }
        };

        Ok(StoredArgument {
            parameter_name: argument.parameter_name().to_string(),
            payload,
        })
    }

    fn to_domain_argument(
        stored: StoredArgument,
    ) -> Result<Argument, DeploymentArgumentRepositoryError> {
        match stored.payload {
            MongoArgumentPayload::PlainText { value } => {
                Ok(Argument::new_plaintext(stored.parameter_name, value))
            }
            MongoArgumentPayload::Encrypted {
                payload_base64,
                key_id,
                nonce,
            } => {
                let payload =
                    Base64EncodedString::new_from_base64(payload_base64).map_err(|error| {
                        Self::internal_error("Invalid encrypted argument payload in MongoDB", error)
                    })?;

                let key_id = KeyId::new(&key_id).map_err(|error| {
                    Self::internal_error("Invalid encrypted argument key ID in MongoDB", error)
                })?;

                let nonce = Base64EncodedString::new_from_base64(nonce).map_err(|error| {
                    Self::internal_error("Invalid encrypted argument nonce in MongoDB", error)
                })?;

                let nonce = Nonce::try_from(nonce).map_err(|error| {
                    Self::internal_error("Could not reconstruct encrypted argument nonce", error)
                })?;

                let metadata = EncryptionEnvelopeMetadata::new_aes_gcm(key_id, nonce);
                Ok(Argument::new_encrypted(
                    stored.parameter_name,
                    EncryptionEnvelope::new(payload, metadata),
                ))
            }
        }
    }
}

#[async_trait]
impl DeploymentArgumentRepository for MongoDeploymentArgumentRepository {
    async fn save_all(
        &self,
        deployment_id: &Uuid,
        arguments: &[Argument],
    ) -> Result<(), DeploymentArgumentRepositoryError> {
        let arguments = arguments
            .iter()
            .map(Self::to_stored_argument)
            .collect::<Result<Vec<_>, _>>()?;

        let document = DeploymentArgumentsDocument {
            deployment_id: deployment_id.to_string(),
            arguments,
        };

        self.write_collection
            .replace_one(doc! { "_id": &document.deployment_id }, document)
            .upsert(true)
            .await
            .map_err(|error| {
                Self::internal_error("Could not save deployment arguments to MongoDB", error)
            })?;

        Ok(())
    }

    async fn find_all_for_deployment(
        &self,
        deployment_id: &Uuid,
    ) -> Result<Vec<Argument>, DeploymentArgumentRepositoryError> {
        let document = self
            .read_collection
            .find_one(doc! { "_id": deployment_id.to_string() })
            .await
            .map_err(|error| {
                Self::internal_error("Could not read deployment arguments from MongoDB", error)
            })?;

        document
            .map(|document| {
                document
                    .arguments
                    .into_iter()
                    .map(Self::to_domain_argument)
                    .collect()
            })
            .unwrap_or_else(|| Ok(Vec::new()))
    }
}
