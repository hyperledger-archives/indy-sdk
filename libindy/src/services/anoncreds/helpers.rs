extern crate indy_crypto;

use errors::common::CommonError;

use domain::credential::AttributeValues;
use domain::proof_request::{AttributeInfo, PredicateInfo};

use self::indy_crypto::cl::{issuer, verifier, CredentialSchema, CredentialValues, SubProofRequest};


use std::collections::{HashSet, HashMap};

pub fn attr_common_view(attr: &str) -> String {
    attr.replace(" ", "").to_lowercase()
}

pub fn build_credential_schema(attrs: &HashSet<String>) -> Result<CredentialSchema, CommonError> {
    let mut credential_schema_builder = issuer::Issuer::new_credential_schema_builder()?;
    for attr in attrs {
        credential_schema_builder.add_attr(&attr_common_view(attr))?;
    }
    Ok(credential_schema_builder.finalize()?)
}

pub fn build_credential_values(credential_values: &HashMap<String, AttributeValues>) -> Result<CredentialValues, CommonError> {
    let mut credential_values_builder = issuer::Issuer::new_credential_values_builder()?;
    for (attr, values) in credential_values {
        credential_values_builder.add_value(&attr_common_view(attr), &values.encoded)?;
    }
    Ok(credential_values_builder.finalize()?)
}

pub fn build_sub_proof_request(attrs_for_credential: &Vec<AttributeInfo>, predicates_for_credential: &Vec<PredicateInfo>) -> Result<SubProofRequest, CommonError> {
    let mut sub_proof_request_builder = verifier::Verifier::new_sub_proof_request_builder()?;

    for attr in attrs_for_credential {
        sub_proof_request_builder.add_revealed_attr(&attr_common_view(&attr.name))?
    }

    for predicate in predicates_for_credential {
        sub_proof_request_builder.add_predicate(&attr_common_view(&predicate.name), "GE", predicate.p_value)?;
    }

    Ok(sub_proof_request_builder.finalize()?)
}

pub fn parse_cred_rev_id(cred_rev_id: &str) -> Result<u32, CommonError> {
    Ok(cred_rev_id.parse::<u32>()
        .map_err(|err| CommonError::InvalidStructure(format!("Cannot parse CredentialRevocationId: {}", err)))?)
}