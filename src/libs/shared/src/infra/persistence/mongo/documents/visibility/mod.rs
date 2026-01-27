mod entity_to_document;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private
}