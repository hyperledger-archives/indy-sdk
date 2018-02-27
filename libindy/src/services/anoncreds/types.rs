extern crate indy_crypto;

use errors::common::CommonError;
use std::collections::{HashMap, HashSet};

use self::indy_crypto::cl::*;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttributeInfo {
    pub name: String,
    pub restrictions: Option<Vec<Filter>>
}

impl JsonEncodable for AttributeInfo {}

impl<'a> JsonDecodable<'a> for AttributeInfo {}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct Filter {
    pub issuer_did: Option<String>,
    pub schema_key: Option<SchemaKeyFilter>
}

impl<'a> JsonDecodable<'a> for Filter {}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct SchemaKeyFilter {
    pub name: Option<String>,
    pub version: Option<String>,
    pub did: Option<String>
}

impl<'a> JsonDecodable<'a> for SchemaKeyFilter {}

pub trait Filtering {
    fn issuer_did(&self) -> String;
    fn schema_key(&self) -> SchemaKey;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialOffer {
    pub issuer_did: String,
    pub schema_key: SchemaKey,
    pub key_correctness_proof: CredentialKeyCorrectnessProof,
    pub nonce: Nonce
}

impl JsonEncodable for CredentialOffer {}

impl<'a> JsonDecodable<'a> for CredentialOffer {}

impl Filtering for CredentialOffer {
    fn issuer_did(&self) -> String { self.issuer_did.clone() }
    fn schema_key(&self) -> SchemaKey { self.schema_key.clone() }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct CredentialInfo {
    pub referent: String,
    pub attrs: HashMap<String, String>,
    pub schema_key: SchemaKey,
    pub issuer_did: String,
    pub revoc_reg_seq_no: Option<i32>
}

impl Filtering for CredentialInfo {
    fn issuer_did(&self) -> String { self.issuer_did.clone() }
    fn schema_key(&self) -> SchemaKey { self.schema_key.clone() }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialRequest {
    pub prover_did: String,
    pub issuer_did: String,
    pub schema_key: SchemaKey,
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

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct CredentialDefinition {
    #[serde(rename = "ref")]
    pub schema_seq_no: i32,
    #[serde(rename = "origin")]
    pub issuer_did: String,
    pub signature_type: SignatureTypes,
    pub data: CredentialDefinitionData
}

impl CredentialDefinition {
    pub fn clone(&self) -> Result<CredentialDefinition, CommonError> {
        Ok(CredentialDefinition {
            schema_seq_no: self.schema_seq_no,
            issuer_did: self.issuer_did.clone(),
            signature_type: self.signature_type.clone(),
            data: self.data.clone()?,
        })
    }
}

impl JsonEncodable for CredentialDefinition {}

impl<'a> JsonDecodable<'a> for CredentialDefinition {}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct CredentialDefinitionData {
    pub primary: CredentialPrimaryPublicKey,
    pub revocation: Option<CredentialRevocationPublicKey>
}

impl CredentialDefinitionData {
    pub fn clone(&self) -> Result<CredentialDefinitionData, CommonError> {
        Ok(CredentialDefinitionData {
            primary: self.primary.clone()?,
            revocation: self.revocation.clone()
        })
    }
}

impl JsonEncodable for CredentialDefinitionData {}

impl<'a> JsonDecodable<'a> for CredentialDefinitionData {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Credential {
    pub values: HashMap<String, Vec<String>>,
    pub schema_key: SchemaKey,
    pub signature: CredentialSignature,
    pub signature_correctness_proof: SignatureCorrectnessProof,
    pub issuer_did: String,
    pub rev_reg_seq_no: Option<i32>,
}

impl JsonEncodable for Credential {}

impl<'a> JsonDecodable<'a> for Credential {}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PredicateInfo {
    pub attr_name: String,
    pub p_type: String,
    pub value: i32,
    pub restrictions: Option<Vec<Filter>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialsForProofRequest {
    pub attrs: HashMap<String, Vec<CredentialInfo>>,
    pub predicates: HashMap<String, Vec<CredentialInfo>>
}

impl JsonEncodable for CredentialsForProofRequest {}

impl<'a> JsonDecodable<'a> for CredentialsForProofRequest {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofRequest {
    pub nonce: Nonce,
    pub name: String,
    pub version: String,
    pub requested_attrs: HashMap<String, AttributeInfo>,
    pub requested_predicates: HashMap<String, PredicateInfo>
}

impl JsonEncodable for ProofRequest {}

impl<'a> JsonDecodable<'a> for ProofRequest {}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub issuer_did: String,
    pub schema_key: SchemaKey,
    pub rev_reg_seq_no: Option<i32>
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
pub enum IssuanceTypes {
    ISSUANCE_BY_DEFAULT,
    ISSUANCE_ON_DEMAND
}

impl IssuanceTypes {
    pub fn to_bool(&self) -> bool {
        self.clone() == IssuanceTypes::ISSUANCE_BY_DEFAULT
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RevocationRegistryDefinitionPublicKeys {
    pub accum_key: RevocationKeyPublic
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RevocationRegistryDefinition {
    pub issuance_type: IssuanceTypes,
    pub max_cred_num: u32,
    pub public_keys: RevocationRegistryDefinitionPublicKeys,
    pub tails_hash: String,
    pub tails_location: String
}

impl JsonEncodable for RevocationRegistryDefinition {}

impl<'a> JsonDecodable<'a> for RevocationRegistryDefinition {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedCredentials {
    pub self_attested_attributes: HashMap<String, String>,
    pub requested_attrs: HashMap<String, (String, bool)>,
    pub requested_predicates: HashMap<String, String>
}

impl JsonEncodable for RequestedCredentials {}

impl<'a> JsonDecodable<'a> for RequestedCredentials {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedProof {
    pub revealed_attrs: HashMap<String, (String, String, String)>,
    pub unrevealed_attrs: HashMap<String, String>,
    pub self_attested_attrs: HashMap<String, String>,
    pub predicates: HashMap<String, String>
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "seqNo")]
    pub seq_no: i32,
    pub dest: String,
    pub data: SchemaData
}

impl Schema {
    pub fn schema_key(&self) -> SchemaKey {
        SchemaKey { name: self.data.name.clone(), version: self.data.version.clone(), did: self.dest.clone() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaData {
    pub name: String,
    pub version: String,
    pub attr_names: HashSet<String>
}

impl JsonEncodable for Schema {}

impl<'a> JsonDecodable<'a> for Schema {}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SchemaKey {
    pub name: String,
    pub version: String,
    pub did: String
}

impl JsonEncodable for SchemaKey {}

impl<'a> JsonDecodable<'a> for SchemaKey {}

