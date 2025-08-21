use shared::presentation::http::v1::dto::headers::Headers;

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

pub fn token_from_headers(headers: &Headers) -> Option<String> {
    // First check the Authoriation
    let tapis_token = headers.get_first_value("Authorization");

    // Check to see if the tapis token passed via the X-Tapis-Token header
    if tapis_token.is_none() {
        return headers.get_first_value("X-Tapis-Token");
    }

    return tapis_token
}