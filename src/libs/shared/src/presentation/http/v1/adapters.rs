use crate::domain::entities::identity::Authority;

pub fn derive_header_keys_from_authorites() -> Vec<String> {
    let mut header_names: Vec<String> = vec![ String::from("Authorization") ];
    
    for authority in Authority::all() {
        match authority {
            Authority::Tapis => {
                header_names.push(String::from("X-Tapis-Token"))
            }
        }
    }

    header_names
}