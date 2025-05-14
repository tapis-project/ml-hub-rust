use actix_web::http::header::HeaderMap;
use crate::errors::Error;
use crate::common::presentation::http::v1::dto::Headers;
 
impl TryFrom<&HeaderMap> for Headers {
    type Error = Error;

    fn try_from(value: &HeaderMap) -> Result<Self, Self::Error> {
        let headers = value
            .iter()
            .map(|(k, v)| {
                let name = k.to_string();
                let value = v.to_str().map_err(|err| {
                    Self::Error::new(format!(
                        "Header '{}' cannot be converted into a string: {}",
                        name, err
                    ))
                })?;

                Ok((name, value.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Headers::new(headers))
    }
}