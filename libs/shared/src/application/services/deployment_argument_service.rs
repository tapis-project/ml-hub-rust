use std::sync::Arc;

use crate::{
    application::ports::{
        cipher::{Cipher, CipherError, CryptoContext},
        deployment_argument::{DeploymentArgumentRepository, DeploymentArgumentRepositoryError},
    },
    domain::entities::{
        deployment::argument::{Argument, ArgumentData},
        deployment_option::deployment_parameters::ResolvedDeploymentParameter,
    },
};

use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};

use once_cell::sync::Lazy;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Error)]
pub enum DeploymentArgumentServiceError {
    #[error(transparent)]
    DecryptionError(#[from] CipherError),

    #[error("Failed to convert decrypted argument data into UTF8: {0}")]
    Utf8ConversionError(String),

    #[error(transparent)]
    DeploymentArgumentPersistenceError(#[from] DeploymentArgumentRepositoryError),
}

#[derive(Debug, Clone)]
pub struct DecryptedArgument {
    pub parameter_name: String,
    pub value: String,
}

pub struct DeploymentArgumentService {
    argument_repo: Arc<dyn DeploymentArgumentRepository>,
    cipher: Arc<dyn Cipher>,
}

impl DeploymentArgumentService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(
        argument_repo: Arc<dyn DeploymentArgumentRepository>,
        cipher: Arc<dyn Cipher>,
    ) -> Self {
        Self {
            argument_repo,
            cipher,
        }
    }

    pub async fn get_decrypted_arguments_for_deployment(
        &self,
        deployment_id: &Uuid,
    ) -> Result<Vec<DecryptedArgument>, DeploymentArgumentServiceError> {
        let find_args = || self.argument_repo.find_all_for_deployment(deployment_id);

        let arguments = retry_async(find_args, &Self::REPO_RETRY_POLICY, None).await?;

        self.decrypt(arguments).await
    }

    pub async fn decrypt(
        &self,
        args: Vec<Argument>,
    ) -> Result<Vec<DecryptedArgument>, DeploymentArgumentServiceError> {
        let mut decrypted_arguments: Vec<DecryptedArgument> = Vec::with_capacity(args.len());

        for arg in args.iter() {
            let encryption_envelope = match arg.data() {
                ArgumentData::Encrypted(e) => e,
                ArgumentData::PlainText(d) => {
                    decrypted_arguments.push(DecryptedArgument {
                        parameter_name: arg.parameter_name().into(),
                        value: d.clone(),
                    });

                    continue;
                }
            };

            let bytes = self.cipher.decrypt(encryption_envelope).await?;

            let decrypted_value = String::from_utf8(bytes)
                .map_err(|e| DeploymentArgumentServiceError::Utf8ConversionError(e.to_string()))?;

            decrypted_arguments.push(DecryptedArgument {
                parameter_name: arg.parameter_name().into(),
                value: decrypted_value,
            });
        }

        Ok(decrypted_arguments)
    }

    pub async fn prepare_arguments(
        &self,
        resolved_parameters: &[ResolvedDeploymentParameter],
    ) -> Result<Vec<Argument>, DeploymentArgumentServiceError> {
        let mut prepared_args = Vec::with_capacity(resolved_parameters.len());

        for parameter in resolved_parameters {
            if !parameter.secret() {
                prepared_args.push(Argument::new_plaintext(
                    parameter.name().into(),
                    parameter.value().into(),
                ));

                continue;
            }

            let encryption_envelope = self
                .cipher
                .encrypt(
                    CryptoContext::DeploymentArgumentSecret,
                    parameter.value().as_bytes().to_vec(),
                )
                .await?;

            prepared_args.push(Argument::new_encrypted(
                parameter.name().into(),
                encryption_envelope,
            ));
        }

        Ok(prepared_args)
    }
}
