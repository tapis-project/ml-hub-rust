use crate::application::inputs::dataset_metadata as inputs;
use crate::domain::entities::task as domain_task;
use crate::domain::entities::dataset_metadata as domain;

use crate::application::errors::ApplicationError;

impl TryFrom<inputs::SystemRequirement> for domain::SystemRequirement {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}


impl TryFrom<inputs::Accelerator> for domain::Accelerator {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::Accelerator) -> Result<Self, Self::Error> {
        let mut system_requirements: Vec<domain::SystemRequirement> = Vec::with_capacity(1);
        for requirement in value.system_requirements {
            system_requirements.push(domain::SystemRequirement::try_from(requirement)?);
        }

        Ok(Self {
            accelerator_type: value.accelerator_type,
            memory_gb: value.memory_gb,
            cores: value.cores,
            system_requirements
        })
    }
}

impl TryFrom<inputs::HardwareRequirements> for domain::HardwareRequirements {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::HardwareRequirements) -> Result<Self, Self::Error> {
        let mut accelerators: Vec<domain::Accelerator> = Vec::with_capacity(1);
        for accelerator in value.accelerators.unwrap_or(Vec::with_capacity(0)) {
            accelerators.push(domain::Accelerator::try_from(accelerator)?);
        }

        Ok(Self {
            cpus: value.cpus,
            memory_gb: value.memory_gb,
            disk_gb: value.disk_gb,
            accelerators: Some(accelerators),
            architectures: value.architectures
        })
    }
}

impl TryFrom<inputs::DatasetIO> for domain::DatasetIO {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::DatasetIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

impl TryFrom<inputs::DatasetMetadata> for domain::DatasetMetadata {
    type Error = ApplicationError;
    
    fn try_from(value: inputs::DatasetMetadata) -> Result<Self, Self::Error> {

        Ok(Self {
            name: value.name,
            author: value.author,
        })
    }
}