use crate::database::{Database, Collection, INFERENCE_SERVER_COLLECTION};
use shared::inference::InferenceServer;

pub struct InferenceServerRepository {
    db: Database,
    collection: Collection<InferenceServer>
}

impl InferenceServerRepository {
    pub fn new(db: Database) -> Self {
        Self {
            collection: db.collection(INFERENCE_SERVER_COLLECTION),
            db,
        }
    }

    pub async fn _list(&self) {
        let _cursor = self.collection.find(None, None).await.unwrap();
    }
}