use crate::domain::entities::deployment::ModelDeployment;

pub struct DeployModelWithOptionOutput {
    pub deployment: ModelDeployment,
}

pub struct StartModelDeploymentOutput {
    pub deployment: ModelDeployment,
}

pub struct StopModelDeploymentOutput {
    pub deployment: ModelDeployment,
}

pub struct UndeployModelDeploymentOutput {
    pub deployment: ModelDeployment,
}
