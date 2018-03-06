extern crate indy_crypto;

use std::collections::{HashMap, HashSet};

use self::indy_crypto::cl::*;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttributeInfo {
    pub name: String,
    pub restrictions: Option<Vec<Filter>>,
    pub freshness: Option<u64>
}

impl JsonEncodable for AttributeInfo {}

impl<'a> JsonDecodable<'a> for AttributeInfo {}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct Filter {
    pub schema_id: Option<String>,
    pub schema_did: Option<String>,
    pub schema_name: Option<String>,
    pub schema_version: Option<String>,
    pub issuer_did: Option<String>,
    pub cred_def_id: Option<String>
}

impl<'a> JsonDecodable<'a> for Filter {}

pub trait Filtering {
    fn schema_id(&self) -> String;
    fn schema_did(&self) -> String;
    fn schema_name(&self) -> String;
    fn schema_version(&self) -> String;
    fn issuer_did(&self) -> String;
    fn cred_def_id(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialOffer {
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>,
    pub key_correctness_proof: CredentialKeyCorrectnessProof,
    pub nonce: Nonce
}

impl JsonEncodable for CredentialOffer {}

impl<'a> JsonDecodable<'a> for CredentialOffer {}

impl Filtering for CredentialOffer {
    fn schema_id(&self) -> String { get_parts(&self.cred_def_id)[2..6].join(":").to_string() }
    fn schema_did(&self) -> String { get_parts(&self.cred_def_id)[2].to_string() }
    fn schema_name(&self) -> String { get_parts(&self.cred_def_id)[4].to_string() }
    fn schema_version(&self) -> String { get_parts(&self.cred_def_id)[5].to_string() }
    fn issuer_did(&self) -> String { get_parts(&self.cred_def_id)[0].to_string() }
    fn cred_def_id(&self) -> String { self.cred_def_id.to_string() }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct CredentialInfo {
    pub referent: String,
    pub attrs: HashMap<String, String>,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>
}

impl Filtering for CredentialInfo {
    fn schema_id(&self) -> String { get_parts(&self.cred_def_id)[2..6].join(":").to_string() }
    fn schema_did(&self) -> String { get_parts(&self.cred_def_id)[2].to_string() }
    fn schema_name(&self) -> String { get_parts(&self.cred_def_id)[4].to_string() }
    fn schema_version(&self) -> String { get_parts(&self.cred_def_id)[5].to_string() }
    fn issuer_did(&self) -> String { get_parts(&self.cred_def_id)[0].to_string() }
    fn cred_def_id(&self) -> String { self.cred_def_id.to_string() }
}

fn get_parts(id: &str) -> Vec<&str> {
    id.split_terminator(":").collect::<Vec<&str>>()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialRequest {
    pub prover_did: String,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>,
    pub blinded_ms: BlindedMasterSecret,
    pub blinded_ms_correctness_proof: BlindedMasterSecretCorrectnessProof,
    pub nonce: Nonce,
}

impl JsonEncodable for CredentialRequest {}

impl<'a> JsonDecodable<'a> for CredentialRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialRequestMetadata {
    pub master_secret_blinding_data: MasterSecretBlindingData,
    pub nonce: Nonce,
    pub master_secret_name: String
}

impl JsonEncodable for CredentialRequestMetadata {}

impl<'a> JsonDecodable<'a> for CredentialRequestMetadata {}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub enum SignatureTypes {
    CL
}

impl<'a> JsonDecodable<'a> for SignatureTypes {}

impl SignatureTypes {
    pub fn to_str(&self) -> &'static str {
        match self {
            &SignatureTypes::CL => "CL"
        }
    }
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDefinition {
    pub id: String,
    pub schema_id: String,
    #[serde(rename = "type")]
    pub signature_type: SignatureTypes,
    pub tag: String,
    pub value: CredentialDefinitionValue
}

impl JsonEncodable for CredentialDefinition {}

impl<'a> JsonDecodable<'a> for CredentialDefinition {}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct CredentialDefinitionValue {
    pub primary: CredentialPrimaryPublicKey,
    pub revocation: Option<CredentialRevocationPublicKey>
}

impl JsonEncodable for CredentialDefinitionValue {}

impl<'a> JsonDecodable<'a> for CredentialDefinitionValue {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Credential {
    pub values: HashMap<String, AttributeValues>,
    pub signature: CredentialSignature,
    pub signature_correctness_proof: SignatureCorrectnessProof,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>
}

impl JsonEncodable for Credential {}

impl<'a> JsonDecodable<'a> for Credential {}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PredicateInfo {
    pub attr_name: String,
    pub p_type: String,
    pub value: i32,
    pub restrictions: Option<Vec<Filter>>,
    pub freshness: Option<u64>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialsForProofRequest {
    pub attrs: HashMap<String, Vec<RequestedCredential>>,
    pub predicates: HashMap<String, Vec<RequestedCredential>>
}

impl JsonEncodable for CredentialsForProofRequest {}

impl<'a> JsonDecodable<'a> for CredentialsForProofRequest {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedCredential {
    pub cred_info: CredentialInfo,
    pub freshness: Option<u64>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofRequest {
    pub nonce: Nonce,
    pub name: String,
    pub version: String,
    pub requested_attrs: HashMap<String, AttributeInfo>,
    pub requested_predicates: HashMap<String, PredicateInfo>,
    pub freshness: Option<u64>
}

impl JsonEncodable for ProofRequest {}

impl<'a> JsonDecodable<'a> for ProofRequest {}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub schema_id: String,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>,
    pub timestamp: Option<u64>
}

impl JsonEncodable for Identifier {}

impl<'a> JsonDecodable<'a> for Identifier {}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullProof {
    pub proof: Proof,
    pub requested_proof: RequestedProof,
    pub identifiers: HashMap<String, Identifier>
}

impl JsonEncodable for FullProof {}

impl<'a> JsonDecodable<'a> for FullProof {}

#[allow(non_camel_case_types)] //FIXME
#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub enum RevocationRegistryTypes {
    CL_ACCUM,
}

impl<'a> JsonDecodable<'a> for RevocationRegistryTypes {}

impl RevocationRegistryTypes {
    pub fn to_str(&self) -> &'static str {
        match self {
            &RevocationRegistryTypes::CL_ACCUM => "CL_ACCUM"
        }
    }
}

#[allow(non_camel_case_types)] //FIXME
#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub enum IssuanceTypes {
    ISSUANCE_BY_DEFAULT,
    ISSUANCE_ON_DEMAND
}

impl<'a> JsonDecodable<'a> for IssuanceTypes {}

impl IssuanceTypes {
    pub fn to_bool(&self) -> bool {
        self.clone() == IssuanceTypes::ISSUANCE_BY_DEFAULT
    }
}


#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RevocationRegistryDefinitionValuePublicKeys {
    pub accum_key: RevocationKeyPublic
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RevocationRegistryConfig {
    pub issuance_type: Option<String>,
    pub max_cred_num: u32
}

impl JsonEncodable for RevocationRegistryConfig {}

impl<'a> JsonDecodable<'a> for RevocationRegistryConfig {}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryDefinition {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: RevocationRegistryTypes,
    pub tag: String,
    pub cred_def_id: String,
    pub value: RevocationRegistryDefinitionValue
}

impl JsonEncodable for RevocationRegistryDefinition {}

impl<'a> JsonDecodable<'a> for RevocationRegistryDefinition {}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RevocationRegistryDefinitionValue {
    pub issuance_type: IssuanceTypes,
    pub max_cred_num: u32,
    pub public_keys: RevocationRegistryDefinitionValuePublicKeys,
    pub tails_hash: String,
    pub tails_location: String
}

impl JsonEncodable for RevocationRegistryDefinitionValue {}

impl<'a> JsonDecodable<'a> for RevocationRegistryDefinitionValue {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedCredentials {
    pub self_attested_attributes: HashMap<String, String>,
    pub requested_attrs: HashMap<String, RequestedAttribute>,
    pub requested_predicates: HashMap<String, ProvingCredentialKey>
}

impl JsonEncodable for RequestedCredentials {}

impl<'a> JsonDecodable<'a> for RequestedCredentials {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedAttribute {
    pub cred_id: String,
    pub timestamp: Option<u64>,
    pub revealed: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedProof {
    pub revealed_attrs: HashMap<String, RevealedAttributeInfo>,
    pub unrevealed_attrs: HashMap<String, String>,
    pub self_attested_attrs: HashMap<String, String>,
    pub predicates: HashMap<String, String>
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "attrNames")]
    pub attr_names: HashSet<String>
}

impl JsonEncodable for Schema {}

impl<'a> JsonDecodable<'a> for Schema {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialDefinitionConfig {
    pub support_revocation: bool
}

impl<'a> JsonDecodable<'a> for CredentialDefinitionConfig {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestedAttributeInfo {
    pub attr_referent: String,
    pub attr_info: AttributeInfo,
    pub revealed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestedPredicateInfo {
    pub predicate_referent: String,
    pub predicate_info: PredicateInfo
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone)]
pub struct ProvingCredentialKey {
    pub cred_id: String,
    pub timestamp: Option<u64>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RevocationInfo {
    pub witness: Witness,
    pub rev_reg: RevocationRegistry,
    pub timestamp: u64
}

impl JsonEncodable for RevocationInfo {}

impl<'a> JsonDecodable<'a> for RevocationInfo {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeValues {
    pub raw: String,
    pub encoded: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RevealedAttributeInfo {
    pub referent: String,
    pub raw: String,
    pub encoded: String
}