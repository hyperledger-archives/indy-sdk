pub mod credential;
pub mod credential_definition;
pub mod credential_for_proof_request;
pub mod credential_offer;
pub mod credential_request;
pub mod filter;
pub mod proof;
pub mod proof_request;
pub mod requested_credential;
pub mod revocation_registry_definition;
pub mod revocation_registry_delta;
pub mod revocation_state;
pub mod schema;

pub const DELIMITER: &'static str = ":";

fn build_id(did: &str, marker: &str, related_entity_id: Option<&str>, word1: &str, word2: &str) -> String {
    let related_entity_id = related_entity_id.map(|s| format!("{}{}", s, DELIMITER)).unwrap_or(String::new());
    format!("{}{}{}{}{}{}{}{}", did, DELIMITER, marker, DELIMITER, related_entity_id, word1, DELIMITER, word2)
}