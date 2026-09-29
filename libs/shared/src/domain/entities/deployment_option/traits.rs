use crate::domain::entities::deployment_option::parameter_set::Parameter;

pub trait ProvideDeploymentParameters {
    fn provide_parameters(&self) -> Vec<Parameter>;
}
