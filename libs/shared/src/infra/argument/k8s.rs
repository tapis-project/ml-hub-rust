use std::collections::BTreeMap;
use async_trait::async_trait;
use uuid::Uuid;
use k8s_openapi::ByteString;
use k8s_openapi::api::core::v1::Secret;
use kube::{Client, Api};
use kube::api::{PostParams, DeleteParams, ObjectMeta};
use serde::{Serialize, Deserialize};

use crate::application::ports::errors::CommonRepositoryError;
use crate::domain::entities::deployment::argument::{Argument, ArgumentData};
use crate::shared_kernel::security::value_objects::{KeyId, Nonce};
use crate::shared_kernel::security::{EncryptionEnvelope, EncryptionEnvelopeMetadata};
use crate::shared_kernel::value_objects::Base64EncodedString;
use crate::application::ports::deployment_argument::{
    DeploymentArgumentRepository, 
    DeploymentArgumentRepositoryError
};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum K8sArgumentPayload {
    PlainText { 
        value: String 
    },
    Encrypted { 
        payload_base64: String, 
        key_id: String, 
        nonce: String 
    },
}

pub struct K8sDeploymentArgumentRepository {
    client: Client,
    namespace: String,
}

impl K8sDeploymentArgumentRepository {
    const MLHUB_ARGUMENT_SECRET_PREFIX: &'static str = "mlhub.deployment-arguments.";

    pub fn new(client: Client, namespace: String) -> Self {
        Self { client, namespace }
    }

    fn secret_name_from_deployment_id(deployment_id: &Uuid) -> String {
        format!("{}{}", Self::MLHUB_ARGUMENT_SECRET_PREFIX, deployment_id)
    }
}

#[async_trait]
impl DeploymentArgumentRepository for K8sDeploymentArgumentRepository {
    async fn save_all(
        &self, 
        deployment_id: &Uuid, 
        arguments: &[Argument]
    ) -> Result<(), DeploymentArgumentRepositoryError> {
        let secrets_api: Api<Secret> = Api::namespaced(self.client.clone(), &self.namespace);
        let secret_name = Self::secret_name_from_deployment_id(deployment_id);

        let mut secret_data = BTreeMap::new();
        for arg in arguments {
            let payload = match arg.data() {
                ArgumentData::PlainText(val) => K8sArgumentPayload::PlainText { 
                    value: val.clone() 
                },
                ArgumentData::Encrypted(envelope) => {
                    let EncryptionEnvelopeMetadata::AesGcm { key_id, nonce } = envelope.metadata();

                    K8sArgumentPayload::Encrypted {
                        payload_base64: envelope.payload().into_inner().to_string(),
                        key_id: key_id.to_string(),
                        nonce: Base64EncodedString::try_from(nonce.clone())
                            .map_err(|e| {
                                let error = CommonRepositoryError::new_internal();
                                log::error!("[{}] Data integrity error when converting from Nonce into Base64EncodedString: {}", error.error_id(), e.to_string());
                                error
                            })?
                            .into_inner()
                            .into(),
                    }
                }
            };

            let json_bytes = serde_json::to_vec(&payload).map_err(|e| {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                error
            })?;

            secret_data.insert(arg.parameter_name().to_string(), ByteString(json_bytes));
        }

        let target_secret = Secret {
            metadata: ObjectMeta {
                name: Some(secret_name.clone()),
                ..ObjectMeta::default()
            },
            data: Some(secret_data),
            ..Secret::default()
        };

        let _ = secrets_api.delete(&secret_name, &DeleteParams::default()).await;

        secrets_api
            .create(&PostParams::default(), &target_secret)
            .await
            .map_err(|_| {
                CommonRepositoryError::new_internal()
            })?;

        Ok(())
    }

    async fn find_all_for_deployment(
        &self, 
        deployment_id: &Uuid
    ) -> Result<Vec<Argument>, DeploymentArgumentRepositoryError> {
        let secrets_api: Api<Secret> = Api::namespaced(self.client.clone(), &self.namespace);
        let secret_name = Self::secret_name_from_deployment_id(deployment_id);

        let secret = match secrets_api.get(&secret_name).await {
            Ok(s) => s,
            Err(kube::Error::Api(status)) if status.is_not_found() => {
                return Ok(Vec::new());
            }
            Err(e) => {
                let error = CommonRepositoryError::new_internal();
                log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                return Err(DeploymentArgumentRepositoryError::from(error))
            }
        };

        let mut domain_arguments = Vec::new();

        if let Some(data_map) = secret.data {
            for (param_name, byte_string) in data_map {
                let payload: K8sArgumentPayload = serde_json::from_slice(&byte_string.0)
                    .map_err(|e| {
                        let error = CommonRepositoryError::new_internal();
                        log::error!("[{}] Persistence error: {}", error.error_id(), e.to_string());
                        error
                    })?;

                let argument = match payload {
                    K8sArgumentPayload::PlainText { value } => {
                        Argument::new_plaintext(param_name, value)
                    }
                    K8sArgumentPayload::Encrypted { payload_base64, key_id: key_id_string, nonce: nonce_string } => {
                        // Enforce your value object safety constraints using new_from_base64
                        let envelope_string = Base64EncodedString::new_from_base64(payload_base64)
                            .map_err(|e| {
                                let error = CommonRepositoryError::new_internal();
                                log::error!("[{}] Data integrity error: {}", error.error_id(), e.to_string());
                                error
                            })?;
                        
                        let key_id = KeyId::new(&key_id_string)
                            .map_err(|e| {
                                let error = CommonRepositoryError::new_internal();
                                log::error!("[{}] Data integrity error when creating KeyId from string: {}", error.error_id(), e.to_string());
                                error
                            })?;
                        
                        let b64_nonce = Base64EncodedString::new_from_base64(nonce_string)
                            .map_err(|e| {
                                let error = CommonRepositoryError::new_internal();
                                log::error!("[{}] Data integrity error when converting stored nonce string into Base64EncodedString: {}", error.error_id(), e.to_string());
                                error
                            })?;

                        let nonce = Nonce::try_from(b64_nonce)
                            .map_err(|e| {
                                let error = CommonRepositoryError::new_internal();
                                log::error!("[{}] Data integrity error: {}", error.error_id(), e.to_string());
                                error
                            })?;

                        let metadata = EncryptionEnvelopeMetadata::new_aes_gcm(
                            key_id,
                            nonce
                        );
                        let envelope = EncryptionEnvelope::new(envelope_string, metadata);
                        
                        Argument::new_encrypted(param_name, envelope)
                    }
                };

                domain_arguments.push(argument);
            }
        }

        Ok(domain_arguments)
    }
}
