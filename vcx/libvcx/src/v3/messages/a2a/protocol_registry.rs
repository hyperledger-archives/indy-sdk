use strum::IntoEnumIterator;

use v3::messages::a2a::A2AMessage;
use v3::messages::a2a::message_family::MessageFamilies;
use v3::messages::a2a::message_type::MessageType;

lazy_static! {
    pub static ref PROTOCOL_REGISTRY: ProtocolRegistry = ProtocolRegistry::init();
}

pub struct ProtocolRegistry {
    protocols: Vec<String>
}

impl ProtocolRegistry {
    fn init() -> ProtocolRegistry {
        let mut protocols = Vec::new();

        for a2a_message in A2AMessage::iter() {
            match a2a_message {
                A2AMessage::Generic(_) => {}
                a2a_message_ => {
                    protocols.push(a2a_message_.type_().to_string())
                }
            }
        }

        ProtocolRegistry { protocols }
    }

    pub fn add_protocol(&mut self, family: MessageFamilies, name: &str) {
        self.protocols.push(MessageType::build(family, name).to_string());
    }

    pub fn get_protocols_for_query(&self, query: Option<&str>) -> Vec<String> {
        use regex::Regex;

        match query {
            Some(query_) if query_ == "*" => self.protocols.clone(),
            Some(query_) => {
                match Regex::new(query_) {
                    Ok(re) => self.protocols.iter().filter(|protocol| re.is_match(protocol)).cloned().collect(),
                    Err(_) => vec![]
                }
            }
            None => self.protocols.clone()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn _protocols() -> Vec<String> {
        vec![
            "protocol_1.0_test".to_string(),
            "protocol_1.0_some".to_string(),
            "protocol_2.0_test".to_string()
        ]
    }

    fn _protocol_registry() -> ProtocolRegistry {
        ProtocolRegistry { protocols: _protocols() }
    }

    #[test]
    fn test_protocol_registry_init_works() {
        let registry: ProtocolRegistry = ProtocolRegistry::init();
        assert!(registry.protocols.len() > 0);
    }

    #[test]
    fn test_get_protocols_for_query_works_for_none_query() {
        let registry: ProtocolRegistry = _protocol_registry();
        let protocols = registry.get_protocols_for_query(None);
        assert_eq!(_protocols(), protocols);
    }

    #[test]
    fn test_get_protocols_for_query_works_for_placeholder() {
        let registry: ProtocolRegistry = _protocol_registry();

        let protocols = registry.get_protocols_for_query(Some("*"));
        assert_eq!(_protocols(), protocols);
    }

    #[test]
    fn test_get_protocols_for_query_works_for_partial() {
        let registry: ProtocolRegistry = _protocol_registry();

        let protocols = registry.get_protocols_for_query(Some("protocol_1.0*"));

        let expected_protocols = vec![
            "protocol_1.0_test".to_string(),
            "protocol_1.0_some".to_string(),
        ];
        assert_eq!(expected_protocols, protocols);
    }

    #[test]
    fn test_get_protocols_for_query_works_for_exact_protocol() {
        let registry: ProtocolRegistry = _protocol_registry();

        let protocols = registry.get_protocols_for_query(Some("protocol_1.0_test"));

        let expected_protocols = vec![
            "protocol_1.0_test".to_string(),
        ];
        assert_eq!(expected_protocols, protocols);
    }

    #[test]
    fn test_get_protocols_for_query_works_for_no_matching() {
        let registry: ProtocolRegistry = _protocol_registry();

        let protocols = registry.get_protocols_for_query(Some("test_some_other"));

        assert!(protocols.is_empty());
    }

    #[test]
    fn test_get_protocols_for_query_works_for_real() {
        let registry: ProtocolRegistry = ProtocolRegistry::init();
        let protocols = registry.get_protocols_for_query(Some("did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/connections/"));
        assert_eq!(4, protocols.len());
    }
}