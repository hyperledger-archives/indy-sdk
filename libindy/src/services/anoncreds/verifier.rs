extern crate indy_crypto;

use services::anoncreds::types::*;
use std::collections::HashMap;
use errors::common::CommonError;
use self::indy_crypto::cl::{CredentialPublicKey, RevocationRegistry};
use self::indy_crypto::cl::verifier::Verifier as CryptoVerifier;
use services::anoncreds::helpers::*;

pub struct Verifier {}

impl Verifier {
    pub fn new() -> Verifier {
        Verifier {}
    }

    pub fn verify(&self,
                  full_proof: &FullProof,
                  proof_req: &ProofRequest,
                  credential_schemas: &HashMap<String, Schema>,
                  credential_defs: &HashMap<String, CredentialDefinition>,
                  rev_reg_defs: &HashMap<String, RevocationRegistryDefinition>,
                  rev_regs: &HashMap<String, HashMap<u64, RevocationRegistry>>) -> Result<bool, CommonError> {
        info!("verify >>> full_proof: {:?}, proof_req: {:?}, credential_schemas: {:?}, credential_defs: {:?}, rev_reg_defs: {:?} rev_regs: {:?}",
              full_proof, proof_req, credential_schemas, credential_defs, rev_reg_defs, rev_regs);

        let mut proof_verifier = CryptoVerifier::new_proof_verifier()?;

        for (referent, identifier) in full_proof.identifiers.iter() {
            let credential_schema = credential_schemas.get(referent)
                .ok_or(CommonError::InvalidStructure(format!("Schema not found")))?;
            let credential_def = credential_defs.get(referent)
                .ok_or(CommonError::InvalidStructure(format!("CredentialDefinition not found")))?;

            let (rev_reg_def, rev_reg) = if credential_def.value.revocation.is_some() {
                let timestamp = identifier.timestamp.ok_or(CommonError::InvalidStructure(format!("Timestamp not found")))?;
                let rev_reg_def = Some(rev_reg_defs.get(referent)
                    .ok_or(CommonError::InvalidStructure(format!("RevocationRegistryDefinition not found")))?);
                let rev_regs_for_cred = rev_regs.get(referent.as_str())
                    .ok_or(CommonError::InvalidStructure(format!("RevocationRegistry not found")))?;
                let rev_reg = Some(rev_regs_for_cred.get(&timestamp)
                    .ok_or(CommonError::InvalidStructure(format!("RevocationRegistry not found")))?);

                (rev_reg_def, rev_reg)
            } else { (None, None) };

            let attrs_for_credential = Verifier::_get_revealed_attributes_for_credential(referent, &full_proof.requested_proof, proof_req)?;
            let predicates_for_credential = Verifier::_get_predicates_for_credential(referent, &full_proof.requested_proof, proof_req)?;

            let credential_schema = build_credential_schema(&credential_schema.attr_names)?;
            let sub_proof_request = build_sub_proof_request(&attrs_for_credential, &predicates_for_credential)?;

            let credential_pub_key = CredentialPublicKey::build_from_parts(&credential_def.value.primary, credential_def.value.revocation.as_ref())?;

            proof_verifier.add_sub_proof_request(referent,
                                                 &sub_proof_request,
                                                 &credential_schema,
                                                 &credential_pub_key,
                                                 rev_reg_def.as_ref().map(|r_reg_def| &r_reg_def.value.public_keys.accum_key),
                                                 rev_reg)?;
        }

        let valid = proof_verifier.verify(&full_proof.proof, &proof_req.nonce)?;

        info!("verify <<< valid: {:?}", valid);

        Ok(valid)
    }

    fn _get_revealed_attributes_for_credential(referent: &str,
                                               requested_proof: &RequestedProof,
                                               proof_req: &ProofRequest) -> Result<Vec<AttributeInfo>, CommonError> {
        info!("_get_revealed_attributes_for_credential >>> referent: {:?}, requested_credentials: {:?}, proof_req: {:?}",
              referent, requested_proof, proof_req);

        let revealed_attrs_for_credential = requested_proof.revealed_attrs
            .iter()
            .filter(|&(attr_referent, ref revealed_attr_info)|
                referent.eq(&revealed_attr_info.referent) && proof_req.requested_attrs.contains_key(attr_referent))
            .map(|(attr_referent, _)|
                proof_req.requested_attrs[attr_referent].clone())
            .collect::<Vec<AttributeInfo>>();

        info!("_get_revealed_attributes_for_credential <<< revealed_attrs_for_credential: {:?}", revealed_attrs_for_credential);

        Ok(revealed_attrs_for_credential)
    }

    fn _get_predicates_for_credential(referent: &str,
                                      requested_proof: &RequestedProof,
                                      proof_req: &ProofRequest) -> Result<Vec<PredicateInfo>, CommonError> {
        info!("_get_predicates_for_credential >>> referent: {:?}, requested_credentials: {:?}, proof_req: {:?}",
              referent, requested_proof, proof_req);

        let predicates_for_credential = requested_proof.predicates
            .iter()
            .filter(|&(predicate_referent, requested_referent)|
                referent.eq(requested_referent) && proof_req.requested_predicates.contains_key(predicate_referent))
            .map(|(predicate_referent, requested_referent)|
                proof_req.requested_predicates[predicate_referent].clone())
            .collect::<Vec<PredicateInfo>>();

        info!("_get_predicates_for_credential <<< predicates_for_credential: {:?}", predicates_for_credential);

        Ok(predicates_for_credential)
    }
}