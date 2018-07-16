extern crate indy_crypto;

use errors::common::CommonError;

use domain::anoncreds::credential::AttributeValues;
use domain::anoncreds::proof_request::{AttributeInfo, PredicateInfo};

use self::indy_crypto::cl::{issuer, verifier, CredentialSchema, NonCredentialSchema, MasterSecret, CredentialValues, SubProofRequest};


use std::collections::{HashSet, HashMap};

pub fn attr_common_view(attr: &str) -> String {
    attr.replace(" ", "").to_lowercase()
}

pub fn build_credential_schema(attrs: &HashSet<String>) -> Result<CredentialSchema, CommonError> {
    trace!("build_credential_schema >>> attrs: {:?}", attrs);

    let mut credential_schema_builder = issuer::Issuer::new_credential_schema_builder()?;
    for attr in attrs {
        credential_schema_builder.add_attr(&attr_common_view(attr))?;
    }

    let res = credential_schema_builder.finalize()?;

    trace!("build_credential_schema <<< res: {:?}", res);

    Ok(res)
}

pub fn build_non_credential_schema() -> Result<NonCredentialSchema, CommonError> {
    trace!("build_non_credential_schema");

    let mut non_credential_schema_builder = issuer::Issuer::new_non_credential_schema_builder()?;
    non_credential_schema_builder.add_attr("master_secret")?;
    let res = non_credential_schema_builder.finalize()?;

    trace!("build_non_credential_schema <<< res: {:?}", res);
    Ok(res)
}

pub fn build_credential_values(credential_values: &HashMap<String, AttributeValues>, master_secret: Option<&MasterSecret>) -> Result<CredentialValues, CommonError> {
    trace!("build_credential_values >>> credential_values: {:?}", credential_values);

    let mut credential_values_builder = issuer::Issuer::new_credential_values_builder()?;
    for (attr, values) in credential_values {
        credential_values_builder.add_dec_known(&attr_common_view(attr), &values.encoded)?;
    }
    if let Some(ms) = master_secret {
        credential_values_builder.add_value_hidden("master_secret", &ms.value()?)?;
    }

    let res = credential_values_builder.finalize()?;

    trace!("build_credential_values <<< res: {:?}", res);

    Ok(res)
}

pub fn build_sub_proof_request(attrs_for_credential: &Vec<AttributeInfo>,
                               predicates_for_credential: &Vec<PredicateInfo>) -> Result<SubProofRequest, CommonError> {
    trace!("build_sub_proof_request >>> attrs_for_credential: {:?}, predicates_for_credential: {:?}", attrs_for_credential, predicates_for_credential);

    let mut sub_proof_request_builder = verifier::Verifier::new_sub_proof_request_builder()?;

    for attr in attrs_for_credential {
        sub_proof_request_builder.add_revealed_attr(&attr_common_view(&attr.name))?
    }

    for predicate in predicates_for_credential {
        sub_proof_request_builder.add_predicate(&attr_common_view(&predicate.name), "GE", predicate.p_value)?;
    }

    let res = sub_proof_request_builder.finalize()?;

    trace!("build_sub_proof_request <<< res: {:?}", res);

    Ok(res)
}

pub fn parse_cred_rev_id(cred_rev_id: &str) -> Result<u32, CommonError> {
    trace!("parse_cred_rev_id >>> cred_rev_id: {:?}", cred_rev_id);

    let res = cred_rev_id.parse::<u32>()
        .map_err(|err| CommonError::InvalidStructure(format!("Cannot parse CredentialRevocationId: {}", err)))?;

    trace!("parse_cred_rev_id <<< res: {:?}", res);

    Ok(res)
}
