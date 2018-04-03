extern crate indy_crypto;

use std::collections::HashMap;
use errors::common::CommonError;
use self::indy_crypto::cl::{CredentialPublicKey};
use self::indy_crypto::cl::verifier::Verifier as CryptoVerifier;
use services::anoncreds::helpers::*;

use domain::schema::SchemaV1;
use domain::credential_definition::CredentialDefinitionV1 as CredentialDefinition;
use domain::revocation_registry_definition::RevocationRegistryDefinitionV1;
use domain::proof::{Proof, RequestedProof};
use domain::proof_request::{ProofRequest, AttributeInfo, PredicateInfo};
use domain::revocation_registry::RevocationRegistryV1;

pub struct Verifier {}

impl Verifier {
    pub fn new() -> Verifier {
        Verifier {}
    }

    pub fn verify(&self,
                  full_proof: &Proof,
                  proof_req: &ProofRequest,
                  schemas: &HashMap<String, SchemaV1>,
                  cred_defs: &HashMap<String, CredentialDefinition>,
                  rev_reg_defs: &HashMap<String, RevocationRegistryDefinitionV1>,
                  rev_regs: &HashMap<String, HashMap<u64, RevocationRegistryV1>>) -> Result<bool, CommonError> {
        trace!("verify >>> full_proof: {:?}, proof_req: {:?}, schemas: {:?}, cred_defs: {:?}, rev_reg_defs: {:?} rev_regs: {:?}",
               full_proof, proof_req, schemas, cred_defs, rev_reg_defs, rev_regs);

        let mut proof_verifier = CryptoVerifier::new_proof_verifier()?;

        for sub_proof_index in 0..full_proof.identifiers.len() {
            let identifier = full_proof.identifiers[sub_proof_index].clone();
            let schema: &SchemaV1 = schemas.get(&identifier.schema_id)
                .ok_or(CommonError::InvalidStructure(format!("Schema not found for id: {:?}", identifier.schema_id)))?;
            let cred_def: &CredentialDefinition = cred_defs.get(&identifier.cred_def_id)
                .ok_or(CommonError::InvalidStructure(format!("CredentialDefinition not found for id: {:?}", identifier.cred_def_id)))?;

            let (rev_reg_def, rev_reg) = if cred_def.value.revocation.is_some() {
                let timestamp = identifier.timestamp.clone().ok_or(CommonError::InvalidStructure(format!("Timestamp not found")))?;
                let rev_reg_id = identifier.rev_reg_id.clone().ok_or(CommonError::InvalidStructure(format!("Revocation Registry Id not found")))?;
                let rev_reg_def = Some(rev_reg_defs.get(&rev_reg_id)
                    .ok_or(CommonError::InvalidStructure(format!("RevocationRegistryDefinition not found for id: {:?}", identifier.rev_reg_id)))?);
                let rev_regs_for_cred = rev_regs.get(&rev_reg_id)
                    .ok_or(CommonError::InvalidStructure(format!("RevocationRegistry not found for id: {:?}", rev_reg_id)))?;
                let rev_reg = Some(rev_regs_for_cred.get(&timestamp)
                    .ok_or(CommonError::InvalidStructure(format!("RevocationRegistry not found for timestamp: {:?}", timestamp)))?);

                (rev_reg_def, rev_reg)
            } else { (None, None) };

            let attrs_for_credential = Verifier::_get_revealed_attributes_for_credential(sub_proof_index, &full_proof.requested_proof, proof_req)?;
            let predicates_for_credential = Verifier::_get_predicates_for_credential(sub_proof_index, &full_proof.requested_proof, proof_req)?;

            let credential_schema = build_credential_schema(&schema.attr_names)?;
            let sub_proof_request = build_sub_proof_request(&attrs_for_credential, &predicates_for_credential)?;

            let credential_pub_key = CredentialPublicKey::build_from_parts(&cred_def.value.primary, cred_def.value.revocation.as_ref())?;

            proof_verifier.add_sub_proof_request(&sub_proof_request,
                                                 &credential_schema,
                                                 &credential_pub_key,
                                                 rev_reg_def.as_ref().map(|r_reg_def| &r_reg_def.value.public_keys.accum_key),
                                                 rev_reg.as_ref().map(|r_reg| &r_reg.value))?;
        }

        let valid = proof_verifier.verify(&full_proof.proof, &proof_req.nonce)?;

        trace!("verify <<< valid: {:?}", valid);

        Ok(valid)
    }

    fn _get_revealed_attributes_for_credential(sub_proof_index: usize,
                                               requested_proof: &RequestedProof,
                                               proof_req: &ProofRequest) -> Result<Vec<AttributeInfo>, CommonError> {
        trace!("_get_revealed_attributes_for_credential >>> sub_proof_index: {:?}, requested_credentials: {:?}, proof_req: {:?}",
               sub_proof_index, requested_proof, proof_req);

        let revealed_attrs_for_credential = requested_proof.revealed_attrs
            .iter()
            .filter(|&(attr_referent, ref revealed_attr_info)|
                sub_proof_index == revealed_attr_info.sub_proof_index as usize && proof_req.requested_attributes.contains_key(attr_referent))
            .map(|(attr_referent, _)|
                proof_req.requested_attributes[attr_referent].clone())
            .collect::<Vec<AttributeInfo>>();

        trace!("_get_revealed_attributes_for_credential <<< revealed_attrs_for_credential: {:?}", revealed_attrs_for_credential);

        Ok(revealed_attrs_for_credential)
    }

    fn _get_predicates_for_credential(sub_proof_index: usize,
                                      requested_proof: &RequestedProof,
                                      proof_req: &ProofRequest) -> Result<Vec<PredicateInfo>, CommonError> {
        trace!("_get_predicates_for_credential >>> sub_proof_index: {:?}, requested_credentials: {:?}, proof_req: {:?}",
               sub_proof_index, requested_proof, proof_req);

        let predicates_for_credential = requested_proof.predicates
            .iter()
            .filter(|&(predicate_referent, requested_referent)|
                sub_proof_index == requested_referent.sub_proof_index as usize && proof_req.requested_predicates.contains_key(predicate_referent))
            .map(|(predicate_referent, _)|
                proof_req.requested_predicates[predicate_referent].clone())
            .collect::<Vec<PredicateInfo>>();

        trace!("_get_predicates_for_credential <<< predicates_for_credential: {:?}", predicates_for_credential);

        Ok(predicates_for_credential)
    }
}