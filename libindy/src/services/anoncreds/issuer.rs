extern crate indy_crypto;
extern crate time;

use errors::anoncreds::AnoncredsError;
use errors::common::CommonError;
use services::anoncreds::types::*;
use services::anoncreds::helpers::*;
use std::collections::HashMap;
use self::indy_crypto::cl::*;
use self::indy_crypto::cl::issuer;

pub struct Issuer {}

impl Issuer {
    pub fn new() -> Issuer {
        Issuer {}
    }

    pub fn new_claim_definition(&self, issuer_did: &str, schema: &Schema, signature_type: Option<&str>,
                                create_non_revoc: bool) -> Result<(ClaimDefinition, IssuerPrivateKey), AnoncredsError> {
        info!(target: "services/anoncreds/issuer", "new_claim_definition >>> issuer_did: {:?}, schema: {:?}, signature_type: {:?}, create_non_revoc: {:?}",
              issuer_did, schema, signature_type, create_non_revoc);

        let signature_type = match signature_type {
            Some("CL") => SignatureTypes::CL,
            None => SignatureTypes::CL,
            _ => return Err(AnoncredsError::CommonError(CommonError::InvalidStructure(format!("Invalid Signature Type"))))
        };

        let claim_schema = build_claim_schema(&schema.data.attr_names)?;

        let (issuer_public_key, issuer_private_key) = issuer::Issuer::new_keys(&claim_schema, create_non_revoc)?;

        let claim_definition = ClaimDefinition {
            schema_seq_no: schema.seq_no,
            issuer_did: issuer_did.to_owned(),
            signature_type,
            data: issuer_public_key
        };

        info!(target: "services/anoncreds/issuer", "new_claim_definition <<< claim_definition: {:?}, issuer_private_key: {:?}",
              claim_definition, issuer_private_key);

        Ok((claim_definition, issuer_private_key))
    }

    pub fn new_revocation_registry(&self, pub_key: &IssuerPublicKey, max_claim_num: u32, issuer_did: &str, schema_seq_no: i32)
                                   -> Result<(RevocationRegistry, RevocationRegistryPrivate), AnoncredsError> {
        info!(target: "services/anoncreds/issuer", "new_revocation_registry >>> pub_key: {:?}, max_claim_num: {:?}, issuer_did: {:?}, schema_seq_no: {:?}",
              pub_key, max_claim_num, issuer_did, schema_seq_no);

        let (rev_reg_pub, rev_reg_priv) = issuer::Issuer::new_revocation_registry(pub_key, max_claim_num)?;

        let revocation_registry = RevocationRegistry {
            issuer_did: issuer_did.to_owned(),
            schema_seq_no,
            data: rev_reg_pub
        };

        info!(target: "services/anoncreds/issuer", "new_revocation_registry <<< revocation_registry: {:?}, revocation_registry_private: {:?}",
              revocation_registry, rev_reg_priv);

        // TODO: solve tails storing
        Ok((revocation_registry, rev_reg_priv))
    }

    pub fn new_claim(&self,
                     issuer_pub_key: &IssuerPublicKey,
                     issuer_priv_key: &IssuerPrivateKey,
                     rev_reg: Option<&mut RevocationRegistry>,
                     rev_reg_priv: Option<&RevocationRegistryPrivate>,
                     claim_request: &ClaimRequest,
                     claim_values: &HashMap<String, Vec<String>>,
                     rev_idx: Option<u32>) -> Result<ClaimSignature, AnoncredsError> {
        info!(target: "services/anoncreds/issuer", "new_claim >>> issuer_pub_key: {:?}, issuer_priv_key: {:?}, rev_reg: {:?}, rev_reg_priv: {:?},\
                       claim_request: {:?}, claim_values: {:?}, rev_idx: {:?}",
              issuer_pub_key, issuer_priv_key, rev_reg, rev_reg_priv, claim_request, claim_values, rev_idx);

        let claim_values = build_claim_values(&claim_values)?;
        let claim_signature = issuer::Issuer::sign_claim(&claim_request.prover_did,
                                                         &claim_request.blinded_ms,
                                                         &claim_values,
                                                         &issuer_pub_key,
                                                         &issuer_priv_key,
                                                         rev_idx,
                                                         rev_reg.map(|r| &mut r.data),
                                                         rev_reg_priv)?;

        info!(target: "services/anoncreds/issuer", "new_claim <<< claim_signature {:?}", claim_signature);

        Ok(claim_signature)
    }

    pub fn revoke(&self, rev_reg: &mut RevocationRegistry, rev_idx: u32) -> Result<i64, AnoncredsError> {
        info!(target: "services/anoncreds/issuer", "revoke >>> rev_reg: {:?}, rev_idx: {:?}", rev_reg, rev_idx);

        issuer::Issuer::revoke_claim(&mut rev_reg.data, rev_idx)?;

        let timestamp = time::now_utc().to_timespec().sec;

        info!(target: "services/anoncreds/issuer", "revoke <<< timestamp: {:?}", timestamp);

        Ok(timestamp)
    }
}