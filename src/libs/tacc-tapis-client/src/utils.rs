use shared::requests::HttpRequest;
use shared::requests::utils::get_header_value;

pub fn build_tenant_base_url(tenant: String) -> String {
    format!(
        "https://{}.tapis.io/v3",
        tenant
    )
}

pub fn build_operation_url(
    tenant: String,
    api: String,
    path: Option<String>
) -> String {
    let mut url = format!(
        "{}/{}",
        build_tenant_base_url(tenant),
        api
    );

    if let Some(value) = path {
        url = format!(
            "{}/{}",
            url,
            value.strip_prefix("/")
                .unwrap_or(&value)
        );
    }

    return url
}

pub fn token_from_request(request: &HttpRequest) -> Option<String> {
    // First check the Authoriation
    let tapis_token = get_header_value(
        "Authorization", 
        request
    );

    // Check to see if the tapis token passed via the X-Tapis-Token header
    if tapis_token.is_none() {
        return get_header_value(
            "X-Tapis-Token", 
            request
        )
    }

    return tapis_token
}