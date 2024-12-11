use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub status: String,
    pub message: String,
    pub result: T,
    pub version: String,
    pub metadata: String,
}

impl<T> Response<T> {
    pub fn new(
        status: String,
        message: String,
        metadata: String,
        version: String,
        result: T
    ) -> Self {
        Self {
            status,
            message,
            result,
            version,
            metadata,
        }
    }
}
