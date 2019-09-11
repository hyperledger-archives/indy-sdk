use regex::Regex;

pub const DEFAULT_PREFIX: &'static str = "did:sov:";

pub struct Qualifier {}

lazy_static! {
    pub static ref REGEX: Regex = Regex::new("^(did:[a-z0-9]+:)([a-zA-Z0-9:.-_]*)").unwrap();
}

impl Qualifier {
    pub fn qualify(entity: &str, prefix: Option<String>) -> String {
        if Qualifier::is_fully_qualified(entity) {
            format!("{}{}", prefix.unwrap_or(DEFAULT_PREFIX.to_string()), entity)
        } else {
            entity.to_string()
        }
    }

    pub fn unqualify(entity: &str) -> String {
        match REGEX.captures(entity) {
            None => entity.to_string(),
            Some(caps) => {
                caps.get(2).map(|m| m.as_str().to_string()).unwrap_or(entity.to_string())
            }
        }
    }

    pub fn prefix(entity: &str) -> Option<String> {
        match REGEX.captures(entity) {
            None => None,
            Some(caps) => {
                caps.get(1).map(|m| m.as_str().to_string())
            }
        }
    }

    pub fn is_fully_qualified(entity: &str) -> bool {
        REGEX.is_match(&entity)
    }
}