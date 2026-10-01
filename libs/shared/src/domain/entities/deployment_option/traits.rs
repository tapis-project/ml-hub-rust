use crate::domain::entities::deployment_option::deployment_parameters::Parameter;

pub trait ProvideDeploymentParameters {
    fn provide_parameters(&self) -> Vec<Parameter>;
}
