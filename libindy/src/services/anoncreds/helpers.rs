extern crate indy_crypto;

use errors::common::CommonError;

use services::anoncreds::types::PredicateInfo;
use self::indy_crypto::cl::{issuer, verifier, CredentialSchema, CredentialValues, SubProofRequest};

use std::collections::{HashSet, HashMap};

pub fn build_credential_schema(attrs: &HashSet<String>) -> Result<CredentialSchema, CommonError> {
    let mut credential_schema_builder = issuer::Issuer::new_credential_schema_builder()?;
    for attr in attrs {
        credential_schema_builder.add_attr(&attr)?;
    }
    Ok(credential_schema_builder.finalize()?)
}

pub fn build_credential_values(credential_values: &HashMap<String, Vec<String>>) -> Result<CredentialValues, CommonError> {
    let mut credential_values_builder = issuer::Issuer::new_credential_values_builder()?;
    for (attr, values) in credential_values {
        let dec_val = values.get(1)
            .ok_or(CommonError::InvalidStructure(format!("Encoded value not found")))?;

        credential_values_builder.add_value(&attr, &dec_val)?;
    }
    Ok(credential_values_builder.finalize()?)
}

pub fn build_sub_proof_request(attrs_for_claim: &Vec<String>, predicates_for_claim: &Vec<PredicateInfo>) -> Result<SubProofRequest, CommonError> {
    let mut sub_proof_request_builder = verifier::Verifier::new_sub_proof_request_builder()?;

    for attr in attrs_for_claim {
        sub_proof_request_builder.add_revealed_attr(&attr)?
    }

    for predicate in predicates_for_claim {
        sub_proof_request_builder.add_predicate(&predicate.attr_name, "GE", predicate.value)?;
    }

    Ok(sub_proof_request_builder.finalize()?)
}