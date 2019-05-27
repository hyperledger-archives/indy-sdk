use std::collections::HashSet;

use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde::de::{Deserializer, Deserialize};

use named_type::NamedType;

fn canon(attr: &str) -> String {
    attr.replace(" ", "").to_lowercase()
}

#[derive(Debug, NamedType)]
pub struct CredentialAttrTagPolicy {
    pub taggable: HashSet<String>
}

impl CredentialAttrTagPolicy {
    pub fn is_taggable(&self, attr_name: &str) -> bool {
        self.taggable.contains(canon(attr_name).as_str())
    }
}

impl From<Vec<String>> for CredentialAttrTagPolicy {
    fn from(taggables: Vec<String>) -> Self {
        CredentialAttrTagPolicy {
            taggable: taggables.into_iter().map(|a| canon(a.as_str())).collect()
        }
    }
}

impl Serialize for CredentialAttrTagPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        let mut seq = serializer.serialize_seq(Some(self.taggable.len()))?;
        for ref element in &self.taggable {
            seq.serialize_element(&element)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for CredentialAttrTagPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>, {
        let attr_names = Vec::deserialize(deserializer)?;
        Ok(CredentialAttrTagPolicy::from(attr_names))
    }
}