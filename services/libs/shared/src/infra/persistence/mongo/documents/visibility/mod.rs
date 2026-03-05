mod entity_to_document;
mod document_to_entity;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private
}