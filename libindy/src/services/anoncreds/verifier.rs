use std::collections::{HashMap, HashSet};

use crate::domain::anoncreds::credential_definition::{CredentialDefinitionV1, CredentialDefinitionId};
use crate::domain::anoncreds::proof::{Proof, RequestedProof, Identifier, RevealedAttributeInfo};
use crate::domain::anoncreds::proof_request::{AttributeInfo, PredicateInfo, ProofRequestPayload, NonRevocedInterval};
use crate::domain::anoncreds::revocation_registry::RevocationRegistryV1;
use crate::domain::anoncreds::revocation_registry_definition::{RevocationRegistryDefinitionV1, RevocationRegistryId};
use crate::domain::anoncreds::schema::{SchemaV1, SchemaId};
use indy_api_types::errors::prelude::*;
use crate::services::anoncreds::helpers::*;

use ursa::bn::BigNumber;
use ursa::cl::{CredentialPublicKey, new_nonce, Nonce};
use ursa::cl::verifier::Verifier as CryptoVerifier;
use crate::utils::wql::Query;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Filter {
    schema_id: String,
    schema_issuer_did: String,
    schema_name: String,
    schema_version: String,
    issuer_did: String,
    cred_def_id: String,
}

lazy_static! {
    static ref INTERNAL_TAG_MATCHER: Regex = Regex::new("^attr::([^:]+)::(value|marker)$").unwrap();
}

pub struct Verifier {}

impl Verifier {
    pub fn new() -> Verifier {
        Verifier {}
    }

    pub fn verify(&self,
                  full_proof: &Proof,
                  proof_req: &ProofRequestPayload,
                  schemas: &HashMap<SchemaId, SchemaV1>,
                  cred_defs: &HashMap<CredentialDefinitionId, CredentialDefinitionV1>,
                  rev_reg_defs: &HashMap<RevocationRegistryId, RevocationRegistryDefinitionV1>,
                  rev_regs: &HashMap<RevocationRegistryId, HashMap<u64, RevocationRegistryV1>>) -> IndyResult<bool> {
        trace!("verify >>> full_proof: {:?}, proof_req: {:?}, schemas: {:?}, cred_defs: {:?}, rev_reg_defs: {:?} rev_regs: {:?}",
               full_proof, proof_req, schemas, cred_defs, rev_reg_defs, rev_regs);

        let received_revealed_attrs: HashMap<String, Identifier> = Verifier::_received_revealed_attrs(&full_proof)?;
        let received_unrevealed_attrs: HashMap<String, Identifier> = Verifier::_received_unrevealed_attrs(&full_proof)?;
        let received_predicates: HashMap<String, Identifier> = Verifier::_received_predicates(&full_proof)?;
        let received_self_attested_attrs: HashSet<String> = Verifier::_received_self_attested_attrs(&full_proof);

        Verifier::_compare_attr_from_proof_and_request(proof_req,
                                                       &received_revealed_attrs,
                                                       &received_unrevealed_attrs,
                                                       &received_self_attested_attrs,
                                                       &received_predicates)?;

        Verifier::_verify_revealed_attribute_values(&proof_req, &full_proof)?;

        Verifier::_verify_requested_restrictions(&proof_req,
                                                 &full_proof.requested_proof,
                                                 &received_revealed_attrs,
                                                 &received_unrevealed_attrs,
                                                 &received_predicates,
                                                 &received_self_attested_attrs)?;

        Verifier::_compare_timestamps_from_proof_and_request(proof_req,
                                                             &received_revealed_attrs,
                                                             &received_unrevealed_attrs,
                                                             &received_self_attested_attrs,
                                                             &received_predicates)?;

        let mut proof_verifier = CryptoVerifier::new_proof_verifier()?;
        let non_credential_schema = build_non_credential_schema()?;

        for sub_proof_index in 0..full_proof.identifiers.len() {
            let identifier = full_proof.identifiers[sub_proof_index].clone();

            let schema: &SchemaV1 = schemas.get(&identifier.schema_id)
                .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("Schema not found for id: {:?}", identifier.schema_id)))?;

            let cred_def: &CredentialDefinitionV1 = cred_defs.get(&identifier.cred_def_id)
                .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("CredentialDefinition not found for id: {:?}", identifier.cred_def_id)))?;

            let (rev_reg_def, rev_reg) =
                if let Some(timestamp) = identifier.timestamp {
                    let rev_reg_id = identifier.rev_reg_id
                        .clone()
                        .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Revocation Registry Id not found"))?;

                    let rev_reg_def = Some(rev_reg_defs
                        .get(&rev_reg_id)
                        .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("RevocationRegistryDefinition not found for id: {:?}", identifier.rev_reg_id)))?);

                    let rev_regs_for_cred = rev_regs
                        .get(&rev_reg_id)
                        .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("RevocationRegistry not found for id: {:?}", rev_reg_id)))?;

                    let rev_reg = Some(rev_regs_for_cred
                        .get(&timestamp)
                        .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("RevocationRegistry not found for timestamp: {:?}", timestamp)))?);

                    (rev_reg_def, rev_reg)
                } else { (None, None) };

            let attrs_for_credential = Verifier::_get_revealed_attributes_for_credential(sub_proof_index, &full_proof.requested_proof, proof_req)?;
            let predicates_for_credential = Verifier::_get_predicates_for_credential(sub_proof_index, &full_proof.requested_proof, proof_req)?;

            let credential_schema = build_credential_schema(&schema.attr_names.0)?;
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

    pub fn generate_nonce(&self) -> IndyResult<Nonce> {
        trace!("generate_nonce >>> ");

        let nonce = new_nonce()?;

        trace!("generate_nonce <<< nonce: {:?} ", nonce);

        Ok(nonce)
    }

    fn _get_revealed_attributes_for_credential(sub_proof_index: usize,
                                               requested_proof: &RequestedProof,
                                               proof_req: &ProofRequestPayload) -> IndyResult<Vec<AttributeInfo>> {
        trace!("_get_revealed_attributes_for_credential >>> sub_proof_index: {:?}, requested_credentials: {:?}, proof_req: {:?}",
               sub_proof_index, requested_proof, proof_req);

        let mut revealed_attrs_for_credential = requested_proof.revealed_attrs
            .iter()
            .filter(|&(attr_referent, ref revealed_attr_info)|
                sub_proof_index == revealed_attr_info.sub_proof_index as usize && proof_req.requested_attributes.contains_key(attr_referent))
            .map(|(attr_referent, _)|
                proof_req.requested_attributes[attr_referent].clone())
            .collect::<Vec<AttributeInfo>>();

        revealed_attrs_for_credential.append(
            &mut requested_proof.revealed_attr_groups
                .iter()
                .filter(|&(attr_referent, ref revealed_attr_info)|
                    sub_proof_index == revealed_attr_info.sub_proof_index as usize && proof_req.requested_attributes.contains_key(attr_referent))
                .map(|(attr_referent, _)|
                    proof_req.requested_attributes[attr_referent].clone())
                .collect::<Vec<AttributeInfo>>()
        );

        trace!("_get_revealed_attributes_for_credential <<< revealed_attrs_for_credential: {:?}", revealed_attrs_for_credential);

        Ok(revealed_attrs_for_credential)
    }

    fn _get_predicates_for_credential(sub_proof_index: usize,
                                      requested_proof: &RequestedProof,
                                      proof_req: &ProofRequestPayload) -> IndyResult<Vec<PredicateInfo>> {
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

    fn _compare_attr_from_proof_and_request(proof_req: &ProofRequestPayload,
                                            received_revealed_attrs: &HashMap<String, Identifier>,
                                            received_unrevealed_attrs: &HashMap<String, Identifier>,
                                            received_self_attested_attrs: &HashSet<String>,
                                            received_predicates: &HashMap<String, Identifier>) -> IndyResult<()> {
        let requested_attrs: HashSet<String> = proof_req.requested_attributes
            .keys()
            .cloned()
            .collect();

        let received_attrs: HashSet<String> = received_revealed_attrs
            .iter()
            .chain(received_unrevealed_attrs)
            .map(|(r, _)| r.to_string())
            .collect::<HashSet<String>>()
            .union(&received_self_attested_attrs)
            .cloned()
            .collect();

        if requested_attrs != received_attrs {
            return Err(err_msg(IndyErrorKind::InvalidStructure,
                               format!("Requested attributes {:?} do not correspond to received {:?}", requested_attrs, received_attrs)));
        }

        let requested_predicates: HashSet<&String> = proof_req.requested_predicates
            .keys()
            .collect();

        let received_predicates_: HashSet<&String> = received_predicates
            .keys()
            .collect();

        if requested_predicates != received_predicates_ {
            return Err(err_msg(IndyErrorKind::InvalidStructure,
                               format!("Requested predicates {:?} do not correspond to received {:?}", requested_predicates, received_predicates)));
        }

        Ok(())
    }

    fn _compare_timestamps_from_proof_and_request(proof_req: &ProofRequestPayload,
                                                  received_revealed_attrs: &HashMap<String, Identifier>,
                                                  received_unrevealed_attrs: &HashMap<String, Identifier>,
                                                  received_self_attested_attrs: &HashSet<String>,
                                                  received_predicates: &HashMap<String, Identifier>) -> IndyResult<()> {
        proof_req.requested_attributes
            .iter()
            .map(|(referent, info)|
                Verifier::_validate_timestamp(&received_revealed_attrs, referent, &proof_req.non_revoked, &info.non_revoked)
                    .or_else(|_| Verifier::_validate_timestamp(&received_unrevealed_attrs, referent, &proof_req.non_revoked, &info.non_revoked))
                    .or_else(|_| received_self_attested_attrs.get(referent).map(|_| ()).ok_or_else(|| IndyError::from(IndyErrorKind::InvalidStructure)))
            )
            .collect::<IndyResult<Vec<()>>>()?;

        proof_req.requested_predicates
            .iter()
            .map(|(referent, info)|
                Verifier::_validate_timestamp(received_predicates, referent, &proof_req.non_revoked, &info.non_revoked))
            .collect::<IndyResult<Vec<()>>>()?;

        Ok(())
    }

    fn _validate_timestamp(received_: &HashMap<String, Identifier>, referent: &str,
                           global_interval: &Option<NonRevocedInterval>, local_interval: &Option<NonRevocedInterval>) -> IndyResult<()> {
        if get_non_revoc_interval(global_interval, local_interval).is_none() {
            return Ok(());
        }

        if !received_
            .get(referent)
            .map(|attr| attr.timestamp.is_some())
            .unwrap_or(false) {
            return Err(IndyError::from(IndyErrorKind::InvalidStructure));
        }

        Ok(())
    }

    fn _received_revealed_attrs(proof: &Proof) -> IndyResult<HashMap<String, Identifier>> {
        let mut revealed_identifiers: HashMap<String, Identifier> = HashMap::new();
        for (referent, info) in proof.requested_proof.revealed_attrs.iter() {
            revealed_identifiers.insert(
                referent.to_string(),
                Verifier::_get_proof_identifier(proof, info.sub_proof_index)?
            );
        }
        for (referent, infos) in proof.requested_proof.revealed_attr_groups.iter() {
            revealed_identifiers.insert(
                referent.to_string(),
                Verifier::_get_proof_identifier(proof, infos.sub_proof_index)?
            );
        }
        Ok(revealed_identifiers)
    }

    fn _received_unrevealed_attrs(proof: &Proof) -> IndyResult<HashMap<String, Identifier>> {
        let mut unrevealed_identifiers: HashMap<String, Identifier> = HashMap::new();
        for (referent, info) in proof.requested_proof.unrevealed_attrs.iter() {
            unrevealed_identifiers.insert(
                referent.to_string(),
                Verifier::_get_proof_identifier(proof, info.sub_proof_index)?
            );
        }
        Ok(unrevealed_identifiers)
    }

    fn _received_predicates(proof: &Proof) -> IndyResult<HashMap<String, Identifier>> {
        let mut predicate_identifiers: HashMap<String, Identifier> = HashMap::new();
        for (referent, info) in proof.requested_proof.predicates.iter() {
            predicate_identifiers.insert(
                referent.to_string(),
                Verifier::_get_proof_identifier(proof, info.sub_proof_index)?
            );
        }
        Ok(predicate_identifiers)
    }

    fn _received_self_attested_attrs(proof: &Proof) -> HashSet<String> {
        proof.requested_proof.self_attested_attrs
            .keys()
            .cloned()
            .collect()
    }

    fn _get_proof_identifier(proof: &Proof, index: u32) -> IndyResult<Identifier> {
        proof.identifiers
            .get(index as usize)
            .cloned()
            .ok_or_else(|| err_msg(
                IndyErrorKind::InvalidStructure,
                format!("Identifier not found for index: {}", index)
            ))
    }

    fn _verify_revealed_attribute_values(proof_req: &ProofRequestPayload,
                                         proof: &Proof) -> IndyResult<()> {
        for (attr_referent, attr_info) in proof.requested_proof.revealed_attrs.iter() {

            let attr_name = proof_req.requested_attributes.get(attr_referent)
                .as_ref()
                .ok_or(IndyError::from_msg(IndyErrorKind::ProofRejected, format!("Attribute with referent \"{}\" not found in ProofRequests", attr_referent)))?
                .name.as_ref()
                .ok_or(IndyError::from_msg(IndyErrorKind::ProofRejected, format!("Attribute with referent \"{}\" not found in ProofRequests", attr_referent)))?;
            Verifier::_verify_revealed_attribute_value(attr_name.as_str(), proof, &attr_info)?;
        }

        for (attr_referent, attr_infos) in proof.requested_proof.revealed_attr_groups.iter() {
            let attr_names = proof_req.requested_attributes.get(attr_referent)
                .as_ref()
                .ok_or(IndyError::from_msg(IndyErrorKind::ProofRejected, format!("Attribute with referent \"{}\" not found in ProofRequests", attr_referent)))?
                .names.as_ref()
                .ok_or(IndyError::from_msg(IndyErrorKind::ProofRejected, format!("Attribute with referent \"{}\" not found in ProofRequests", attr_referent)))?;
            if attr_infos.values.len() != attr_names.len() {
                error!("Proof Revealed Attr Group does not match Proof Request Attribute Group, proof request attrs: {:?}, referent: {:?}, attr_infos: {:?}", proof_req.requested_attributes, attr_referent, attr_infos);
                return Err(IndyError::from_msg(IndyErrorKind::InvalidStructure, "Proof Revealed Attr Group does not match Proof Request Attribute Group"))
            }
            for attr_name in attr_names {
                let attr_info = &attr_infos.values.get(attr_name)
                    .ok_or(IndyError::from_msg(IndyErrorKind::InvalidStructure, "Proof Revealed Attr Group does not match Proof Request Attribute Group"))?;
                Verifier::_verify_revealed_attribute_value(attr_name, proof, &RevealedAttributeInfo {
                    sub_proof_index: attr_infos.sub_proof_index,
                    raw: attr_info.raw.clone(),
                    encoded: attr_info.encoded.clone()
                })?;
            }
        }
        Ok(())
    }

    fn _verify_revealed_attribute_value(attr_name: &str,
                                        proof: &Proof,
                                        attr_info: &RevealedAttributeInfo) -> IndyResult<()> {
        let reveal_attr_encoded = &attr_info.encoded;
        let sub_proof_index = attr_info.sub_proof_index as usize;

        let crypto_proof_encoded = proof.proof.proofs
            .get(sub_proof_index)
            .ok_or(IndyError::from_msg(IndyErrorKind::ProofRejected, format!("CryptoProof not found by index \"{}\"", sub_proof_index)))?
            .revealed_attrs()?
            .iter()
            .find(|(key, _)| attr_common_view(attr_name) == attr_common_view(&key))
            .map(|(_, val)| val.to_string())
            .ok_or(IndyError::from_msg(IndyErrorKind::ProofRejected, format!("Attribute with name \"{}\" not found in CryptoProof", attr_name)))?;

        if BigNumber::from_dec(reveal_attr_encoded)? != BigNumber::from_dec(&crypto_proof_encoded)? {
            return Err(IndyError::from_msg(IndyErrorKind::ProofRejected,
                                           format!("Encoded Values for \"{}\" are different in RequestedProof \"{}\" and CryptoProof \"{}\"", attr_name, reveal_attr_encoded, crypto_proof_encoded)));
        }

        Ok(())
    }

    fn _verify_requested_restrictions(proof_req: &ProofRequestPayload,
                                      requested_proof: &RequestedProof,
                                      received_revealed_attrs: &HashMap<String, Identifier>,
                                      received_unrevealed_attrs: &HashMap<String, Identifier>,
                                      received_predicates: &HashMap<String, Identifier>,
                                      self_attested_attrs: &HashSet<String>) -> IndyResult<()> {
        let proof_attr_identifiers: HashMap<String, Identifier> = received_revealed_attrs
            .iter()
            .chain(received_unrevealed_attrs)
            .map(|(r, id)| (r.to_string(), id.clone()))
            .collect();

        let requested_attrs: HashMap<String, AttributeInfo> = proof_req.requested_attributes
            .iter()
            .filter(|&(referent, info)| !Verifier::_is_self_attested(&referent, &info, self_attested_attrs))
            .map(|(referent, info)| (referent.to_string(), info.clone()))
            .collect();

        for (referent, info) in requested_attrs {
            if let Some(ref query) = info.restrictions {
                let filter = Verifier::_gather_filter_info(&referent, &proof_attr_identifiers)?;

                let name_value_map: HashMap<String, Option<&str>> = if let Some(name) = info.name {
                    let mut map = HashMap::new();
                    map.insert(name.clone(), requested_proof.revealed_attrs.get(&referent).map(|attr| attr.raw.as_str()));
                    map
                } else if let Some(names) = info.names {
                    let mut map = HashMap::new();
                    let attrs = requested_proof.revealed_attr_groups.get(&referent)
                        .ok_or(IndyError::from_msg(IndyErrorKind::InvalidStructure, "Proof does not have referent from proof request"))?;
                    for name in names {
                        let val = attrs.values.get(&name).map(|attr| attr.raw.as_str());
                        map.insert(name, val);
                    }
                    map
                } else {
                    error!(r#"Proof Request attribute restriction should contain "name" or "names" param. Current proof request: {:?}"#, proof_req);
                    return Err(IndyError::from_msg(IndyErrorKind::InvalidStructure, r#"Proof Request attribute restriction should contain "name" or "names" param"#));
                };

                Verifier::_do_process_operator(&name_value_map, &query, &filter)
                    .map_err(|err| err.extend(format!("Requested restriction validation failed for \"{:?}\" attributes", &name_value_map)))?;
            }
        }

        for (referent, info) in proof_req.requested_predicates.iter() {
            if let Some(ref query) = info.restrictions {
                let filter = Verifier::_gather_filter_info(&referent, received_predicates)?;

                Verifier::_process_operator(&info.name, &query, &filter, None)
                    .map_err(|err| err.extend(format!("Requested restriction validation failed for \"{}\" predicate", &info.name)))?;
            }
        }

        Ok(())
    }

    fn _is_self_attested(referent: &str, info: &AttributeInfo, self_attested_attrs: &HashSet<String>) -> bool {
        match info.restrictions.as_ref() {
            Some(&Query::And(ref array)) | Some(&Query::Or(ref array)) if array.is_empty() =>
                self_attested_attrs.contains(referent),
            None => self_attested_attrs.contains(referent),
            Some(_) => false
        }
    }

    fn _gather_filter_info(referent: &str,
                           identifiers: &HashMap<String, Identifier>) -> IndyResult<Filter> {
        let identifier = identifiers
            .get(referent)
            .ok_or_else(|| err_msg(
                IndyErrorKind::InvalidState,
                format!("Identifier not found for referent: {}", referent))
            )?;

        let (schema_issuer_did, schema_name, schema_version) = identifier.schema_id.parts()
            .ok_or(IndyError::from_msg(IndyErrorKind::InvalidState, format!("Invalid Schema ID `{}`: wrong number of parts", identifier.schema_id.0)))?;

        let issuer_did = identifier.cred_def_id.issuer_did()
            .ok_or(IndyError::from_msg(IndyErrorKind::InvalidState, format!("Invalid Credential Definition ID `{}`: wrong number of parts", identifier.cred_def_id.0)))?;

        Ok(Filter {
            schema_id: identifier.schema_id.0.to_string(),
            schema_name,
            schema_issuer_did: schema_issuer_did.0,
            schema_version,
            cred_def_id: identifier.cred_def_id.0.to_string(),
            issuer_did: issuer_did.0
        })
    }

    fn _process_operator(attr: &str,
                         restriction_op: &Query,
                         filter: &Filter,
                         revealed_value: Option<&str>) -> IndyResult<()> {
        let mut attr_value_map = HashMap::new();
        attr_value_map.insert(attr.to_string(), revealed_value);
        Verifier::_do_process_operator(&attr_value_map, restriction_op, filter)
    }

    fn _do_process_operator(attr_value_map: &HashMap<String, Option<&str>>,
                            restriction_op: &Query,
                            filter: &Filter) -> IndyResult<()> {
        match restriction_op {
            Query::Eq(ref tag_name, ref tag_value) => {
                Verifier::_process_filter(attr_value_map, &tag_name, &tag_value, filter)
                    .map_err(|err| err.extend(format!("$eq operator validation failed for tag: \"{}\", value: \"{}\"", tag_name, tag_value)))
            }
            Query::Neq(ref tag_name, ref tag_value) => {
                if Verifier::_process_filter(attr_value_map, &tag_name, &tag_value, filter).is_err() {
                    Ok(())
                } else {
                    Err(IndyError::from_msg(IndyErrorKind::ProofRejected,
                                            format!("$neq operator validation failed for tag: \"{}\", value: \"{}\". Condition was passed.", tag_name, tag_value)))
                }
            }
            Query::In(ref tag_name, ref tag_values) => {
                let res = tag_values
                    .iter()
                    .any(|val| Verifier::_process_filter(attr_value_map, &tag_name, &val, filter).is_ok());
                if res {
                    Ok(())
                } else {
                    Err(IndyError::from_msg(IndyErrorKind::ProofRejected,
                                            format!("$in operator validation failed for tag: \"{}\", values \"{:?}\".", tag_name, tag_values)))
                }
            }
            Query::And(ref operators) => {
                operators
                    .iter()
                    .map(|op| Verifier::_do_process_operator(attr_value_map, op, filter))
                    .collect::<IndyResult<Vec<()>>>()
                    .map(|_| ())
                    .map_err(|err| err.extend("$and operator validation failed."))
            }
            Query::Or(ref operators) => {
                let res = operators
                    .iter()
                    .any(|op| Verifier::_do_process_operator(attr_value_map, op, filter).is_ok());
                if res {
                    Ok(())
                } else {
                    Err(IndyError::from_msg(IndyErrorKind::ProofRejected, "$or operator validation failed. All conditions were failed."))
                }
            }
            Query::Not(ref operator) => {
                if Verifier::_do_process_operator(attr_value_map, &*operator, filter).is_err() {
                    Ok(())
                } else {
                    Err(IndyError::from_msg(IndyErrorKind::ProofRejected, "$not operator validation failed. All conditions were passed."))
                }
            }
            _ => Err(IndyError::from_msg(IndyErrorKind::ProofRejected, "unsupported operator"))
        }
    }

    fn _process_filter(attr_value_map: &HashMap<String, Option<&str>>,
                       tag: &str,
                       tag_value: &str,
                       filter: &Filter) -> IndyResult<()> {
        trace!("_process_filter: attr_value_map: {:?}, tag: {}, tag_value: {}, filter: {:?}", attr_value_map, tag, tag_value, filter);
        match tag {
            tag_ @ "schema_id" => Verifier::_precess_filed(tag_, &filter.schema_id, tag_value),
            tag_ @ "schema_issuer_did" => Verifier::_precess_filed(tag_, &filter.schema_issuer_did, tag_value),
            tag_ @ "schema_name" => Verifier::_precess_filed(tag_, &filter.schema_name, tag_value),
            tag_ @ "schema_version" => Verifier::_precess_filed(tag_, &filter.schema_version, tag_value),
            tag_ @ "cred_def_id" => Verifier::_precess_filed(tag_, &filter.cred_def_id, tag_value),
            tag_ @ "issuer_did" => Verifier::_precess_filed(tag_, &filter.issuer_did, tag_value),
            x if Verifier::_is_attr_internal_tag(x, attr_value_map) => Verifier::_check_internal_tag_revealed_value(x, tag_value, attr_value_map),
            x if Verifier::_is_attr_operator(x) => Ok(()),
            _ => Err(err_msg(IndyErrorKind::InvalidStructure, "Unknown Filter Type"))
        }
    }

    fn _precess_filed(filed: &str, filter_value: &str, tag_value: &str) -> IndyResult<()> {
        if filter_value == tag_value {
            Ok(())
        } else {
            Err(IndyError::from_msg(IndyErrorKind::ProofRejected, format!("\"{}\" values are different: expected: \"{}\", actual: \"{}\"", filed, tag_value, filter_value)))
        }
    }

    fn _is_attr_internal_tag(key: &str, attr_value_map: &HashMap<String, Option<&str>>) -> bool {
        INTERNAL_TAG_MATCHER.captures(key).map( |caps|
            caps.get(1).map(|s| attr_value_map.contains_key(&s.as_str().to_string())).unwrap_or(false)
        ).unwrap_or(false)
    }

    fn _check_internal_tag_revealed_value(key: &str, tag_value: &str, attr_value_map: &HashMap<String, Option<&str>>) -> IndyResult<()> {
        let attr_name = INTERNAL_TAG_MATCHER.captures(key)
            .ok_or(IndyError::from_msg(IndyErrorKind::InvalidState, format!("Attribute name became unparseable")))?
            .get(1)
            .ok_or(IndyError::from_msg(IndyErrorKind::InvalidState, format!("No name has been parsed")))?
            .as_str();
        if let Some(Some(revealed_value)) = attr_value_map.get(attr_name) {
            if *revealed_value != tag_value {
                return Err(IndyError::from_msg(IndyErrorKind::ProofRejected,
                                               format!("\"{}\" values are different: expected: \"{}\", actual: \"{}\"", key, tag_value, revealed_value)));
            }
        }
        Ok(())
    }

    fn _is_attr_operator(key: &str) -> bool { key.starts_with("attr::") && key.ends_with("::marker") }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const SCHEMA_ID: &str = "123";
    pub const SCHEMA_NAME: &str = "Schema Name";
    pub const SCHEMA_ISSUER_DID: &str = "234";
    pub const SCHEMA_VERSION: &str = "1.2.3";
    pub const CRED_DEF_ID: &str = "345";
    pub const ISSUER_DID: &str = "456";

    fn schema_id_tag() -> String { "schema_id".to_string() }

    fn schema_name_tag() -> String { "schema_name".to_string() }

    fn schema_issuer_did_tag() -> String { "schema_issuer_did".to_string() }

    fn schema_version_tag() -> String { "schema_version".to_string() }

    fn cred_def_id_tag() -> String { "cred_def_id".to_string() }

    fn issuer_did_tag() -> String { "issuer_did".to_string() }

    fn attr_tag() -> String { "attr::zip::marker".to_string() }

    fn attr_tag_value() -> String { "attr::zip::value".to_string() }

    fn bad_attr_tag() -> String { "bad::zip::marker".to_string() }

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
    fn test_empty_non_revoc_interval() {
        let req: ProofRequestPayload = serde_json::from_str(r#"{"nonce":"350088117388834113550940","name":"","version":"1.0","requested_attributes":{"c":{"name":"c","self_attest_allowed":false}},"requested_predicates":{},"non_revoked":{},"ver":"1.0"}"#).unwrap();
        let proof: Proof = serde_json::from_str(r#"{"proof":{"proofs":[{"primary_proof":{"eq_proof":{"revealed_attrs":{"c":"21027550693477535543327579570081618952892630736730429980018215117041635618758"},"a_prime":"50388520465356767417155078132030364828017082181937472057908950883185050795637638521491388022180158096759232455736729261978731637595596549484937975342410151565026856449579187408834016491818336678384318635829996844370239148877256608389597012804517879711463995894364211890879328570767112882081259647215261169380817621943175013397600182171422667024541604800127259870349821935302913920399265872658916941791487376754251806405022052841456989599520941866630864402059993498429340012748885964230763777867696791164977533357096580962063739663676920288158021876789561157406722334879653981433905074563263686820665755735941850393511","e":"3718252388493220772864850593185990486135588489586438191852057691994557460042151039739352906654425489071229420043584597583873303387892223","v":"104640528163077132809792479674713277838244565048693549308340760186318981362337779355633304005425234273822804109722544797277985695549991316279763495266300729281061157372981253110523240081016421445239006840238133538749388746702160521905800492207456762246574543682763121153299505947499924773003208950905019356780627958242742692116587293954021084445817233197192748937381218872493995710613051335755003619837718060183564330338304363793977418067994819375112798429676406003204697931185427433641122048516176160127110000771649047444572825666070065159533366557725976635979829699967134168074789619670201114793084666975282740956550669322787762871238486492698983311622912661482195047592809308438881684271009272299491341075496882870805853135019732764728282598384361962265009133066725402781677996257080500889204479458351603004601880071221922297408377583195187139021070874000837495039932763863398129908581530563333100942188144602123787677","m":{"a":"4102297283094307290070770953943747038183410804299116114299791883760019597647346702211117588763089727351195631823886410052637465380736732353976328309032323428299014078787101226568","b":"11997686085288782554771339082926619203863597953919130241562055492237714615117568559401642079900460111445298407658208048722258427615792706143750355705939032320754588549379013958815","master_secret":"7403368031144039226544488947490391020647450887020280303878603763599271583741034875603370919905207018690229154315040064338060768194187934216018446695234924266613830660197632191538"},"m2":"2760732335130882579050846046090122927882682127404914937843020411814103100916300452069994839423297306091507968691632756648481949369278627609217256945492808886926235206847576613769"},"ge_proofs":[]},"non_revoc_proof":null}],"aggregated_proof":{"c_hash":"84983350747251740997636647643875390715825568809832255503441787009241108371023","c_list":[[1,143,39,106,130,242,130,124,217,142,142,7,16,79,228,0,94,107,83,200,99,25,75,124,93,96,250,10,165,162,53,107,29,157,224,211,100,100,228,37,105,64,202,38,212,234,48,143,210,67,85,42,73,210,122,16,21,240,75,197,79,216,72,160,19,114,114,134,36,5,237,230,81,108,7,250,174,166,53,161,84,47,243,37,168,123,53,197,244,147,218,18,28,164,112,165,2,193,72,159,159,229,199,94,201,123,30,171,108,95,242,214,213,107,13,75,165,199,145,194,76,62,246,251,244,139,101,233,49,195,130,248,197,210,49,252,201,123,61,164,200,90,37,166,25,83,91,19,239,85,237,45,82,175,0,130,132,93,62,59,1,34,140,127,28,64,45,66,154,22,122,94,181,107,98,79,156,228,72,20,69,132,251,250,2,220,225,169,83,83,253,22,152,203,244,113,102,91,227,12,46,32,142,209,191,211,85,183,251,80,105,247,49,130,136,140,155,22,154,163,156,110,189,204,3,186,231,155,21,26,114,212,224,229,150,44,196,248,199,189,166,221,141,113,43,40,98,147,196,147,5,0,199,55,64,235,167]]}},"requested_proof":{"revealed_attrs":{"c":{"sub_proof_index":0,"raw":"c","encoded":"21027550693477535543327579570081618952892630736730429980018215117041635618758"}},"self_attested_attrs":{},"unrevealed_attrs":{},"predicates":{}},"identifiers":[{"schema_id":"WpoQirzsNJK2MUZh3NtbF2:2:r-test6:0.0.6","cred_def_id":"WpoQirzsNJK2MUZh3NtbF2:3:CL:13054:latest","rev_reg_id":null,"timestamp":null}]}"#).unwrap();
        let schema: HashMap<SchemaId, SchemaV1> = serde_json::from_str(r#"{"WpoQirzsNJK2MUZh3NtbF2:2:r-test6:0.0.6": {"id":"WpoQirzsNJK2MUZh3NtbF2:2:r-test6:0.0.6","name":"r-test6","version":"0.0.6","attrNames":["c","b","a"],"seqNo":13054,"ver":"1.0"} }"#).unwrap();
        let creds: HashMap<CredentialDefinitionId, CredentialDefinitionV1> = serde_json::from_str(r#"{ "WpoQirzsNJK2MUZh3NtbF2:3:CL:13054:latest": {"id":"WpoQirzsNJK2MUZh3NtbF2:3:CL:13054:latest","type":"CL","schemaId":"13054","tag":"latest","ver":"1.0","value":{"primary":{"s":"96281451931856950071247732301145128947395610493013790310955990632633196885524793845334092537818922462264233022335361183133303852252721080285326870125311415851097177488432178086983471524491310364473905754976792959714458623277090581761856030582322555208165314932987606825708270123900987672643134498105957346390925811566016857855277893601189551576344965811905485514252943025045995878374350226086530516844678113271693623547902253153800561152360320953531905567988013083052780868065774395923270627658499493102580743356089788769148151557400535173723073517353217325789939958934630352069511258016506202345767783345525080514474","rctxt":"96998451572271599792505515998905404313822535725743659333009620972024474419165225006658994968652665129982368129141430714029514712803791540302463754426686263475178479390493338285067758304533650260622668304644944662586345358384128327350458464071515504566507909265959846213397477648229035068852561101255935668747499429993060499833043824211261563456704613189698846135769826705989673705929539984289006474553594930706936326055032186119654958987858469763606042213078013130548232666993520473193450697524811754936832672642999087232999664717919255976658569052776471889054382630343081737911995800323308162658118020148187375859346","n":"113909196105256369298794250761803707037872974702162648916132271040126036520095478143959818528541457730199089909158389355847613025323021961254681263253018494381662582562386791440595165294379567145823122114055133246595774750219255157013053637028070800432740286027834262091355949659551244384233091515975531556158112365859372417503487250115030463589910881694542256734795571839713753795964719045885959330186901174862010728157608340363095920921222819146956958351183432894842099559181519425881567672609297067126008424449576376885625476556442018005482629148358509413392075152511721848631280738309309273110686347955436921141757","r":{"a":"110128981229894217289000041492471646607231091138474619889388522059639243576977359609128317744176147865322251874732765979876056559192698976435215588402978276349860053066456685802166597798564986869248643522975662590517463296807944008918364822611990860724233744628049818938587483522671136037026664206423191661796143287688851027306901871002493315458325648804809247015099876745919397691362034719825864663730316054871526119042379859903809496822148282667019008992890737310457009323390133530255483685575574384062379768728734890648785178109304235097836968585258825571207614703459189373282519835379577958865751055495452861262377","master_secret":"106210598528633765713867548127123931325839222644113518212169901407494857424926435983985511935195880774339068703651163413794635973372563730936558585629753445688835127043954705644990189643521094565675572094879712761794052268208353323672847703575087498806334277298785542119933885848841906239832071565222367429603886205681745154729634647870198651779369569163613206282806992627703520325290910425080530088629190524240264697380132804569005664543143336614708036312973679572122060258024361868915466817853822901184163639414792187579917996881662528096890770892985619580735441167183082237286207513931691214829540248663398414463093","c":"90884019650334942009676982498450127388588482189207541064219041184166116776398138572870544845682439237083052793877086793078204513573121904761365373246291878444483913042824450193890151529157446723610487668049209816394449193515809338690982666650214928414865522410883305856438137961463195613461552315441241462285268915919255573022049026932180805578256042928592633468606981240952285958657029154733433737948884509351613422628434650291829099433872466002096126279765016230751650409357221364497623030797134194317844080450753610659950999775313853360404266023181884700329393766428537041381211155080204899325330817022052038122520","b":"25289224731620891580745406001804170586383137910854270930230203111536358416558534310576699292130038318363697156167208450553087573274410767259244474717715942957096724588072671951751311699376714345997045259622206358703957664488885354464994708867328975356632764943968211064353967776351128358106331951860772501655778558010162368989946304061517995631431740306419762877205897691736164562835459313211044176809993807366088031862883674460100427138170570261806168011698838759587962016689773768947556607754099087791412406559221306023952805465006012539382740099965546259533910152697046157505609036464837442617964329051465643410334"},"z":"13448999734459810526894396672497725000152968672330433365643675850410852172788604372585908366969611286220921877457227714468366054783073419906923567316811894875571027354406236017467282592343920751798636633194686898311360231186164162309744544125989143104238467399330171200792094494850155591502329487845456246958013783767154602668852911245845750519409966502420160094016840289569846303357769716921617338174843486004045930151446328223906479952073328637045335041738711974320925607713462228774461301805619085790123472704544756026055470449420564951716493811825626469751081429719777219768204597043609857510514835736157982993912"}}} }"#).unwrap();
        let rev_reg_defs: HashMap<RevocationRegistryId, RevocationRegistryDefinitionV1> = HashMap::new();
        let rev_regs: HashMap<RevocationRegistryId, HashMap<u64, RevocationRegistryV1>> = HashMap::new();
        let rc = Verifier::new().verify(
            &proof, &req, &schema, &creds, &rev_reg_defs, &rev_regs
        ).unwrap();
    }

    #[test]
    fn test_process_op_eq() {
        let filter = filter();

        let mut op = Query::Eq(schema_id_tag(), SCHEMA_ID.to_string());
        Verifier::_process_operator("zip", &op, &filter, None).unwrap();

        op = Query::And(vec![
            Query::Eq(attr_tag(), "1".to_string()),
            Query::Eq(schema_id_tag(), SCHEMA_ID.to_string()),
        ]);
        Verifier::_process_operator("zip", &op, &filter, None).unwrap();

        op = Query::And(vec![
            Query::Eq(bad_attr_tag(), "1".to_string()),
            Query::Eq(schema_id_tag(), SCHEMA_ID.to_string()),
        ]);
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());

        op = Query::Eq(schema_id_tag(), "NOT HERE".to_string());
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());
    }

    #[test]
    fn test_process_op_ne() {
        let filter = filter();
        let mut op = Query::Neq(schema_id_tag(), SCHEMA_ID.to_string());
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());

        op = Query::Neq(schema_id_tag(), "NOT HERE".to_string());
        Verifier::_process_operator("zip", &op, &filter, None).unwrap()
    }

    #[test]
    fn test_process_op_in() {
        let filter = filter();
        let mut cred_def_ids = vec!["Not Here".to_string()];

        let mut op = Query::In(cred_def_id_tag(), cred_def_ids.clone());
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());

        cred_def_ids.push(CRED_DEF_ID.to_string());
        op = Query::In(cred_def_id_tag(), cred_def_ids.clone());
        Verifier::_process_operator("zip", &op, &filter, None).unwrap()
    }

    #[test]
    fn test_process_op_or() {
        let filter = filter();
        let mut op = Query::Or(vec![
            Query::Eq(schema_id_tag(), "Not Here".to_string()),
            Query::Eq(cred_def_id_tag(), "Not Here".to_string())
        ]);
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());

        op = Query::Or(vec![
            Query::Eq(schema_id_tag(), SCHEMA_ID.to_string()),
            Query::Eq(cred_def_id_tag(), "Not Here".to_string())
        ]);
        Verifier::_process_operator("zip", &op, &filter, None).unwrap()
    }

    #[test]
    fn test_process_op_and() {
        let filter = filter();
        let mut op = Query::And(vec![
            Query::Eq(schema_id_tag(), "Not Here".to_string()),
            Query::Eq(cred_def_id_tag(), "Not Here".to_string())
        ]);
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());

        op = Query::And(vec![
            Query::Eq(schema_id_tag(), SCHEMA_ID.to_string()),
            Query::Eq(cred_def_id_tag(), "Not Here".to_string())
        ]);
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());

        op = Query::And(vec![
            Query::Eq(schema_id_tag(), SCHEMA_ID.to_string()),
            Query::Eq(cred_def_id_tag(), CRED_DEF_ID.to_string())
        ]);
        Verifier::_process_operator("zip", &op, &filter, None).unwrap()
    }

    #[test]
    fn test_process_op_not() {
        let filter = filter();
        let mut op = Query::Not(Box::new(Query::And(vec![
            Query::Eq(schema_id_tag(), SCHEMA_ID.to_string()),
            Query::Eq(cred_def_id_tag(), CRED_DEF_ID.to_string())
        ])));
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());

        op = Query::Not(Box::new(Query::And(vec![
            Query::Eq(schema_id_tag(), "Not Here".to_string()),
            Query::Eq(cred_def_id_tag(), "Not Here".to_string())
        ])));
        Verifier::_process_operator("zip", &op, &filter, None).unwrap()
    }

    #[test]
    fn test_proccess_op_or_with_nested_and() {
        let filter = filter();
        let mut op = Query::Or(vec![
            Query::And(vec![
                Query::Eq(schema_id_tag(), "Not Here".to_string()),
                Query::Eq(cred_def_id_tag(), "Not Here".to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_issuer_did_tag(), "Not Here".to_string()),
                Query::Eq(schema_name_tag(), "Not Here".to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_name_tag(), "Not Here".to_string()),
                Query::Eq(issuer_did_tag(), "Not Here".to_string())
            ]),
        ]);
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());

        op = Query::Or(vec![
            Query::And(vec![
                Query::Eq(schema_id_tag(), SCHEMA_ID.to_string()),
                Query::Eq(cred_def_id_tag(), "Not Here".to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_issuer_did_tag(), "Not Here".to_string()),
                Query::Eq(schema_name_tag(), "Not Here".to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_name_tag(), "Not Here".to_string()),
                Query::Eq(issuer_did_tag(), "Not Here".to_string())
            ]),
        ]);
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());

        op = Query::Or(vec![
            Query::And(vec![
                Query::Eq(schema_id_tag(), SCHEMA_ID.to_string()),
                Query::Eq(cred_def_id_tag(), CRED_DEF_ID.to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_issuer_did_tag(), "Not Here".to_string()),
                Query::Eq(schema_name_tag(), "Not Here".to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_name_tag(), "Not Here".to_string()),
                Query::Eq(issuer_did_tag(), "Not Here".to_string())
            ]),
        ]);
        Verifier::_process_operator("zip", &op, &filter, None).unwrap()
    }

    #[test]
    fn test_verify_op_complex_nested() {
        let filter = filter();
        let mut op = Query::And(vec![
            Query::And(vec![
                Query::Or(vec![
                    Query::Eq(schema_name_tag(), "Not Here".to_string()),
                    Query::Eq(issuer_did_tag(), "Not Here".to_string())
                ]),
                Query::Eq(schema_id_tag(), SCHEMA_ID.to_string()),
                Query::Eq(cred_def_id_tag(), CRED_DEF_ID.to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_issuer_did_tag(), SCHEMA_ISSUER_DID.to_string()),
                Query::Eq(schema_name_tag(), SCHEMA_NAME.to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_version_tag(), SCHEMA_VERSION.to_string()),
                Query::Eq(issuer_did_tag(), ISSUER_DID.to_string())
            ]),
        ]);
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());

        op = Query::And(vec![
            Query::And(vec![
                Query::Or(vec![
                    Query::Eq(schema_name_tag(), SCHEMA_NAME.to_string()),
                    Query::Eq(issuer_did_tag(), "Not Here".to_string())
                ]),
                Query::Eq(schema_id_tag(), SCHEMA_ID.to_string()),
                Query::Eq(cred_def_id_tag(), CRED_DEF_ID.to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_issuer_did_tag(), SCHEMA_ISSUER_DID.to_string()),
                Query::Eq(schema_name_tag(), SCHEMA_NAME.to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_version_tag(), SCHEMA_VERSION.to_string()),
                Query::Eq(issuer_did_tag(), ISSUER_DID.to_string())
            ]),
            Query::Not(Box::new(Query::Eq(schema_version_tag(), "NOT HERE".to_string())))
        ]);
        Verifier::_process_operator("zip", &op, &filter, None).unwrap();

        op = Query::And(vec![
            Query::And(vec![
                Query::Or(vec![
                    Query::Eq(schema_name_tag(), SCHEMA_NAME.to_string()),
                    Query::Eq(issuer_did_tag(), "Not Here".to_string())
                ]),
                Query::Eq(schema_id_tag(), SCHEMA_ID.to_string()),
                Query::Eq(cred_def_id_tag(), CRED_DEF_ID.to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_issuer_did_tag(), SCHEMA_ISSUER_DID.to_string()),
                Query::Eq(schema_name_tag(), SCHEMA_NAME.to_string())
            ]),
            Query::And(vec![
                Query::Eq(schema_version_tag(), SCHEMA_VERSION.to_string()),
                Query::Eq(issuer_did_tag(), ISSUER_DID.to_string())
            ]),
            Query::Not(Box::new(Query::Eq(schema_version_tag(), SCHEMA_VERSION.to_string())))
        ]);
        assert!(Verifier::_process_operator("zip", &op, &filter, None).is_err());
    }

    #[test]
    fn test_process_op_eq_revealed_value() {
        let filter = filter();
        let value = "value";

        let mut op = Query::Eq(attr_tag_value(), value.to_string());
        Verifier::_process_operator("zip", &op, &filter, Some(value)).unwrap();

        op = Query::And(vec![
            Query::Eq(attr_tag_value(), value.to_string()),
            Query::Eq(schema_issuer_did_tag(), SCHEMA_ISSUER_DID.to_string()),
        ]);
        Verifier::_process_operator("zip", &op, &filter, Some(value)).unwrap();

        op = Query::Eq(attr_tag_value(), value.to_string());
        assert!(Verifier::_process_operator("zip", &op, &filter, Some("NOT HERE")).is_err());
    }

    fn _received() -> HashMap<String, Identifier> {
        let mut res: HashMap<String, Identifier> = HashMap::new();
        res.insert("referent_1".to_string(), Identifier { timestamp: Some(1234), schema_id: SchemaId(String::new()), cred_def_id: CredentialDefinitionId(String::new()), rev_reg_id: Some(RevocationRegistryId(String::new())) });
        res.insert("referent_2".to_string(), Identifier { timestamp: None, schema_id: SchemaId(String::new()), cred_def_id: CredentialDefinitionId(String::new()), rev_reg_id: Some(RevocationRegistryId(String::new())) });
        res
    }

    fn _interval() -> NonRevocedInterval {
        NonRevocedInterval { from: None, to: Some(1234) }
    }

    #[test]
    fn validate_timestamp_works() {
        Verifier::_validate_timestamp(&_received(), "referent_1", &None, &None).unwrap();
        Verifier::_validate_timestamp(&_received(), "referent_1", &Some(_interval()), &None).unwrap();
        Verifier::_validate_timestamp(&_received(), "referent_1", &None, &Some(_interval())).unwrap();
    }

    #[test]
    fn validate_timestamp_not_work() {
        Verifier::_validate_timestamp(&_received(), "referent_2", &Some(_interval()), &None).unwrap_err();
        Verifier::_validate_timestamp(&_received(), "referent_2", &None, &Some(_interval())).unwrap_err();
        Verifier::_validate_timestamp(&_received(), "referent_3", &None, &Some(_interval())).unwrap_err();
    }
}
