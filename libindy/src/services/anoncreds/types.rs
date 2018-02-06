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

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimOffer {
    pub issuer_did: String,
    pub schema_key: SchemaKey,
    pub key_correctness_proof: KeyCorrectnessProof,
    pub nonce: Nonce
}

impl JsonEncodable for ClaimOffer {}

impl<'a> JsonDecodable<'a> for ClaimOffer {}

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

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ClaimInfo {
    pub referent: String,
    pub attrs: HashMap<String, String>,
    pub schema_key: SchemaKey,
    pub issuer_did: String,
    pub revoc_reg_seq_no: Option<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimRequest {
    pub prover_did: String,
    pub issuer_did: String,
    pub schema_key: SchemaKey,
    pub blinded_ms: BlindedMasterSecret,
    pub blinded_ms_correctness_proof: BlindedMasterSecretProofCorrectness,
    pub nonce: Nonce,
}

impl JsonEncodable for ClaimRequest {}

impl<'a> JsonDecodable<'a> for ClaimRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimRequestMetadata {
    pub master_secret_blinding_data: MasterSecretBlindingData,
    pub nonce: Nonce,
    pub master_secret_name: String
}

impl JsonEncodable for ClaimRequestMetadata {}

impl<'a> JsonDecodable<'a> for ClaimRequestMetadata {}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub enum SignatureTypes {
    CL
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct ClaimDefinition {
    #[serde(rename = "ref")]
    pub schema_seq_no: i32,
    #[serde(rename = "origin")]
    pub issuer_did: String,
    pub signature_type: SignatureTypes,
    pub data: ClaimDefinitionData
}

impl ClaimDefinition {
    pub fn clone(&self) -> Result<ClaimDefinition, CommonError> {
        Ok(ClaimDefinition {
            schema_seq_no: self.schema_seq_no,
            issuer_did: self.issuer_did.clone(),
            signature_type: self.signature_type.clone(),
            data: self.data.clone()?,
        })
    }
}

impl JsonEncodable for ClaimDefinition {}

impl<'a> JsonDecodable<'a> for ClaimDefinition {}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct ClaimDefinitionData {
    pub primary: IssuerPrimaryPublicKey,
    pub revocation: Option<IssuerRevocationPublicKey>
}

impl ClaimDefinitionData {
    pub fn clone(&self) -> Result<ClaimDefinitionData, CommonError> {
        Ok(ClaimDefinitionData {
            primary: self.primary.clone()?,
            revocation: self.revocation.clone()
        })
    }
}

impl JsonEncodable for ClaimDefinitionData {}

impl<'a> JsonDecodable<'a> for ClaimDefinitionData {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claim {
    pub values: HashMap<String, Vec<String>>,
    pub schema_key: SchemaKey,
    pub signature: ClaimSignature,
    pub signature_correctness_proof: SignatureCorrectnessProof,
    pub issuer_did: String,
    pub rev_reg_seq_no: Option<i32>,
}

impl JsonEncodable for Claim {}

impl<'a> JsonDecodable<'a> for Claim {}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PredicateInfo {
    pub attr_name: String,
    pub p_type: String,
    pub value: i32,
    pub restrictions: Option<Vec<Filter>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimsForProofRequest {
    pub attrs: HashMap<String, Vec<ClaimInfo>>,
    pub predicates: HashMap<String, Vec<ClaimInfo>>
}

impl JsonEncodable for ClaimsForProofRequest {}

impl<'a> JsonDecodable<'a> for ClaimsForProofRequest {}

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

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RevocationRegistry {
    pub issuer_did: String,
    pub schema_seq_no: i32,
    pub data: RevocationRegistryPublic
}

impl JsonEncodable for RevocationRegistry {}

impl<'a> JsonDecodable<'a> for RevocationRegistry {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedClaims {
    pub self_attested_attributes: HashMap<String, String>,
    pub requested_attrs: HashMap<String, (String, bool)>,
    pub requested_predicates: HashMap<String, String>
}

impl JsonEncodable for RequestedClaims {}

impl<'a> JsonDecodable<'a> for RequestedClaims {}

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

