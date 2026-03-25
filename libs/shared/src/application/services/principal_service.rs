use crate::application::errors::ApplicationError;
use crate::domain::entities::principal::Principal;
use crate::application::ports::principal::PrincipalRepository;

use std::sync::Arc;

pub struct PrincipalService {
    principal_repository: Arc<dyn PrincipalRepository>
}

impl PrincipalService {
    pub async fn save(&self, principal: Principal) -> Result<(), ApplicationError> {
        let x = self.principal_repository.save(&principal).await;
        Ok(())
    }
}