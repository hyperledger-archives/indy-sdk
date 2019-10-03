use regex::Regex;

lazy_static! {
    pub static ref REGEX: Regex = Regex::new("did:([a-z0-9]+):([a-zA-Z0-9:.-_]*)").unwrap();
}

pub struct Qualifier {}

impl Qualifier {
    pub fn is_fully_qualified(entity: &str) -> bool {
        REGEX.is_match(&entity)
    }
}