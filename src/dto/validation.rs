use std::net::IpAddr;

use url::Url;

const MAX_URL_LENGTH: usize = 2048;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UrlValidationError {
    Empty,
    TooLong,
    InvalidUrl,
    UnsupportedScheme,
    MissingHost,
    BlockedHost,
}

impl UrlValidationError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::Empty => "URL is required",
            Self::TooLong => "URL is too long (max 2048 characters)",
            Self::InvalidUrl => "URL is not valid",
            Self::UnsupportedScheme => "URL must start with http:// or https://",
            Self::MissingHost => "URL must include a host",
            Self::BlockedHost => "URL host is not allowed",
        }
    }
}

pub fn validate_original_url(input: &str) -> Result<String, UrlValidationError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(UrlValidationError::Empty);
    }

    if trimmed.len() > MAX_URL_LENGTH {
        return Err(UrlValidationError::TooLong);
    }

    let parsed = Url::parse(trimmed).map_err(|_| UrlValidationError::InvalidUrl)?;

    match parsed.scheme() {
        "http" | "https" => {}
        _ => return Err(UrlValidationError::UnsupportedScheme),
    }

    let host = parsed.host_str().ok_or(UrlValidationError::MissingHost)?;

    if is_blocked_host(host) {
        return Err(UrlValidationError::BlockedHost);
    }

    Ok(trimmed.to_owned())
}

fn is_blocked_host(host: &str) -> bool {
    let host_lower = host.to_lowercase();

    if matches!(
        host_lower.as_str(),
        "localhost" | "localhost.localdomain" | "0.0.0.0"
    ) {
        return true;
    }

    if let Ok(ip) = host.parse::<IpAddr>() {
        return is_blocked_ip(ip);
    }

    false
}

fn is_blocked_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(addr) => {
            addr.is_loopback()
                || addr.is_private()
                || addr.is_link_local()
                || addr.is_unspecified()
                || addr.is_broadcast()
        }
        IpAddr::V6(addr) => addr.is_loopback() || addr.is_unspecified() || addr.is_multicast(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_https_url() {
        let url = validate_original_url("  https://example.com/path  ").unwrap();
        assert_eq!(url, "https://example.com/path");
    }

    #[test]
    fn rejects_javascript_scheme() {
        assert_eq!(
            validate_original_url("javascript:alert(1)"),
            Err(UrlValidationError::UnsupportedScheme)
        );
    }

    #[test]
    fn rejects_localhost() {
        assert_eq!(
            validate_original_url("http://localhost:3000"),
            Err(UrlValidationError::BlockedHost)
        );
    }

    #[test]
    fn rejects_private_ip() {
        assert_eq!(
            validate_original_url("http://192.168.1.1/admin"),
            Err(UrlValidationError::BlockedHost)
        );
    }
}
