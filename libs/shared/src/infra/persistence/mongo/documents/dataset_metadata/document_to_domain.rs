use crate::infra::persistence::mongo::documents::dataset_metadata;
use crate::domain::entities::dataset_metadata as domain;
use crate::domain::entities::task as domain_task;
use crate::errors::Error;

impl TryFrom<dataset_metadata::SystemRequirement> for domain::SystemRequirement {
    type Error = Error;
    
    fn try_from(value: dataset_metadata::SystemRequirement) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            version: value.version
        })
    }
}

impl TryFrom<dataset_metadata::Accelerator> for domain::Accelerator {
    type Error = Error;
    
    fn try_from(value: dataset_metadata::Accelerator) -> Result<Self, Self::Error> {
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

impl TryFrom<dataset_metadata::HardwareRequirements> for domain::HardwareRequirements {
    type Error = Error;
    
    fn try_from(value: dataset_metadata::HardwareRequirements) -> Result<Self, Self::Error> {
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

impl TryFrom<dataset_metadata::DatasetIO> for domain::DatasetIO {
    type Error = Error;
    
    fn try_from(value: dataset_metadata::DatasetIO) -> Result<Self, Self::Error> {
        Ok(Self {
            data_type: value.data_type,
            shape: value.shape
        })
    }
}

impl TryFrom<dataset_metadata::DatasetMetadata> for domain::DatasetMetadata {
    type Error = Error;
    
    fn try_from(value: dataset_metadata::DatasetMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            author: value.author
        })
    }
}