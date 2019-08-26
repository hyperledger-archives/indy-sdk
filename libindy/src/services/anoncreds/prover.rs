use std::collections::hash_map::Entry;
use std::collections::HashMap;

use ursa::cl::{
    BlindedCredentialSecrets,
    BlindedCredentialSecretsCorrectnessProof,
    CredentialPublicKey,
    CredentialSecretsBlindingFactors,
    MasterSecret,
    SubProofRequest,
};
use ursa::cl::issuer::Issuer as CryptoIssuer;
use ursa::cl::prover::Prover as CryptoProver;
use ursa::cl::verifier::Verifier as CryptoVerifier;

use domain::anoncreds::credential::{AttributeValues, Credential};
use domain::anoncreds::credential_attr_tag_policy::CredentialAttrTagPolicy;
use domain::anoncreds::credential_definition::CredentialDefinitionV1 as CredentialDefinition;
use domain::anoncreds::credential_offer::CredentialOffer;
use domain::anoncreds::credential_request::CredentialRequestMetadata;
use domain::anoncreds::proof::{Identifier, Proof, RequestedProof, RevealedAttributeInfo, SubProofReferent};
use domain::anoncreds::proof_request::{PredicateInfo, PredicateTypes, ProofRequest, ProofRequestExtraQuery, RequestedAttributeInfo, RequestedPredicateInfo};
use domain::anoncreds::requested_credential::ProvingCredentialKey;
use domain::anoncreds::requested_credential::RequestedCredentials;
use domain::anoncreds::revocation_registry_definition::RevocationRegistryDefinitionV1;
use domain::anoncreds::revocation_state::RevocationState;
use domain::anoncreds::schema::SchemaV1;
use errors::prelude::*;
use services::anoncreds::helpers::*;

const ATTRIBUTE_EXISTENCE_MARKER: &str = "1";

pub struct Prover {}

macro_rules! serde_map {
    ($( $key: expr => $val: expr ),*) => {
        {
            let mut map = serde_json::Map::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    }
}

impl Prover {
    pub fn new() -> Prover {
        Prover {}
    }

    pub fn new_master_secret(&self) -> IndyResult<MasterSecret> {
        trace!("new_master_secret >>> ");

        let master_secret = CryptoProver::new_master_secret()?;

        trace!("new_master_secret <<< master_secret: {:?} ", secret!(&master_secret));

        Ok(master_secret)
    }

    pub fn new_credential_request(&self,
                                  cred_def: &CredentialDefinition,
                                  master_secret: &MasterSecret,
                                  credential_offer: &CredentialOffer) -> IndyResult<(BlindedCredentialSecrets,
                                                                                     CredentialSecretsBlindingFactors,
                                                                                     BlindedCredentialSecretsCorrectnessProof)> {
        trace!("new_credential_request >>> cred_def: {:?}, master_secret: {:?}, credential_offer: {:?}",
               cred_def, secret!(&master_secret), credential_offer);

        let credential_pub_key = CredentialPublicKey::build_from_parts(&cred_def.value.primary, cred_def.value.revocation.as_ref())?;
        let mut credential_values_builder = CryptoIssuer::new_credential_values_builder()?;
        credential_values_builder.add_value_hidden("master_secret", &master_secret.value()?)?;
        let cred_values = credential_values_builder.finalize()?;

        let (blinded_credential_secrets, credential_secrets_blinding_factors, blinded_credential_secrets_correctness_proof) =
            CryptoProver::blind_credential_secrets(&credential_pub_key,
                                                   &credential_offer.key_correctness_proof,
                                                   &cred_values,
                                                   &credential_offer.nonce)?;

        trace!("new_credential_request <<< blinded_credential_secrets: {:?}, credential_secrets_blinding_factors: {:?}, blinded_credential_secrets_correctness_proof: {:?}",
               blinded_credential_secrets, credential_secrets_blinding_factors, blinded_credential_secrets_correctness_proof);

        Ok((blinded_credential_secrets, credential_secrets_blinding_factors, blinded_credential_secrets_correctness_proof))
    }

    pub fn process_credential(&self,
                              credential: &mut Credential,
                              cred_request_metadata: &CredentialRequestMetadata,
                              master_secret: &MasterSecret,
                              cred_def: &CredentialDefinition,
                              rev_reg_def: Option<&RevocationRegistryDefinitionV1>) -> IndyResult<()> {
        trace!("process_credential >>> credential: {:?}, cred_request_metadata: {:?}, master_secret: {:?}, cred_def: {:?}, rev_reg_def: {:?}",
               credential, cred_request_metadata, secret!(&master_secret), cred_def, rev_reg_def);

        let credential_pub_key = CredentialPublicKey::build_from_parts(&cred_def.value.primary, cred_def.value.revocation.as_ref())?;
        let credential_values = build_credential_values(&credential.values, Some(master_secret))?;

        CryptoProver::process_credential_signature(&mut credential.signature,
                                                   &credential_values,
                                                   &credential.signature_correctness_proof,
                                                   &cred_request_metadata.master_secret_blinding_data,
                                                   &credential_pub_key,
                                                   &cred_request_metadata.nonce,
                                                   rev_reg_def.as_ref().map(|r_reg_def| &r_reg_def.value.public_keys.accum_key),
                                                   credential.rev_reg.as_ref(),
                                                   credential.witness.as_ref())?;

        trace!("process_credential <<< ");

        Ok(())
    }

    pub fn create_proof(&self,
                        credentials: &HashMap<String, Credential>,
                        proof_req: &ProofRequest,
                        requested_credentials: &RequestedCredentials,
                        master_secret: &MasterSecret,
                        schemas: &HashMap<String, SchemaV1>,
                        cred_defs: &HashMap<String, CredentialDefinition>,
                        rev_states: &HashMap<String, HashMap<u64, RevocationState>>) -> IndyResult<Proof> {
        trace!("create_proof >>> credentials: {:?}, proof_req: {:?}, requested_credentials: {:?}, master_secret: {:?}, schemas: {:?}, cred_defs: {:?}, rev_states: {:?}",
               credentials, proof_req, requested_credentials, secret!(&master_secret), schemas, cred_defs, rev_states);

        let mut proof_builder = CryptoProver::new_proof_builder()?;
        proof_builder.add_common_attribute("master_secret")?;

        let mut requested_proof = RequestedProof::default();

        requested_proof.self_attested_attrs = requested_credentials.self_attested_attributes.clone();

        let credentials_for_proving = Prover::_prepare_credentials_for_proving(requested_credentials, proof_req)?;
        let mut sub_proof_index = 0;
        let non_credential_schema = build_non_credential_schema()?;

        let mut identifiers: Vec<Identifier> = Vec::with_capacity(credentials_for_proving.len());
        for (cred_key, (req_attrs_for_cred, req_predicates_for_cred)) in credentials_for_proving {
            let credential: &Credential = credentials.get(cred_key.cred_id.as_str())
                .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("Credential not found by id: {:?}", cred_key.cred_id)))?;

            let schema: &SchemaV1 = schemas.get(&credential.schema_id)
                .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("Schema not found by id: {:?}", credential.schema_id)))?;

            let cred_def: &CredentialDefinition = cred_defs.get(&credential.cred_def_id)
                .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("CredentialDefinition not found by id: {:?}", credential.cred_def_id)))?;

            let rev_state = if let Some(timestamp) = cred_key.timestamp {
                let rev_reg_id = credential.rev_reg_id
                    .clone()
                    .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Revocation Registry Id not found"))?;

                let rev_states_for_timestamp = rev_states.get(&rev_reg_id)
                    .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("RevocationState not found by id: {:?}", rev_reg_id)))?;

                Some(rev_states_for_timestamp.get(&timestamp)
                    .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("RevocationInfo not found by timestamp: {:?}", timestamp)))?)
            } else { None };

            let credential_pub_key = CredentialPublicKey::build_from_parts(&cred_def.value.primary, cred_def.value.revocation.as_ref())?;

            let credential_schema = build_credential_schema(&schema.attr_names)?;
            let credential_values = build_credential_values(&credential.values, Some(master_secret))?;
            let sub_proof_request = Prover::_build_sub_proof_request(&req_attrs_for_cred, &req_predicates_for_cred)?;

            proof_builder.add_sub_proof_request(&sub_proof_request,
                                                &credential_schema,
                                                &non_credential_schema,
                                                &credential.signature,
                                                &credential_values,
                                                &credential_pub_key,
                                                rev_state.as_ref().map(|r_info| &r_info.rev_reg),
                                                rev_state.as_ref().map(|r_info| &r_info.witness))?;

            identifiers.push(Identifier {
                schema_id: credential.schema_id.clone(),
                cred_def_id: credential.cred_def_id.clone(),
                rev_reg_id: credential.rev_reg_id.clone(),
                timestamp: cred_key.timestamp,
            });

            self._update_requested_proof(req_attrs_for_cred,
                                         req_predicates_for_cred,
                                         proof_req, credential,
                                         sub_proof_index,
                                         &mut requested_proof)?;

            sub_proof_index += 1;
        }

        let proof = proof_builder.finalize(&proof_req.nonce)?;

        let full_proof = Proof {
            proof,
            requested_proof,
            identifiers,
        };

        trace!("create_proof <<< full_proof: {:?}", full_proof);

        Ok(full_proof)
    }

    pub fn _prepare_credentials_for_proving(requested_credentials: &RequestedCredentials,
                                            proof_req: &ProofRequest) -> IndyResult<HashMap<ProvingCredentialKey, (Vec<RequestedAttributeInfo>, Vec<RequestedPredicateInfo>)>> {
        trace!("_prepare_credentials_for_proving >>> requested_credentials: {:?}, proof_req: {:?}", requested_credentials, proof_req);

        let mut credentials_for_proving: HashMap<ProvingCredentialKey, (Vec<RequestedAttributeInfo>, Vec<RequestedPredicateInfo>)> = HashMap::new();

        for (attr_referent, requested_attr) in requested_credentials.requested_attributes.iter() {
            let attr_info = proof_req.requested_attributes
                .get(attr_referent.as_str())
                .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("AttributeInfo not found in ProofRequest for referent \"{}\"", attr_referent.as_str())))?;

            let req_attr_info = RequestedAttributeInfo {
                attr_referent: attr_referent.clone(),
                attr_info: attr_info.clone(),
                revealed: requested_attr.revealed,
            };

            match credentials_for_proving.entry(ProvingCredentialKey { cred_id: requested_attr.cred_id.clone(), timestamp: requested_attr.timestamp }) {
                Entry::Occupied(cred_for_proving) => {
                    let &mut (ref mut attributes_for_credential, _) = cred_for_proving.into_mut();
                    attributes_for_credential.push(req_attr_info);
                }
                Entry::Vacant(attributes_for_credential) => {
                    attributes_for_credential.insert((vec![req_attr_info], Vec::new()));
                }
            };
        }

        for (predicate_referent, proving_cred_key) in requested_credentials.requested_predicates.iter() {
            let predicate_info = proof_req.requested_predicates
                .get(predicate_referent.as_str())
                .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("PredicateInfo not found in ProofRequest for referent \"{}\"", predicate_referent.as_str())))?;

            let req_predicate_info = RequestedPredicateInfo {
                predicate_referent: predicate_referent.clone(),
                predicate_info: predicate_info.clone(),
            };

            match credentials_for_proving.entry(proving_cred_key.clone()) {
                Entry::Occupied(cred_for_proving) => {
                    let &mut (_, ref mut predicates_for_credential) = cred_for_proving.into_mut();
                    predicates_for_credential.push(req_predicate_info);
                }
                Entry::Vacant(v) => {
                    v.insert((Vec::new(), vec![req_predicate_info]));
                }
            };
        }

        trace!("_prepare_credentials_for_proving <<< credentials_for_proving: {:?}", credentials_for_proving);

        Ok(credentials_for_proving)
    }

    pub fn get_credential_values_for_attribute(&self, credential_attrs: &HashMap<String, AttributeValues>,
                                               requested_attr: &str) -> Option<AttributeValues> {
        trace!("get_credential_values_for_attribute >>> credential_attrs: {:?}, requested_attr: {:?}", credential_attrs, requested_attr);

        let res = credential_attrs.iter()
            .find(|&(ref key, _)| attr_common_view(key) == attr_common_view(&requested_attr))
            .map(|(_, values)| values.clone());

        trace!("get_credential_values_for_attribute <<< res: {:?}", res);

        res
    }

    pub fn build_credential_tags(&self, credential: &Credential, catpol: Option<&CredentialAttrTagPolicy>) -> HashMap<String, String> {
        trace!("build_credential_tags >>> credential: {:?}, catpol: {:?}", credential, catpol);

        let mut res: HashMap<String, String> = HashMap::new();
        res.insert("schema_id".to_string(), credential.schema_id());
        res.insert("schema_issuer_did".to_string(), credential.schema_issuer_did());
        res.insert("schema_name".to_string(), credential.schema_name());
        res.insert("schema_version".to_string(), credential.schema_version());
        res.insert("issuer_did".to_string(), credential.issuer_did());
        res.insert("cred_def_id".to_string(), credential.cred_def_id());
        res.insert("rev_reg_id".to_string(), credential.rev_reg_id.clone().unwrap_or_else(|| "None".to_string()));

        credential.values
            .iter()
            .for_each(|(attr, values)| {
                if catpol.map(|cp| cp.is_taggable(attr.as_str())).unwrap_or(true) {
                    // abstain for attrs policy marks untaggable
                    res.insert(format!("attr::{}::marker", attr_common_view(&attr)), ATTRIBUTE_EXISTENCE_MARKER.to_string());
                    res.insert(format!("attr::{}::value", attr_common_view(&attr)), values.raw.clone());
                }
            });

        trace!("build_credential_tags <<< res: {:?}", res);

        res
    }

    pub fn build_query(&self,
                       name: &str,
                       referent: &str,
                       restrictions: &Option<serde_json::Value>,
                       extra_query: Option<&ProofRequestExtraQuery>) -> IndyResult<String> {
        trace!("build_query >>> name: {:?}, referent: {:?}, restrictions: {:?}, extra_query: {:?}", name, referent, restrictions, extra_query);

        let mut sub_queries: Vec<serde_json::Value> = vec![];

        sub_queries.push(serde_json::Value::Object(serde_map!(
            format!("attr::{}::marker", &attr_common_view(name)) => serde_json::Value::String(ATTRIBUTE_EXISTENCE_MARKER.to_string())
        )));

        match restrictions.as_ref() {
            // Convert old restrictions format to valid wql
            Some(&serde_json::Value::Array(ref array)) => {
                // skip Null's
                let mut res: Vec<serde_json::Value> = Vec::new();
                for sub_query in array {
                    let sub_query = sub_query.as_object()
                        .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Restriction is invalid"))?
                        .clone()
                        .into_iter()
                        .filter(|&(_, ref v)| !v.is_null())
                        .collect();
                    res.push(serde_json::Value::Object(sub_query));
                }

                if !res.is_empty() {
                    sub_queries.push(serde_json::Value::Object(serde_map!(
                    "$or".to_string() => serde_json::Value::Array(res)
                    )));
                }
            }
            Some(&serde_json::Value::Object(ref object)) => {
                sub_queries.push(serde_json::Value::Object(object.clone()));
            }
            None => {}
            _ => {
                return Err(err_msg(IndyErrorKind::InvalidStructure, "Restriction is invalid"));
            }
        };

        if let Some(q) = extra_query.as_ref().and_then(|ex_query| ex_query.get(referent)) {
            sub_queries.push(serde_json::Value::Object(q.clone()));
        }

        let mut query: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
        query.insert("$and".to_string(), sub_queries);

        let res = serde_json::to_string(&query)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize Query")?;

        trace!("build_query <<< res: {:?}", res);

        Ok(res)
    }

    pub fn attribute_satisfy_predicate(&self,
                                       predicate: &PredicateInfo,
                                       attribute_value: &str) -> IndyResult<bool> {
        trace!("attribute_satisfy_predicate >>> predicate: {:?}, attribute_value: {:?}", predicate, attribute_value);

        let res = match predicate.p_type {
            PredicateTypes::GE => {
                let attribute_value = attribute_value.parse::<i32>()
                    .to_indy(IndyErrorKind::InvalidStructure, format!("Credential attribute value \"{:?}\" is invalid", attribute_value))?;
                Ok(attribute_value >= predicate.p_value)
            }
            PredicateTypes::GT => {
                let attribute_value = attribute_value.parse::<i32>()
                    .to_indy(IndyErrorKind::InvalidStructure, format!("Credential attribute value \"{:?}\" is invalid", attribute_value))?;
                Ok(attribute_value > predicate.p_value)
            }
            PredicateTypes::LE => {
                let attribute_value = attribute_value.parse::<i32>()
                    .to_indy(IndyErrorKind::InvalidStructure, format!("Credential attribute value \"{:?}\" is invalid", attribute_value))?;
                Ok(attribute_value <= predicate.p_value)
            }
            PredicateTypes::LT => {
                let attribute_value = attribute_value.parse::<i32>()
                    .to_indy(IndyErrorKind::InvalidStructure, format!("Credential attribute value \"{:?}\" is invalid", attribute_value))?;
                Ok(attribute_value < predicate.p_value)
            }
        };

        trace!("attribute_satisfy_predicate <<< res: {:?}", res);
        res
    }

    fn _update_requested_proof(&self, req_attrs_for_credential: Vec<RequestedAttributeInfo>,
                               req_predicates_for_credential: Vec<RequestedPredicateInfo>,
                               proof_req: &ProofRequest,
                               credential: &Credential,
                               sub_proof_index: i32,
                               requested_proof: &mut RequestedProof) -> IndyResult<()> {
        trace!("_update_requested_proof >>> req_attrs_for_credential: {:?}, req_predicates_for_credential: {:?}, proof_req: {:?}, credential: {:?}, \
               sub_proof_index: {:?}, requested_proof: {:?}",
               req_attrs_for_credential, req_predicates_for_credential, proof_req, credential, sub_proof_index, requested_proof);

        for attr_info in req_attrs_for_credential {
            if attr_info.revealed {
                let attribute = &proof_req.requested_attributes[&attr_info.attr_referent];
                let attribute_values =
                    self.get_credential_values_for_attribute(&credential.values, &attribute.name)
                        .ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, format!("Credential value not found for attribute {:?}", attribute.name)))?;

                requested_proof.revealed_attrs.insert(attr_info.attr_referent,
                                                      RevealedAttributeInfo {
                                                          sub_proof_index,
                                                          raw: attribute_values.raw,
                                                          encoded: attribute_values.encoded,
                                                      });
            } else {
                requested_proof.unrevealed_attrs.insert(attr_info.attr_referent, SubProofReferent { sub_proof_index });
            }
        }

        for predicate_info in req_predicates_for_credential {
            requested_proof.predicates.insert(predicate_info.predicate_referent, SubProofReferent { sub_proof_index });
        }

        trace!("_update_requested_proof <<<");

        Ok(())
    }

    fn _build_sub_proof_request(req_attrs_for_credential: &[RequestedAttributeInfo],
                                req_predicates_for_credential: &[RequestedPredicateInfo]) -> IndyResult<SubProofRequest> {
        trace!("_build_sub_proof_request <<< req_attrs_for_credential: {:?}, req_predicates_for_credential: {:?}",
               req_attrs_for_credential, req_predicates_for_credential);

        let mut sub_proof_request_builder = CryptoVerifier::new_sub_proof_request_builder()?;

        for attr in req_attrs_for_credential {
            if attr.revealed {
                sub_proof_request_builder.add_revealed_attr(&attr_common_view(&attr.attr_info.name))?
            }
        }

        for predicate in req_predicates_for_credential {
            let p_type = format!("{}", predicate.predicate_info.p_type);

            sub_proof_request_builder.add_predicate(&attr_common_view(&predicate.predicate_info.name), &p_type, predicate.predicate_info.p_value)?;
        }

        let sub_proof_request = sub_proof_request_builder.finalize()?;

        trace!("_build_sub_proof_request <<< sub_proof_request: {:?}", sub_proof_request);


        Ok(sub_proof_request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCHEMA_ID: &str = "did:2:gvt:1.0";
    const SCHEMA_ISSUER_DID: &str = "did";
    const SCHEMA_NAME: &str = "gvt";
    const SCHEMA_VERSION: &str = "1.0";
    const ISSUER_DID: &str = "did";
    const CRED_DEF_ID: &str = "did:3:CL:did:2:gvt:1.0";
    const REV_REG_ID: &str = "did:4:did:3:CL:did:2:gvt:1.0:CL_ACCUM:TAG_1";
    const NO_REV_REG_ID: &str = "None";

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {
            {
                let mut map = ::std::collections::HashMap::new();
                $(
                    map.insert($key, $val);
                )*
                map
            }
        }
    }

    mod build_credential_tags {
        use super::*;

        fn _credential() -> Credential {
            // note that encoding is not standardized by Indy except that 32-bit integers are encoded as themselves. IS-786
            // so Alex -> 12345 is an application choice while 25 -> 25 is not
            let mut attr_values: HashMap<String, AttributeValues> = HashMap::new();
            attr_values.insert("name".to_string(), AttributeValues { raw: "Alex".to_string(), encoded: "12345".to_string() });
            attr_values.insert("age".to_string(), AttributeValues { raw: "25".to_string(), encoded: "25".to_string() });

            serde_json::from_str::<Credential>(
                &json!({
                    "schema_id": SCHEMA_ID,
                    "cred_def_id": CRED_DEF_ID,
                    "values": attr_values,
                    "signature": json!({
                        "p_credential": json!({"m_2": "0","a": "0","e": "0","v": "0"})
                    }),
                    "signature_correctness_proof": json!({"se":"0", "c":"0"})
                }).to_string()
            ).unwrap()
        }

        #[test]
        fn build_credential_tags_works() {
            let ps = Prover::new();
            let tags = ps.build_credential_tags(&_credential(), None);

            let expected_tags: HashMap<String, String> = hashmap!(
                    "schema_id".to_string() => SCHEMA_ID.to_string(),
                    "schema_issuer_did".to_string() => SCHEMA_ISSUER_DID.to_string(),
                    "schema_name".to_string() => SCHEMA_NAME.to_string(),
                    "schema_version".to_string() => SCHEMA_VERSION.to_string(),
                    "issuer_did".to_string() => ISSUER_DID.to_string(),
                    "cred_def_id".to_string() => CRED_DEF_ID.to_string(),
                    "rev_reg_id".to_string() => NO_REV_REG_ID.to_string(),
                    "attr::name::marker".to_string() => ATTRIBUTE_EXISTENCE_MARKER.to_string(),
                    "attr::name::value".to_string() => "Alex".to_string(),
                    "attr::age::marker".to_string() => ATTRIBUTE_EXISTENCE_MARKER.to_string(),
                    "attr::age::value".to_string() => "25".to_string()
                 );

            assert_eq!(expected_tags, tags)
        }

        #[test]
        fn build_credential_tags_works_for_catpol() {
            let ps = Prover::new();
            let catpol = CredentialAttrTagPolicy::from(vec!(String::from("name")));
            let tags = ps.build_credential_tags(&_credential(), Some(catpol).as_ref());

            let expected_tags: HashMap<String, String> = hashmap!(
                    "schema_id".to_string() => SCHEMA_ID.to_string(),
                    "schema_issuer_did".to_string() => SCHEMA_ISSUER_DID.to_string(),
                    "schema_name".to_string() => SCHEMA_NAME.to_string(),
                    "schema_version".to_string() => SCHEMA_VERSION.to_string(),
                    "issuer_did".to_string() => ISSUER_DID.to_string(),
                    "cred_def_id".to_string() => CRED_DEF_ID.to_string(),
                    "rev_reg_id".to_string() => NO_REV_REG_ID.to_string(),
                    "attr::name::marker".to_string() => ATTRIBUTE_EXISTENCE_MARKER.to_string(),
                    "attr::name::value".to_string() => "Alex".to_string()
                 );

            assert_eq!(expected_tags, tags)
        }

        #[test]
        fn build_credential_tags_works_for_rev_reg_id() {
            let ps = Prover::new();
            let mut credential = _credential();
            credential.rev_reg_id = Some(REV_REG_ID.to_string());
            let tags = ps.build_credential_tags(&credential, None);

            let expected_tags: HashMap<String, String> = hashmap!(
                    "schema_id".to_string() => SCHEMA_ID.to_string(),
                    "schema_issuer_did".to_string() => SCHEMA_ISSUER_DID.to_string(),
                    "schema_name".to_string() => SCHEMA_NAME.to_string(),
                    "schema_version".to_string() => SCHEMA_VERSION.to_string(),
                    "issuer_did".to_string() => ISSUER_DID.to_string(),
                    "cred_def_id".to_string() => CRED_DEF_ID.to_string(),
                    "rev_reg_id".to_string() => REV_REG_ID.to_string(),
                    "attr::name::marker".to_string() => ATTRIBUTE_EXISTENCE_MARKER.to_string(),
                    "attr::name::value".to_string() => "Alex".to_string(),
                    "attr::age::marker".to_string() => ATTRIBUTE_EXISTENCE_MARKER.to_string(),
                    "attr::age::value".to_string() => "25".to_string()
                 );

            assert_eq!(expected_tags, tags)
        }
    }

    mod build_query {
        use super::*;

        const ATTR_NAME: &str = "name";
        const ATTR_REFERENT: &str = "attr_1";

        fn _value(json: &str) -> serde_json::Value {
            serde_json::from_str::<serde_json::Value>(json).unwrap()
        }

        #[test]
        fn build_query_works() {
            let ps = Prover::new();
            let query = ps.build_query(ATTR_NAME, ATTR_REFERENT, &None, None).unwrap();
            let expected_query = json!({
                "$and": vec![
                    json!({
                        "attr::name::marker": ATTRIBUTE_EXISTENCE_MARKER
                    })
                ]
            });
            assert_eq!(expected_query, _value(&query));
        }

        #[test]
        fn build_query_works_for_restriction() {
            let ps = Prover::new();

            let restriction = json!({"schema_id": SCHEMA_ID, "cred_def_id": CRED_DEF_ID});
            let query = ps.build_query(ATTR_NAME, ATTR_REFERENT, &Some(restriction), None).unwrap();

            let expected_query = json!({
                "$and": vec![
                    json!({
                        "attr::name::marker": ATTRIBUTE_EXISTENCE_MARKER
                    }),
                    json!({
                        "schema_id": SCHEMA_ID,
                        "cred_def_id": CRED_DEF_ID
                    })
                ]
            });

            assert_eq!(expected_query, _value(&query));
        }

        #[test]
        fn build_query_works_for_empty_restrictions() {
            let ps = Prover::new();
            let query = ps.build_query(ATTR_NAME, ATTR_REFERENT, &Some(json!([])), None).unwrap();
            let expected_query = json!({
                "$and": vec![
                    json!({
                        "attr::name::marker": ATTRIBUTE_EXISTENCE_MARKER
                    })
                ]
            });
            assert_eq!(expected_query, _value(&query));
        }

        #[test]
        fn build_query_works_for_extra_query() {
            let ps = Prover::new();

            let extra_query: ProofRequestExtraQuery = hashmap!(
                ATTR_REFERENT.to_string() =>
                    serde_map!(
                        "name".to_string() => serde_json::Value::String("Alex".to_string())
                    )
            );

            let query = ps.build_query(ATTR_NAME, ATTR_REFERENT, &None, Some(&extra_query)).unwrap();

            let expected_query = json!({
                "$and": vec![
                    json!({
                        "attr::name::marker": ATTRIBUTE_EXISTENCE_MARKER
                    }),
                    json!({
                        "name": "Alex"
                    })
                ]
            });

            assert_eq!(expected_query, _value(&query));
        }

        #[test]
        fn build_query_works_for_mix_restriction_and_extra_query() {
            let ps = Prover::new();

            let restriction = json!({"schema_id": SCHEMA_ID, "cred_def_id": CRED_DEF_ID});

            let extra_query: ProofRequestExtraQuery = hashmap!(
                ATTR_REFERENT.to_string() =>
                    serde_map!(
                        "name".to_string() => serde_json::Value::String("Alex".to_string())
                    )
            );

            let query = ps.build_query(ATTR_NAME, ATTR_REFERENT, &Some(restriction), Some(&extra_query)).unwrap();

            let expected_query = json!({
                "$and": vec![
                    json!({
                        "attr::name::marker": ATTRIBUTE_EXISTENCE_MARKER
                    }),
                    json!({
                        "schema_id": SCHEMA_ID,
                        "cred_def_id": CRED_DEF_ID
                    }),
                    json!({
                        "name": "Alex"
                    })
                ]
            });

            assert_eq!(expected_query, _value(&query));
        }

        #[test]
        fn build_query_works_for_restriction_in_old_format() {
            let ps = Prover::new();

            let restriction_1 = json!({"schema_id": SCHEMA_ID, "cred_def_id": CRED_DEF_ID});
            let restriction_2 = json!({"cred_def_id": CRED_DEF_ID});
            let restirctions = serde_json::Value::Array(vec![restriction_1, restriction_2]);

            let query = ps.build_query(ATTR_NAME, ATTR_REFERENT, &Some(restirctions), None).unwrap();

            let expected_query = json!({
                "$and": vec![
                    json!({
                        "attr::name::marker": ATTRIBUTE_EXISTENCE_MARKER,
                    }),
                    json!({
                        "$or": vec![
                            json!({
                                "schema_id": SCHEMA_ID,
                                "cred_def_id": CRED_DEF_ID,
                            }),
                            json!({
                                "cred_def_id": CRED_DEF_ID,
                            })
                        ]
                    })
                ]
            });

            assert_eq!(expected_query, _value(&query));
        }

        #[test]
        fn build_query_works_for_restriction_in_old_format_with_nulls() {
            let ps = Prover::new();

            let restriction_1 = json!({"schema_id": SCHEMA_ID, "issuer_did": serde_json::Value::Null});
            let restriction_2 = json!({"schema_id":  serde_json::Value::Null, "cred_def_id": CRED_DEF_ID});
            let restirctions = serde_json::Value::Array(vec![restriction_1, restriction_2]);

            let query = ps.build_query(ATTR_NAME, ATTR_REFERENT, &Some(restirctions), None).unwrap();

            let expected_query = json!({
                "$and": vec![
                    json!({
                        "attr::name::marker": ATTRIBUTE_EXISTENCE_MARKER,
                    }),
                    json!({
                        "$or": vec![
                            json!({
                                "schema_id": SCHEMA_ID
                            }),
                            json!({
                                "cred_def_id": CRED_DEF_ID,
                            })
                        ]
                    })
                ]
            });

            assert_eq!(expected_query, _value(&query));
        }

        #[test]
        fn build_query_works_for_extra_query_with_other_referent() {
            let ps = Prover::new();

            let extra_query: ProofRequestExtraQuery = hashmap!(
                "other_attr_referent".to_string() =>
                    serde_map!(
                        "age".to_string() => serde_json::Value::String("25".to_string())
                    )
            );

            let query = ps.build_query(ATTR_NAME, ATTR_REFERENT, &None, Some(&extra_query)).unwrap();

            let expected_query = json!({
                "$and": vec![
                    json!({
                        "attr::name::marker": ATTRIBUTE_EXISTENCE_MARKER
                    })
                ]
            });

            assert_eq!(expected_query, _value(&query));
        }

        #[test]
        fn build_query_works_for_restriction_and_extra_query_contain_or_operator() {
            let ps = Prover::new();

            let restriction = json!({
                "$or": vec![
                    json!({ "schema_id": SCHEMA_ID }),
                    json!({ "schema_id": "schema_id_2" })
                ]
            });


            let extra_query: ProofRequestExtraQuery = hashmap!(
                ATTR_REFERENT.to_string() =>
                    serde_map!(
                        "$or".to_string() => serde_json::Value::Array(vec![
                            json!({ "name": "Alex" }),
                            json!({ "name": "Alexander" })
                        ])
                    )
            );

            let query = ps.build_query(ATTR_NAME, ATTR_REFERENT, &Some(restriction), Some(&extra_query)).unwrap();

            let expected_query = json!({
                "$and": [
                    json!({
                        "attr::name::marker": ATTRIBUTE_EXISTENCE_MARKER
                    }),
                    json!({
                        "$or": vec![
                            json!({ "schema_id": SCHEMA_ID }),
                            json!({ "schema_id": "schema_id_2" })
                        ]
                    }),
                    json!({
                        "$or": vec![
                            json!({ "name": "Alex" }),
                            json!({ "name": "Alexander" })
                        ]
                    })
                ]
            });

            assert_eq!(expected_query, _value(&query));
        }
    }

    mod attribute_satisfy_predicate {
        use super::*;

        fn predicate_info() -> PredicateInfo {
            PredicateInfo {
                name: "age".to_string(),
                p_type: PredicateTypes::GE,
                p_value: 8,
                restrictions: None,
                non_revoked: None,
            }
        }

        #[test]
        fn attribute_satisfy_predicate_works() {
            let ps = Prover::new();
            let res = ps.attribute_satisfy_predicate(&predicate_info(), "10").unwrap();
            assert!(res);
        }

        #[test]
        fn attribute_satisfy_predicate_works_for_false() {
            let ps = Prover::new();
            let res = ps.attribute_satisfy_predicate(&predicate_info(), "5").unwrap();
            assert!(!res);
        }

        #[test]
        fn attribute_satisfy_predicate_works_for_invalid_attribute_value() {
            let ps = Prover::new();
            let res = ps.attribute_satisfy_predicate(&predicate_info(), "string");
            assert_kind!(IndyErrorKind::InvalidStructure, res);
        }
    }

    mod prepare_credentials_for_proving {
        use domain::anoncreds::proof_request::{AttributeInfo, PredicateInfo};
        use domain::anoncreds::requested_credential::RequestedAttribute;

        use super::*;

        const CRED_ID: &str = "8591bcac-ee7d-4bef-ba7e-984696440b30";
        const ATTRIBUTE_REFERENT: &str = "attribute_referent";
        const PREDICATE_REFERENT: &str = "predicate_referent";

        fn _attr_info() -> AttributeInfo {
            AttributeInfo {
                name: "name".to_string(),
                restrictions: None,
                non_revoked: None,
            }
        }

        fn _predicate_info() -> PredicateInfo {
            PredicateInfo {
                name: "age".to_string(),
                p_type: PredicateTypes::GE,
                p_value: 8,
                restrictions: None,
                non_revoked: None,
            }
        }

        fn _proof_req() -> ProofRequest {
            ProofRequest {
                nonce: ursa::cl::new_nonce().unwrap(),
                name: "Job-Application".to_string(),
                version: "0.1".to_string(),
                requested_attributes: hashmap!(
                    ATTRIBUTE_REFERENT.to_string() => _attr_info()
                ),
                requested_predicates: hashmap!(
                    PREDICATE_REFERENT.to_string() => _predicate_info()
                ),
                non_revoked: None,
            }
        }

        fn _req_cred() -> RequestedCredentials {
            RequestedCredentials {
                self_attested_attributes: HashMap::new(),
                requested_attributes: hashmap!(
                    ATTRIBUTE_REFERENT.to_string() => RequestedAttribute{
                        cred_id: CRED_ID.to_string(),
                        timestamp: None,
                        revealed: false,
                    }
                ),
                requested_predicates: hashmap!(
                    PREDICATE_REFERENT.to_string() => ProvingCredentialKey{ cred_id: CRED_ID.to_string(), timestamp: None }
                ),
            }
        }

        #[test]
        fn prepare_credentials_for_proving_works() {
            let req_cred = _req_cred();
            let proof_req = _proof_req();

            let res = Prover::_prepare_credentials_for_proving(&req_cred, &proof_req).unwrap();


            assert_eq!(1, res.len());
            assert!(res.contains_key(&ProvingCredentialKey { cred_id: CRED_ID.to_string(), timestamp: None }));

            let (req_attr_info, req_pred_info) = res.get(&ProvingCredentialKey { cred_id: CRED_ID.to_string(), timestamp: None }).unwrap();
            assert_eq!(1, req_attr_info.len());
            assert_eq!(1, req_pred_info.len());
        }

        #[test]
        fn prepare_credentials_for_proving_works_for_multiple_attributes_with_same_credential() {
            let mut req_cred = _req_cred();
            let mut proof_req = _proof_req();

            req_cred.requested_attributes.insert("attribute_referent_2".to_string(), RequestedAttribute {
                cred_id: CRED_ID.to_string(),
                timestamp: None,
                revealed: false,
            });

            proof_req.requested_attributes.insert("attribute_referent_2".to_string(), AttributeInfo {
                name: "last_name".to_string(),
                restrictions: None,
                non_revoked: None,
            });

            let res = Prover::_prepare_credentials_for_proving(&req_cred, &proof_req).unwrap();

            assert_eq!(1, res.len());
            assert!(res.contains_key(&ProvingCredentialKey { cred_id: CRED_ID.to_string(), timestamp: None }));

            let (req_attr_info, req_pred_info) = res.get(&ProvingCredentialKey { cred_id: CRED_ID.to_string(), timestamp: None }).unwrap();
            assert_eq!(2, req_attr_info.len());
            assert_eq!(1, req_pred_info.len());
        }

        #[test]
        fn prepare_credentials_for_proving_works_for_missed_attribute() {
            let req_cred = _req_cred();
            let mut proof_req = _proof_req();

            proof_req.requested_attributes.clear();

            let res = Prover::_prepare_credentials_for_proving(&req_cred, &proof_req);
            assert_kind!(IndyErrorKind::InvalidStructure, res);
        }

        #[test]
        fn prepare_credentials_for_proving_works_for_missed_predicate() {
            let req_cred = _req_cred();
            let mut proof_req = _proof_req();

            proof_req.requested_predicates.clear();

            let res = Prover::_prepare_credentials_for_proving(&req_cred, &proof_req);
            assert_kind!(IndyErrorKind::InvalidStructure, res);
        }
    }

    mod get_credential_values_for_attribute {
        use super::*;

        fn _attr_values() -> AttributeValues {
            AttributeValues { raw: "Alex".to_string(), encoded: "123".to_string() }
        }

        fn _cred_values() -> HashMap<String, AttributeValues> {
            hashmap!("name".to_string() => _attr_values())
        }

        #[test]
        fn get_credential_values_for_attribute_works() {
            let ps = Prover::new();

            let res = ps.get_credential_values_for_attribute(&_cred_values(), "name").unwrap();
            assert_eq!(_attr_values(), res);
        }

        #[test]
        fn get_credential_values_for_attribute_works_for_requested_attr_different_case() {
            let ps = Prover::new();

            let res = ps.get_credential_values_for_attribute(&_cred_values(), "NAme").unwrap();
            assert_eq!(_attr_values(), res);
        }

        #[test]
        fn get_credential_values_for_attribute_works_for_requested_attr_contains_spaces() {
            let ps = Prover::new();

            let res = ps.get_credential_values_for_attribute(&_cred_values(), "   na me  ").unwrap();
            assert_eq!(_attr_values(), res);
        }

        #[test]
        fn get_credential_values_for_attribute_works_for_cred_values_different_case() {
            let ps = Prover::new();

            let cred_values = hashmap!("NAME".to_string() => _attr_values());

            let res = ps.get_credential_values_for_attribute(&cred_values, "name").unwrap();
            assert_eq!(_attr_values(), res);
        }

        #[test]
        fn get_credential_values_for_attribute_works_for_cred_values_contains_spaces() {
            let ps = Prover::new();

            let cred_values = hashmap!("    name    ".to_string() => _attr_values());

            let res = ps.get_credential_values_for_attribute(&cred_values, "name").unwrap();
            assert_eq!(_attr_values(), res);
        }

        #[test]
        fn get_credential_values_for_attribute_works_for_cred_values_and_requested_attr_contains_spaces() {
            let ps = Prover::new();

            let cred_values = hashmap!("    name    ".to_string() => _attr_values());

            let res = ps.get_credential_values_for_attribute(&cred_values, "            name            ").unwrap();
            assert_eq!(_attr_values(), res);
        }
    }
}
