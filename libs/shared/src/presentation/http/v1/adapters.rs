use crate::bootstrap::Idp;

pub fn derive_header_keys_from_authorities() -> Vec<String> {
    let mut header_names: Vec<String> = vec![ String::from("Authorization") ];
    
    for authority in Idp::all() {
        match authority {
            Idp::Tapis => {
                header_names.push(String::from("X-Tapis-Token"))
            }
        }
    }

    header_names
}