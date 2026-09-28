pub enum ModelAction {
    Create,
    Read,
    List,
    Update,
    Delete,
    Deploy,
}

pub enum DatasetAction {
    Create,
    Read,
    List,
    Update,
    Delete,
}

pub enum ExternalModelAction {
    Read,
    List,
    Search,
    Create,
    Update,
}

pub enum DeploymentAction {
    Create,
    Read,
    List,
    Start,
    Stop,
    Delete,
    Undeploy,
}

pub enum HpcClusterAction {
    Create,
    Read,
    List,
    Update,
    Enable,
    Disable,
    Delete,
}

pub enum BatchSchedulerQueueAction {
    Create,
    Read,
    List,
    Update,
    Enable,
    Disable,
    Delete,
}