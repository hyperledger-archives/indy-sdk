extern crate indy_crypto;
extern crate time;

use errors::anoncreds::AnoncredsError;
use errors::common::CommonError;
use services::anoncreds::types::*;
use services::anoncreds::helpers::*;
use std::collections::HashMap;
use self::indy_crypto::cl::*;
use self::indy_crypto::cl::issuer::Issuer as CryptoIssuer;

pub struct Issuer {}

impl Issuer {
    pub fn new() -> Issuer {
        Issuer {}
    }

    pub fn new_claim_definition(&self, issuer_did: &str, schema: &Schema, signature_type: Option<&str>,
                                create_non_revoc: bool) -> Result<(ClaimDefinition, IssuerPrivateKey, KeyCorrectnessProof), AnoncredsError> {
        info!("new_claim_definition >>> issuer_did: {:?}, schema: {:?}, signature_type: {:?}, create_non_revoc: {:?}",
              issuer_did, schema, signature_type, create_non_revoc);

        let signature_type = match signature_type {
            Some("CL") => SignatureTypes::CL,
            None => SignatureTypes::CL,
            _ => return Err(AnoncredsError::CommonError(CommonError::InvalidStructure(format!("Invalid Signature Type"))))
        };

        let claim_schema = build_claim_schema(&schema.data.attr_names)?;

        let (issuer_public_key, issuer_private_key, issuer_key_correctness_proof) =
            CryptoIssuer::new_keys(&claim_schema, create_non_revoc)?;

        let claim_definition_data = ClaimDefinitionData {
            primary: issuer_public_key.get_primary_key()?.clone()?,
            revocation: issuer_public_key.get_revocation_key()?.clone()
        };

        let claim_definition = ClaimDefinition {
            schema_seq_no: schema.seq_no,
            issuer_did: issuer_did.to_owned(),
            signature_type,
            data: claim_definition_data
        };

        info!("new_claim_definition <<< claim_definition: {:?}, issuer_private_key: {:?}, issuer_key_correctness_proof: {:?}",
              claim_definition, issuer_private_key, issuer_key_correctness_proof);

        Ok((claim_definition, issuer_private_key, issuer_key_correctness_proof))
    }

    pub fn new_revocation_registry(&self, claim_def_data: &ClaimDefinitionData, max_claim_num: u32, issuer_did: &str, schema_seq_no: i32)
                                   -> Result<(RevocationRegistry, RevocationRegistryPrivate), AnoncredsError> {
        info!("new_revocation_registry >>> pub_key: {:?}, max_claim_num: {:?}, issuer_did: {:?}, schema_seq_no: {:?}",
              claim_def_data, max_claim_num, issuer_did, schema_seq_no);

        let issuer_pub_key = IssuerPublicKey::build_from_parts(&claim_def_data.primary, claim_def_data.revocation.as_ref())?;
        let (rev_reg_pub, rev_reg_priv) = CryptoIssuer::new_revocation_registry(&issuer_pub_key, max_claim_num)?;

        let revocation_registry = RevocationRegistry {
            issuer_did: issuer_did.to_owned(),
            schema_seq_no,
            data: rev_reg_pub
        };

        info!("new_revocation_registry <<< revocation_registry: {:?}, revocation_registry_private: {:?}",
              revocation_registry, rev_reg_priv);

        // TODO: solve tails storing
        Ok((revocation_registry, rev_reg_priv))
    }

    pub fn new_claim(&self,
                     claim_def_data: &ClaimDefinitionData,
                     issuer_priv_key: &IssuerPrivateKey,
                     rev_reg: Option<&mut RevocationRegistry>,
                     rev_reg_priv: Option<&RevocationRegistryPrivate>,
                     nonce: &Nonce,
                     claim_request: &ClaimRequest,
                     claim_values: &HashMap<String, Vec<String>>,
                     rev_idx: Option<u32>) -> Result<(ClaimSignature, SignatureCorrectnessProof), AnoncredsError> {
        info!("new_claim >>> claim_def_data: {:?}, issuer_priv_key: {:?}, rev_reg: {:?}, rev_reg_priv: {:?},\
                       nonce: {:?}, claim_request: {:?}, claim_values: {:?}, rev_idx: {:?}",
              claim_def_data, issuer_priv_key, rev_reg, rev_reg_priv, nonce, claim_request, claim_values, rev_idx);

        let claim_values = build_claim_values(&claim_values)?;
        let issuer_pub_key = IssuerPublicKey::build_from_parts(&claim_def_data.primary, claim_def_data.revocation.as_ref())?;

        let (claim_signature, signature_correctness_proof) = CryptoIssuer::sign_claim(&claim_request.prover_did,
                                                                                      &claim_request.blinded_ms,
                                                                                      &claim_request.blinded_ms_correctness_proof,
                                                                                      nonce,
                                                                                      &claim_request.nonce,
                                                                                      &claim_values,
                                                                                      &issuer_pub_key,
                                                                                      &issuer_priv_key,
                                                                                      rev_idx,
                                                                                      rev_reg.map(|r| &mut r.data),
                                                                                      rev_reg_priv)?;

        info!("new_claim <<< claim_signature {:?}, signature_correctness_proof {:?}", claim_signature, signature_correctness_proof);

        Ok((claim_signature, signature_correctness_proof))
    }

    pub fn revoke(&self, rev_reg: &mut RevocationRegistry, rev_idx: u32) -> Result<i64, AnoncredsError> {
        info!("revoke >>> rev_reg: {:?}, rev_idx: {:?}", rev_reg, rev_idx);

        CryptoIssuer::revoke_claim(&mut rev_reg.data, rev_idx)?;

        let timestamp = time::now_utc().to_timespec().sec;

        info!("revoke <<< timestamp: {:?}", timestamp);

        Ok(timestamp)
    }
}