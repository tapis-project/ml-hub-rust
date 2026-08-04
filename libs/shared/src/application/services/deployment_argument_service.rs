use std::sync::Arc;

use crate::{
    application::{inputs::deployment::Argument as ArgumentInput, ports::{
        cipher::{Cipher, CipherError, CryptoContext},
        deployment_argument::{DeploymentArgumentRepository, DeploymentArgumentRepositoryError}
    }},
    domain::entities::{
        deployment::{
            argument::{Argument, ArgumentData},
            ModelDeployment
        },
        deployment_strategy::strategy::{Strategy, StrategyError}
    }
};

use retry_utils::{
    retry_async,
    FixedBackoff,
    Retry,
    RetryPolicy,
};

use thiserror::Error;
use once_cell::sync::Lazy;
use uuid::Uuid;


#[derive(Debug, Clone, Error)]
pub enum DeploymentArgumentServiceError {
    #[error(transparent)]
    DecryptionError(#[from] CipherError),

    #[error("Failed to convert decrypted argument data into UTF8: {0}")]
    Utf8ConversionError(String),

    #[error(transparent)]
    StrategyError(#[from] StrategyError),

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
        cipher: Arc<dyn Cipher>
    ) -> Self {
        Self { argument_repo, cipher }
    }

    pub async fn save(&self, deployment: &ModelDeployment, strategy: &Strategy, arguments: &[ArgumentInput]) -> Result<(), DeploymentArgumentServiceError> {
        let prepared_arguments = self.prepare_arguments(strategy, &arguments)
            .await?;

        let save_args = || self.argument_repo.save_all(&deployment.id, &prepared_arguments);

        Ok(retry_async(save_args, &Self::REPO_RETRY_POLICY, None).await?)
    }

    pub async fn get_decrypted_arguments_for_deployment(&self, deployment_id: &Uuid) -> Result<Vec<DecryptedArgument>, DeploymentArgumentServiceError> {
        let find_args = || self.argument_repo.find_all_for_deployment(deployment_id);

        let arguments = retry_async(find_args, &Self::REPO_RETRY_POLICY, None).await?;

        self.decrypt(arguments).await
    }

    pub async fn decrypt(&self, args: Vec<Argument>) -> Result<Vec<DecryptedArgument>, DeploymentArgumentServiceError> {
        let mut decrypted_arguments: Vec<DecryptedArgument> = Vec::with_capacity(args.len());
        for arg in args.iter() {
            let encryption_envelope = match arg.data() {
                ArgumentData::Encrypted(e) => e,
                ArgumentData::PlainText(d) => {
                    decrypted_arguments.push(
                        DecryptedArgument {
                            parameter_name: arg.parameter_name().into(),
                            value: d.clone()
                        }
                    );

                    continue
                }
            };

            let bytes = self.cipher.decrypt(encryption_envelope).await?;

            let decrypted_value = String::from_utf8(bytes)
                .map_err(|e| DeploymentArgumentServiceError::Utf8ConversionError(e.to_string()))?;

            decrypted_arguments.push(
                DecryptedArgument {
                    parameter_name: arg.parameter_name().into(),
                    value: decrypted_value,
                }
            );
        }

        Ok(decrypted_arguments)
    }

    pub async fn prepare_arguments(&self, strategy: &Strategy, inputs: &[ArgumentInput]) -> Result<Vec<Argument>, DeploymentArgumentServiceError> {
        let mut prepared_args: Vec<Argument> = vec![];
        for arg in inputs {
            // Create the non-secret arguments
            if !strategy.is_parameter_secret(&arg.parameter_name) {
                prepared_args.push(Argument::new_plaintext(arg.parameter_name.clone(), arg.value.clone()));
                continue
            }

            // Encrypt the argument value and create a secret argument
            let encryption_envelope = self.cipher.encrypt(
                CryptoContext::DeploymentArgumentSecret,
                arg.value.clone().into_bytes()
            ).await?;

            prepared_args.push(Argument::new_encrypted(arg.parameter_name.clone(), encryption_envelope))
        }
       
        strategy.validate_arguments(&prepared_args)?;

        Ok(prepared_args)
    }
}