use uuid::Uuid;
use nanoid::nanoid;

use crate::shared_kernel::identifiers::{traits::UrnGenerator, urn::Urn};

pub struct Endpoint {
    id: Uuid,
    target_urn: Urn,
    target_base_url: String,
    slug: String,
}

impl Endpoint {
    pub fn new_from_resource(resource: impl NetworkAddressableResource) -> Self {
        let id = Uuid::now_v7();
        Self {
            id,
            target_urn: resource.urn(),
            target_base_url: resource.get_base_url(),
            slug: Self::generate_slug()
        }
    }

    fn generate_slug() -> String {
        // 36 allowed characters: lowercase and digits only. The slug generated
        // from this alphabet will be compatible with both subdomains and routes
        let alphabet: [char; 36] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 
            'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
        ];
        
        // Generates your 10-character subdomain (e.g., "4x9m2qtz1b")
        nanoid!(10, &alphabet)
    } 
}

pub trait NetworkAddressableResource: UrnGenerator {
    fn get_base_url(&self) -> String;
}