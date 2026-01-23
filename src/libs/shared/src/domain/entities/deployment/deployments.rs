use crate::domain::entities::visibility::Visiblity;

pub enum ModelDeploymentInterface {
    RestApi,
    Grpc,
}

pub enum Accelerator {
    Gpu {
        /// Number of GPUs
        count: u16,
        vram_gb: u64,
        vendor: Option<String>
    }
}

pub struct Resources {
    pub memory_gb: u64,
    pub disk_gb: u64,
    pub cpu_millicores: u32,
    pub accelerator: Option<Accelerator>,
}

pub struct ModelDeployent {
    pub model_reference: ModelReference,
    pub status: ModelDeploymentStatus,
    pub resources: Resources,
    pub visibility: Visiblity,
}