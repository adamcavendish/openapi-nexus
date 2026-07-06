use heck::ToLowerCamelCase as _;

/// Stable operation-response entry name used by language emitters.
///
/// This names the OpenAPI response entry itself. Body schema names are payload
/// types and must not decide classification.
pub fn response_entry_name(status: &str) -> String {
    let upper = status.to_ascii_uppercase();
    match status {
        s if s.eq_ignore_ascii_case("default") => "Default".to_string(),
        s if s.eq_ignore_ascii_case("4XX") => "ClientError".to_string(),
        s if s.eq_ignore_ascii_case("5XX") => "ServerError".to_string(),
        s if upper.ends_with("XX") => format!("Status{}xx", &s[..s.len() - 2]),
        s => match s.parse::<u16>().ok() {
            Some(400) => "BadRequest".to_string(),
            Some(401) => "Unauthorized".to_string(),
            Some(403) => "Forbidden".to_string(),
            Some(404) => "NotFound".to_string(),
            Some(409) => "Conflict".to_string(),
            Some(422) => "Validation".to_string(),
            Some(429) => "TooManyRequests".to_string(),
            Some(500) => "InternalServerError".to_string(),
            Some(502) => "BadGateway".to_string(),
            Some(503) => "ServiceUnavailable".to_string(),
            Some(504) => "GatewayTimeout".to_string(),
            Some(code) => format!("Status{code}"),
            None => "Unexpected".to_string(),
        },
    }
}

pub fn response_entry_kind(status: &str) -> String {
    response_entry_name(status).to_lower_camel_case()
}

pub fn response_match_rank(status: &str) -> u8 {
    if status.parse::<u16>().is_ok() {
        0
    } else if status.to_ascii_uppercase().ends_with("XX") {
        1
    } else if status.eq_ignore_ascii_case("default") {
        2
    } else {
        3
    }
}
