extern crate indy_crypto;

use std::collections::{HashMap};

use domain::anoncreds::credential_definition::{CredentialDefinitionV1, CredentialDefinition};
use domain::anoncreds::proof::{Proof, RequestedProof};
use domain::anoncreds::proof_request::{AttributeInfo, PredicateInfo, ProofRequest};
use domain::anoncreds::revocation_registry::RevocationRegistryV1;
use domain::anoncreds::revocation_registry_definition::RevocationRegistryDefinitionV1;
use domain::anoncreds::schema::{SchemaV1, Schema};
use errors::prelude::*;
use services::anoncreds::helpers::*;

use self::indy_crypto::cl::CredentialPublicKey;
use self::indy_crypto::cl::verifier::Verifier as CryptoVerifier;
use services::anoncreds::prover::Prover;
use services::wallet::language::{parse_from_json, Operator};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Filter {
    schema_id: String,
    schema_issuer_did: String,
    schema_name: String,
    schema_version: String,
    issuer_did: String,
    cred_def_id: String,
}

pub struct Verifier {}

impl Verifier {
    pub fn new() -> Verifier {
        Verifier {}
    }

    pub fn verify(&self,
                  full_proof: &Proof,
                  proof_req: &ProofRequest,
                  schemas: &HashMap<String, SchemaV1>,
                  cred_defs: &HashMap<String, CredentialDefinitionV1>,
                  rev_reg_defs: &HashMap<String, RevocationRegistryDefinitionV1>,
                  rev_regs: &HashMap<String, HashMap<u64, RevocationRegistryV1>>) -> IndyResult<bool> {
        trace!("verify >>> full_proof: {:?}, proof_req: {:?}, schemas: {:?}, cred_defs: {:?}, rev_reg_defs: {:?} rev_regs: {:?}",
               full_proof, proof_req, schemas, cred_defs, rev_reg_defs, rev_regs);

        let mut proof_verifier = CryptoVerifier::new_proof_verifier()?;
        let non_credential_schema = build_non_credential_schema()?;

        for sub_proof_index in 0..full_proof.identifiers.len() {
            let identifier = full_proof.identifiers[sub_proof_index].clone();

            let schema: &SchemaV1 = schemas.get(&identifier.schema_id)
                .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Schema not found for id: {:?}", identifier.schema_id)))?;

            let cred_def: &CredentialDefinitionV1 = cred_defs.get(&identifier.cred_def_id)
                .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("CredentialDefinition not found for id: {:?}", identifier.cred_def_id)))?;

            let (rev_reg_def, rev_reg) = if cred_def.value.revocation.is_some() {
                let timestamp = identifier.timestamp
                    .clone()
                    .ok_or(err_msg(IndyErrorKind::InvalidStructure, "Timestamp not found"))?;

                let rev_reg_id = identifier.rev_reg_id
                    .clone()
                    .ok_or(err_msg(IndyErrorKind::InvalidStructure, "Revocation Registry Id not found"))?;

                let rev_reg_def = Some(rev_reg_defs
                    .get(&rev_reg_id)
                    .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("RevocationRegistryDefinition not found for id: {:?}", identifier.rev_reg_id)))?);

                let rev_regs_for_cred = rev_regs
                    .get(&rev_reg_id)
                    .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("RevocationRegistry not found for id: {:?}", rev_reg_id)))?;

                let rev_reg = Some(rev_regs_for_cred
                    .get(&timestamp)
                    .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("RevocationRegistry not found for timestamp: {:?}", timestamp)))?);

                (rev_reg_def, rev_reg)
            } else { (None, None) };

            let attrs_for_credential = Verifier::_get_revealed_attributes_for_credential(sub_proof_index, &full_proof.requested_proof, proof_req)?;
            let predicates_for_credential = Verifier::_get_predicates_for_credential(sub_proof_index, &full_proof.requested_proof, proof_req)?;

            let credential_schema = build_credential_schema(&schema.attr_names)?;
            let sub_proof_request = build_sub_proof_request(&attrs_for_credential, &predicates_for_credential)?;

            let credential_pub_key = CredentialPublicKey::build_from_parts(&cred_def.value.primary, cred_def.value.revocation.as_ref())?;

            proof_verifier.add_sub_proof_request(&sub_proof_request,
                                                 &credential_schema,
                                                 &non_credential_schema,
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
                                               proof_req: &ProofRequest) -> IndyResult<Vec<AttributeInfo>> {
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
                                      proof_req: &ProofRequest) -> IndyResult<Vec<PredicateInfo>> {
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

    pub fn verify_requested_restrictions(&self,
                                         prover_service: &Prover,
                                         proof_req: &ProofRequest,
                                         full_proof: &Proof,
                                         schemas: &HashMap<String, SchemaV1>,
                                         cred_defs: &HashMap<String, CredentialDefinitionV1>) -> IndyResult<bool> {
        let proof_attr_indexes = full_proof.requested_proof.unrevealed_attrs
            .iter()
            .map(|(r, info)| (r.to_string(), info.sub_proof_index as usize))
            .collect::<HashMap<String, usize>>()
            .into_iter()
            .chain(
                full_proof.requested_proof.revealed_attrs
                    .iter()
                    .map(|(r, info)| (r.to_string(), info.sub_proof_index as usize))
                    .collect::<HashMap<String, usize>>()
            )
            .collect::<HashMap<String, usize>>();

        let predicate_indexes = full_proof.requested_proof.predicates
            .iter()
            .map(|(r, info)| (r.to_string(), info.sub_proof_index as usize))
            .collect::<HashMap<String, usize>>();

        let requested_attrs: HashMap<String, AttributeInfo> = proof_req.requested_attributes
            .iter()
            .filter(|&(referent, info)| !Verifier::_self_attested(&referent, &info, &full_proof) )
            .map(|(referent, info)| (referent.to_string(), info.clone()))
            .collect();

        for (referent, info) in requested_attrs {

            let op = parse_from_json(&prover_service
                .build_query(&info.name, &referent, &info.restrictions, &None)?
            )?;

            let filter = Verifier::_gather_filter_info(&referent, full_proof, &proof_attr_indexes, schemas, cred_defs)?;

            if !Verifier::_process_operator(&info.name, &op, &filter)? { return Ok(false) }
        }

        for (referent, info) in proof_req.requested_predicates.iter() {

            let op = parse_from_json(&prover_service
                .build_query(&info.name, &referent, &info.restrictions, &None)?
            )?;

            let filter = Verifier::_gather_filter_info(&referent, full_proof, &predicate_indexes, schemas, cred_defs)?;

            if !Verifier::_process_operator(&info.name, &op, &filter)? { return Ok(false) }
        }

        Ok(true)
    }

    fn _self_attested(referent: &str, info: &AttributeInfo, full_proof: &Proof) -> bool {
        match info.restrictions.as_ref() {
            Some(&serde_json::Value::Array(ref array)) if array.is_empty() =>
                full_proof.requested_proof.self_attested_attrs.contains_key(referent),
            None => full_proof.requested_proof.self_attested_attrs.contains_key(referent),
            Some(_) => false
        }
    }

    fn _gather_filter_info(referent: &str,
                           full_proof: &Proof,
                           indexes: &HashMap<String, usize>,
                           schemas: &HashMap<String, SchemaV1>,
                           cred_defs: &HashMap<String, CredentialDefinitionV1>) -> IndyResult<Filter> {

        let index = indexes
            .get(referent)
            .ok_or(err_msg(
                IndyErrorKind::InvalidState,
                format!("Referent '{}' not found for Proof attribute", referent))
            )?;

        let identifier = full_proof.identifiers
            .get(index.clone())
            .ok_or(err_msg(
                IndyErrorKind::InvalidState,
                format!("Identifier not found for referent: {}", referent))
            )?;

        let schema: &SchemaV1 = schemas
            .get(&identifier.schema_id)
            .ok_or(err_msg(
                IndyErrorKind::InvalidStructure,
                format!("Schema not found for id: {:?}", identifier.schema_id))
            )?;

        let cred_def: &CredentialDefinitionV1 = cred_defs
            .get(&identifier.cred_def_id)
            .ok_or(err_msg(
                IndyErrorKind::InvalidStructure,
                format!("CredentialDefinitionV1 not found for id: {:?}", identifier.cred_def_id))
            )?;

        let schema_issuer_did = Schema::issuer_did(&schema.id)
            .ok_or(err_msg(
                IndyErrorKind::InvalidStructure,
                format!("schema_id has invalid format: {:?}", schema.id))
            )?;

        let issuer_did = CredentialDefinition::issuer_did(&cred_def.id)
            .ok_or(err_msg(
                IndyErrorKind::InvalidStructure,
                format!("cred_def_id has invalid format: {:?}", cred_def.id))
            )?;

        Ok(Filter {
            schema_id: identifier.schema_id.to_string(),
            schema_name: schema.name.to_string(),
            schema_issuer_did: schema_issuer_did.to_string(),
            schema_version: schema.version.to_string(),
            cred_def_id: identifier.cred_def_id.to_string(),
            issuer_did: issuer_did.to_string()
        })
    }

    fn _process_operator(attr: &str,
                         restriction_op: &Operator,
                         filter: &Filter) -> IndyResult<bool> {
        let found = match restriction_op {
            Operator::Eq(ref tag_name, ref tag_value) =>
                Verifier::_process_filter(&tag_name.from_utf8()?, &tag_value.value(), filter)?,
            Operator::Neq(ref tag_name, ref tag_value) =>
                !Verifier::_process_filter(&tag_name.from_utf8()?, &tag_value.value(), filter)?,
            Operator::In(ref tag_name, ref tag_values) =>
                tag_values
                    .iter()
                    .map(|val|
                        Verifier::_process_filter(&tag_name.from_utf8()?, &val.value(), filter))
                    .collect::<IndyResult<Vec<bool>>>()?
                    .contains(&true),
            Operator::And(ref operators) =>
                operators
                    .iter()
                    .map(|op| Verifier::_process_operator(attr, op, filter))
                    .collect::<IndyResult<Vec<bool>>>()?
                    .iter()
                    .all(|a| a == &true),
            Operator::Or(ref operators) =>
                operators
                    .iter()
                    .map(|op| Verifier::_process_operator(attr, op, filter))
                    .collect::<IndyResult<Vec<bool>>>()?
                    .contains(&true),
            Operator::Not(ref operator) => {
                !Verifier::_process_operator(attr, &*operator, filter)?
            },
            _ => false
        };
        Ok(found)
    }

    fn _process_filter(tag: &str,
                       tag_value: &str,
                       filter: &Filter) -> IndyResult<bool> {

        let found = match tag {
            "schema_id" => &filter.schema_id == tag_value,
            "schema_issuer_did" => &filter.schema_issuer_did == tag_value,
            "schema_name" => &filter.schema_name == tag_value,
            "schema_version" => &filter.schema_version == tag_value,
            "cred_def_id" => &filter.cred_def_id == tag_value,
            "issuer_did" => &filter.issuer_did == tag_value,
            x if Verifier::_is_attr_operator(x) => true,
            _ => return Err(err_msg(IndyErrorKind::InvalidStructure, "Unknown Filter Type"))
        };
        Ok(found)
    }

    fn _is_attr_operator(key: &str) -> bool { key.starts_with("attr::") && key.ends_with("::marker") }

}

#[cfg(test)]
mod tests {
    use super::*;
    use services::wallet::language::{TagName, TargetValue};

    pub const SCHEMA_ID: &'static str = "123";
    pub const SCHEMA_NAME: &'static str = "Schema Name";
    pub const SCHEMA_ISSUER_DID: &'static str = "234";
    pub const SCHEMA_VERSION: &'static str = "1.2.3";
    pub const CRED_DEF_ID: &'static str = "345";
    pub const ISSUER_DID: &'static str = "456";

    fn encrypted_tag(tag: String) -> TagName { TagName::EncryptedTagName(tag.into_bytes()) }

    fn unencrypted_target(tag: String) -> TargetValue { TargetValue::Unencrypted(tag) }

    fn schema_id_tag() -> TagName { encrypted_tag("schema_id".to_string()) }

    fn schema_name_tag() -> TagName { encrypted_tag("schema_name".to_string()) }

    fn schema_issuer_did_tag() -> TagName { encrypted_tag("schema_issuer_did".to_string()) }

    fn schema_version_tag() -> TagName { encrypted_tag("schema_version".to_string()) }

    fn cred_def_id_tag() -> TagName { encrypted_tag("cred_def_id".to_string()) }

    fn issuer_did_tag() -> TagName { encrypted_tag("issuer_did".to_string()) }

    fn attr_tag() -> TagName { encrypted_tag("attr::zip::marker".to_string()) }

    fn bad_attr_tag() -> TagName { encrypted_tag("bad::zip::marker".to_string()) }

    fn filter() -> Filter {
        Filter {
            schema_id: SCHEMA_ID.to_string(),
            schema_name: SCHEMA_NAME.to_string(),
            schema_issuer_did: SCHEMA_ISSUER_DID.to_string(),
            schema_version: SCHEMA_VERSION.to_string(),
            cred_def_id: CRED_DEF_ID.to_string(),
            issuer_did: ISSUER_DID.to_string(),
        }
    }

    #[test]
    fn test_process_op_eq() {
        let filter = filter();

        let mut op = Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string()));
        assert_eq!(true, Verifier::_process_operator("zip", &op, &filter).unwrap());

        op = Operator::And(vec![
            Operator::Eq(attr_tag(), unencrypted_target("1".to_string())),
            Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string())),
        ]);
        assert_eq!(true, Verifier::_process_operator("zip", &op, &filter).unwrap());

        op = Operator::And(vec![
            Operator::Eq(bad_attr_tag(), unencrypted_target("1".to_string())),
            Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string())),
        ]);
        assert!(Verifier::_process_operator("zip", &op, &filter).is_err());

        op = Operator::Eq(schema_id_tag(), unencrypted_target("NOT HERE".to_string()));
        assert_eq!(false, Verifier::_process_operator("zip", &op, &filter).unwrap());
    }

    #[test]
    fn test_process_op_ne() {
        let filter = filter();
        let mut op = Operator::Neq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string()));
        assert_eq!(false, Verifier::_process_operator("zip", &op, &filter).unwrap());

        op = Operator::Neq(schema_id_tag(), unencrypted_target("NOT HERE".to_string()));
        assert_eq!(true, Verifier::_process_operator("zip", &op, &filter).unwrap());
    }

    #[test]
    fn test_process_op_in() {
        let filter = filter();
        let mut cred_def_ids = vec![unencrypted_target("Not Here".to_string())];

        let mut op = Operator::In(cred_def_id_tag(), cred_def_ids.clone());
        assert_eq!(false, Verifier::_process_operator("zip", &op, &filter).unwrap());

        cred_def_ids.push(unencrypted_target(CRED_DEF_ID.to_string()));
        op = Operator::In(cred_def_id_tag(), cred_def_ids.clone());
        assert_eq!(true, Verifier::_process_operator("zip", &op, &filter).unwrap());
    }

    #[test]
    fn test_process_op_or() {
        let filter = filter();
        let mut op = Operator::Or(vec![
            Operator::Eq(schema_id_tag(), unencrypted_target("Not Here".to_string())),
            Operator::Eq(cred_def_id_tag(), unencrypted_target("Not Here".to_string()))
        ]);
        assert_eq!(false, Verifier::_process_operator("zip", &op, &filter).unwrap());

        op = Operator::Or(vec![
            Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string())),
            Operator::Eq(cred_def_id_tag(), unencrypted_target("Not Here".to_string()))
        ]);
        assert_eq!(true, Verifier::_process_operator("zip", &op, &filter).unwrap());
    }

    #[test]
    fn test_process_op_and() {
        let filter = filter();
        let mut op = Operator::And(vec![
            Operator::Eq(schema_id_tag(), unencrypted_target("Not Here".to_string())),
            Operator::Eq(cred_def_id_tag(), unencrypted_target("Not Here".to_string()))
        ]);
        assert_eq!(false, Verifier::_process_operator("zip", &op, &filter).unwrap());

        op = Operator::And(vec![
            Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string())),
            Operator::Eq(cred_def_id_tag(), unencrypted_target("Not Here".to_string()))
        ]);
        assert_eq!(false, Verifier::_process_operator("zip", &op, &filter).unwrap());

        op = Operator::And(vec![
            Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string())),
            Operator::Eq(cred_def_id_tag(), unencrypted_target(CRED_DEF_ID.to_string()))
        ]);
        assert_eq!(true, Verifier::_process_operator("zip", &op, &filter).unwrap());
    }

    #[test]
    fn test_process_op_not() {
        let filter = filter();
        let mut op = Operator::Not(Box::new(Operator::And(vec![
            Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string())),
            Operator::Eq(cred_def_id_tag(), unencrypted_target(CRED_DEF_ID.to_string()))
        ])));
        assert_eq!(false, Verifier::_process_operator("zip", &op, &filter).unwrap());

        op = Operator::Not(Box::new(Operator::And(vec![
            Operator::Eq(schema_id_tag(), unencrypted_target("Not Here".to_string())),
            Operator::Eq(cred_def_id_tag(), unencrypted_target("Not Here".to_string()))
        ])));
        assert_eq!(true, Verifier::_process_operator("zip", &op, &filter).unwrap());
    }

    #[test]
    fn test_proccess_op_or_with_nested_and() {
        let filter = filter();
        let mut op = Operator::Or(vec![
            Operator::And(vec![
                Operator::Eq(schema_id_tag(), unencrypted_target("Not Here".to_string())),
                Operator::Eq(cred_def_id_tag(), unencrypted_target("Not Here".to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_issuer_did_tag(), unencrypted_target("Not Here".to_string())),
                Operator::Eq(schema_name_tag(), unencrypted_target("Not Here".to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_name_tag(), unencrypted_target("Not Here".to_string())),
                Operator::Eq(issuer_did_tag(), unencrypted_target("Not Here".to_string()))
            ]),
        ]);
        assert_eq!(false, Verifier::_process_operator("zip", &op, &filter).unwrap());

        op = Operator::Or(vec![
            Operator::And(vec![
                Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string())),
                Operator::Eq(cred_def_id_tag(), unencrypted_target("Not Here".to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_issuer_did_tag(), unencrypted_target("Not Here".to_string())),
                Operator::Eq(schema_name_tag(), unencrypted_target("Not Here".to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_name_tag(), unencrypted_target("Not Here".to_string())),
                Operator::Eq(issuer_did_tag(), unencrypted_target("Not Here".to_string()))
            ]),
        ]);
        assert_eq!(false, Verifier::_process_operator("zip", &op, &filter).unwrap());

        op = Operator::Or(vec![
            Operator::And(vec![
                Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string())),
                Operator::Eq(cred_def_id_tag(), unencrypted_target(CRED_DEF_ID.to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_issuer_did_tag(), unencrypted_target("Not Here".to_string())),
                Operator::Eq(schema_name_tag(), unencrypted_target("Not Here".to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_name_tag(), unencrypted_target("Not Here".to_string())),
                Operator::Eq(issuer_did_tag(), unencrypted_target("Not Here".to_string()))
            ]),
        ]);
        assert_eq!(true, Verifier::_process_operator("zip", &op, &filter).unwrap());
    }

    #[test]
    fn test_verify_op_complex_nested() {
        let filter = filter();
        let mut op = Operator::And(vec![
            Operator::And(vec![
                Operator::Or(vec![
                    Operator::Eq(schema_name_tag(), unencrypted_target("Not Here".to_string())),
                    Operator::Eq(issuer_did_tag(), unencrypted_target("Not Here".to_string()))
                ]),
                Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string())),
                Operator::Eq(cred_def_id_tag(), unencrypted_target(CRED_DEF_ID.to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_issuer_did_tag(), unencrypted_target(SCHEMA_ISSUER_DID.to_string())),
                Operator::Eq(schema_name_tag(), unencrypted_target(SCHEMA_NAME.to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_version_tag(), unencrypted_target(SCHEMA_VERSION.to_string())),
                Operator::Eq(issuer_did_tag(), unencrypted_target(ISSUER_DID.to_string()))
            ]),
        ]);
        assert_eq!(false, Verifier::_process_operator("zip", &op, &filter).unwrap());

        op = Operator::And(vec![
            Operator::And(vec![
                Operator::Or(vec![
                    Operator::Eq(schema_name_tag(), unencrypted_target(SCHEMA_NAME.to_string())),
                    Operator::Eq(issuer_did_tag(), unencrypted_target("Not Here".to_string()))
                ]),
                Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string())),
                Operator::Eq(cred_def_id_tag(), unencrypted_target(CRED_DEF_ID.to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_issuer_did_tag(), unencrypted_target(SCHEMA_ISSUER_DID.to_string())),
                Operator::Eq(schema_name_tag(), unencrypted_target(SCHEMA_NAME.to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_version_tag(), unencrypted_target(SCHEMA_VERSION.to_string())),
                Operator::Eq(issuer_did_tag(), unencrypted_target(ISSUER_DID.to_string()))
            ]),
            Operator::Not(Box::new(Operator::Eq(schema_version_tag(), unencrypted_target("NOT HERE".to_string()))))
        ]);
        assert_eq!(true, Verifier::_process_operator("zip", &op, &filter).unwrap());

        op = Operator::And(vec![
            Operator::And(vec![
                Operator::Or(vec![
                    Operator::Eq(schema_name_tag(), unencrypted_target(SCHEMA_NAME.to_string())),
                    Operator::Eq(issuer_did_tag(), unencrypted_target("Not Here".to_string()))
                ]),
                Operator::Eq(schema_id_tag(), unencrypted_target(SCHEMA_ID.to_string())),
                Operator::Eq(cred_def_id_tag(), unencrypted_target(CRED_DEF_ID.to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_issuer_did_tag(), unencrypted_target(SCHEMA_ISSUER_DID.to_string())),
                Operator::Eq(schema_name_tag(), unencrypted_target(SCHEMA_NAME.to_string()))
            ]),
            Operator::And(vec![
                Operator::Eq(schema_version_tag(), unencrypted_target(SCHEMA_VERSION.to_string())),
                Operator::Eq(issuer_did_tag(), unencrypted_target(ISSUER_DID.to_string()))
            ]),
            Operator::Not(Box::new(Operator::Eq(schema_version_tag(), unencrypted_target(SCHEMA_VERSION.to_string()))))
        ]);
        assert_eq!(false, Verifier::_process_operator("zip", &op, &filter).unwrap());
    }
}
