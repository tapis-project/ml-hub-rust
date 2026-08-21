use crate::shared_kernel::security::EncryptionEnvelope;

#[derive(Clone, Debug)]
pub struct Argument {
    pub parameter_name: String,
    data: ArgumentData,
}

impl Argument {
    pub fn new_plaintext(parameter_name: String, value: String) -> Self {
        Argument {
            parameter_name,
            data: ArgumentData::PlainText(value)
        }
    }

    pub fn new_encrypted(parameter_name: String, encryption_envelope: EncryptionEnvelope) -> Self {
        Argument {
            parameter_name,
            data: ArgumentData::Encrypted(encryption_envelope)
        }
    }

    pub fn parameter_name(&self) -> &str {
        &self.parameter_name
    }

    pub fn data(&self) -> &ArgumentData {
        &self.data
    }
}

#[derive(Clone, Debug)]
pub enum ArgumentData {
    PlainText(String),
    Encrypted(EncryptionEnvelope),
}