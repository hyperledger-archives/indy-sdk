extern crate indy_crypto;

use self::indy_crypto::cl::{
    CredentialSignature,
    RevocationRegistry,
    SignatureCorrectnessProof,
    Witness
};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use super::DELIMITER;

use std::collections::HashMap;
use named_type::NamedType;

#[derive(Debug, Deserialize, Serialize, NamedType)]
pub struct Credential {
    pub schema_id: String,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>,
    pub values: HashMap<String, AttributeValues>,
    pub signature: CredentialSignature,
    pub signature_correctness_proof: SignatureCorrectnessProof,
    pub rev_reg: Option<RevocationRegistry>,
    pub witness: Option<Witness>
}

impl Credential {
    fn schema_parts(&self) -> Vec<&str> {
        self.schema_id.split_terminator(DELIMITER).collect::<Vec<&str>>()
    }

    pub fn schema_id(&self) -> String { self.schema_id.to_string() }

    pub fn schema_issuer_did(&self) -> String {
        self.schema_parts().get(0).map(|s| s.to_string()).unwrap_or(String::new())
    }

    pub fn schema_name(&self) -> String {
        self.schema_parts().get(2).map(|s| s.to_string()).unwrap_or(String::new())
    }

    pub fn schema_version(&self) -> String {
        self.schema_parts().get(3).map(|s| s.to_string()).unwrap_or(String::new())
    }

    pub fn issuer_did(&self) -> String {
        self.cred_def_id.split_terminator(DELIMITER).collect::<Vec<&str>>()[0].to_string()
    }

    pub fn cred_def_id(&self) -> String { self.cred_def_id.to_string() }
}

impl JsonEncodable for Credential {}

impl<'a> JsonDecodable<'a> for Credential {}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct CredentialInfo {
    pub referent: String,
    pub attrs: HashMap<String, String>,
    pub schema_id: String,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>,
    pub cred_rev_id: Option<String>
}

impl JsonEncodable for CredentialInfo {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttributeValues {
    pub raw: String,
    pub encoded: String
}