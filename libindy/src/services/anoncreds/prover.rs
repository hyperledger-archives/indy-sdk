extern crate indy_crypto;
extern crate uuid;

use errors::common::CommonError;
use errors::anoncreds::AnoncredsError;
use services::anoncreds::types::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use services::anoncreds::types::{CredentialInfo, RequestedCredentials, ProofRequest, PredicateInfo, Identifier};

use self::indy_crypto::cl::*;
use self::indy_crypto::cl::prover::Prover as CryptoProver;
use services::anoncreds::helpers::*;

pub struct Prover {}

impl Prover {
    pub fn new() -> Prover {
        Prover {}
    }

    pub fn new_master_secret(&self) -> Result<MasterSecret, CommonError> {
        trace!("new_master_secret >>> ");

        let master_secret = CryptoProver::new_master_secret()?;

        trace!("new_master_secret <<< master_secret: {:?} ", master_secret);

        Ok(master_secret)
    }

    pub fn new_credential_request(&self,
                                  credential_def: &CredentialDefinition,
                                  master_secret: &MasterSecret,
                                  credential_offer: &CredentialOffer) -> Result<(BlindedMasterSecret,
                                                                                 MasterSecretBlindingData,
                                                                                 BlindedMasterSecretCorrectnessProof), CommonError> {
        trace!("new_credential_request >>> credential_def: {:?}, master_secret: {:?}, credential_offer: {:?}",
               credential_def, master_secret, credential_offer);

        let credential_pub_key = CredentialPublicKey::build_from_parts(&credential_def.value.primary, credential_def.value.revocation.as_ref())?;

        let (blinded_ms, master_secret_blinding_data, blinded_ms_correctness_proof) =
            CryptoProver::blind_master_secret(&credential_pub_key,
                                              &credential_offer.key_correctness_proof,
                                              &master_secret,
                                              &credential_offer.nonce)?;

        trace!("new_credential_request <<< blinded_ms: {:?}, master_secret_blinding_data: {:?}, blinded_ms_correctness_proof: {:?}",
               blinded_ms, master_secret_blinding_data, blinded_ms_correctness_proof);

        Ok((blinded_ms, master_secret_blinding_data, blinded_ms_correctness_proof))
    }

    pub fn process_credential(&self,
                              credential: &mut Credential,
                              credential_request_metadata: &CredentialRequestMetadata,
                              master_secret: &MasterSecret,
                              credential_def: &CredentialDefinition,
                              rev_reg_def: Option<&RevocationRegistryDefinition>,
                              rev_reg: Option<&RevocationRegistry>,
                              witness: Option<&Witness>) -> Result<(), CommonError> {
        trace!("process_credential >>> credential: {:?}, credential_request_metadata: {:?}, master_secret: {:?}, credential_def: {:?}, rev_reg_def: {:?}, \
               rev_reg: {:?}, witness: {:?}", credential, credential_request_metadata, master_secret, credential_def, rev_reg_def, rev_reg, witness);

        let credential_pub_key = CredentialPublicKey::build_from_parts(&credential_def.value.primary, credential_def.value.revocation.as_ref())?;
        let credential_values = build_credential_values(&credential.values)?;

        CryptoProver::process_credential_signature(&mut credential.signature,
                                                   &credential_values,
                                                   &credential.signature_correctness_proof,
                                                   &credential_request_metadata.master_secret_blinding_data,
                                                   &master_secret,
                                                   &credential_pub_key,
                                                   &credential_request_metadata.nonce,
                                                   rev_reg_def.as_ref().map(|r_reg_def| &r_reg_def.value.public_keys.accum_key),
                                                   rev_reg,
                                                   witness)?;

        trace!("process_credential <<< ");

        Ok(())
    }

    pub fn get_credentials_for_proof_req(&self,
                                         proof_request: &ProofRequest,
                                         credentials: &mut Vec<CredentialInfo>) -> Result<CredentialsForProofRequest, CommonError> {
        trace!("get_credentials_for_proof_req >>> proof_request: {:?}, credentials: {:?}", proof_request, credentials);

        let mut credentials_for_proof_request = CredentialsForProofRequest {
            attrs: HashMap::new(),
            predicates: HashMap::new()
        };

        for (attr_id, requested_attr) in &proof_request.requested_attrs {
            let credentials_for_attribute = credentials
                .iter_mut()
                .filter(|credential|
                    Prover::_credential_value_for_attribute(&credential.attrs, &requested_attr.name).is_some() &&
                        self._credential_satisfy_restrictions(credential, &requested_attr.restrictions))
                .map(|credential| {
                    let freshness = Prover::_get_freshness(proof_request.freshness, requested_attr.freshness);
                    RequestedCredential { cred_info: credential.clone(), freshness }
                })
                .collect::<Vec<RequestedCredential>>();

            credentials_for_proof_request.attrs.insert(attr_id.clone(), credentials_for_attribute);
        }

        for (predicate_id, requested_predicate) in &proof_request.requested_predicates {
            let mut credentials_for_predicate: Vec<RequestedCredential> = Vec::new();

            for credential in credentials.iter_mut() {
                let satisfy = match Prover::_credential_value_for_attribute(&credential.attrs, &requested_predicate.attr_name) {
                    Some(attribute_value) => Prover::_attribute_satisfy_predicate(&requested_predicate, &attribute_value)?,
                    None => false
                } && self._credential_satisfy_restrictions(credential, &requested_predicate.restrictions);

                if satisfy {
                    let freshness = Prover::_get_freshness(proof_request.freshness, requested_predicate.freshness);
                    credentials_for_predicate.push(RequestedCredential { cred_info: credential.clone(), freshness });
                }
            }

            credentials_for_proof_request.predicates.insert(predicate_id.clone(), credentials_for_predicate);
        }

        trace!("get_credentials_for_proof_req <<< credentials_for_proof_request: {:?}", credentials_for_proof_request);
        Ok(credentials_for_proof_request)
    }

    pub fn create_proof(&self,
                        credentials: &HashMap<String, Credential>,
                        proof_req: &ProofRequest,
                        requested_credentials: &RequestedCredentials,
                        master_secret: &MasterSecret,
                        schemas: &HashMap<String, Schema>,
                        credential_defs: &HashMap<String, CredentialDefinition>,
                        rev_states: &HashMap<String, HashMap<u64, RevocationState>>) -> Result<FullProof, AnoncredsError> {
        trace!("create_proof >>> credentials: {:?}, proof_req: {:?}, requested_credentials: {:?}, master_secret: {:?}, schemas: {:?}, credential_defs: {:?}, rev_states: {:?}",
               credentials, proof_req, requested_credentials, master_secret, schemas, credential_defs, rev_states);

        let mut proof_builder = CryptoProver::new_proof_builder()?;

        let mut identifiers: HashMap<String, Identifier> = HashMap::new();

        let mut requested_proof = RequestedProof {
            self_attested_attrs: requested_credentials.self_attested_attributes.clone(),
            revealed_attrs: HashMap::new(),
            unrevealed_attrs: HashMap::new(),
            predicates: HashMap::new()
        };

        let credentials_for_proving = Prover::_prepare_credentials_for_proving(requested_credentials, proof_req);

        for (cred_key, &(ref req_attrs_for_cred, ref req_predicates_for_cred)) in credentials_for_proving.iter() {
            let schema = schemas.get(cred_key.cred_id.as_str())
                .ok_or(CommonError::InvalidStructure(format!("Schema not found")))?;
            let credential_definition: &CredentialDefinition = credential_defs.get(cred_key.cred_id.as_str())
                .ok_or(CommonError::InvalidStructure(format!("CredentialDefinition not found")))?;
            let credential: &Credential = credentials.get(cred_key.cred_id.as_str())
                .ok_or(CommonError::InvalidStructure(format!("Credential not found")))?;

            let rev_state = if credential_definition.value.revocation.is_some() {
                let timestamp = cred_key.timestamp.ok_or(CommonError::InvalidStructure(format!("Timestamp not found")))?;
                let rev_states_for_timestamp = rev_states.get(cred_key.cred_id.as_str())
                    .ok_or(CommonError::InvalidStructure(format!("RevocationInfo not found")))?;
                Some(rev_states_for_timestamp.get(&timestamp)
                    .ok_or(CommonError::InvalidStructure(format!("RevocationInfo not found")))?)
            } else { None };

            let credential_pub_key = CredentialPublicKey::build_from_parts(&credential_definition.value.primary, credential_definition.value.revocation.as_ref())?;

            let credential_schema = build_credential_schema(&schema.attr_names)?;
            let credential_values = build_credential_values(&credential.values)?;
            let sub_proof_request = Prover::_build_sub_proof_request(req_attrs_for_cred, req_predicates_for_cred)?;

            let sub_proof_id = uuid::Uuid::new_v4().to_string();

            proof_builder.add_sub_proof_request(sub_proof_id.as_str(),
                                                &sub_proof_request,
                                                &credential_schema,
                                                &credential.signature,
                                                &credential_values,
                                                &credential_pub_key,
                                                rev_state.as_ref().map(|r_info| &r_info.rev_reg),
                                                rev_state.as_ref().map(|r_info| &r_info.witness))?;

            identifiers.insert(sub_proof_id.clone(), Identifier {
                schema_id: credential_definition.schema_id.clone(),
                cred_def_id: credential.cred_def_id.clone(),
                rev_reg_id: credential.rev_reg_id.clone(),
                timestamp: cred_key.timestamp.clone()
            });

            Prover::_update_requested_proof(req_attrs_for_cred,
                                            req_predicates_for_cred,
                                            proof_req, credential,
                                            &sub_proof_id,
                                            &mut requested_proof)?;
        }

        let proof = proof_builder.finalize(&proof_req.nonce, &master_secret)?;

        let full_proof = FullProof {
            proof,
            requested_proof,
            identifiers
        };

        trace!("create_proof <<< full_proof: {:?}", full_proof);

        Ok(full_proof)
    }

    fn _get_freshness(global_freshness: Option<u64>, local_freshness: Option<u64>) -> Option<u64> {
        trace!("_get_freshness >>> global_freshness: {:?}, local_freshness: {:?}", global_freshness, local_freshness);

        let freshness = local_freshness.or(global_freshness.or(None));

        trace!("_get_freshness <<< freshness: {:?}", freshness);

        freshness
    }

    pub fn _prepare_credentials_for_proving(requested_credentials: &RequestedCredentials,
                                            proof_req: &ProofRequest) -> HashMap<ProvingCredentialKey, (Vec<RequestedAttributeInfo>, Vec<RequestedPredicateInfo>)> {
        trace!("_prepare_credentials_for_proving >>> requested_credentials: {:?}, proof_req: {:?}", requested_credentials, proof_req);

        let mut credentials_for_proving: HashMap<ProvingCredentialKey, (Vec<RequestedAttributeInfo>, Vec<RequestedPredicateInfo>)> = HashMap::new();

        for (attr_referent, requested_attr) in requested_credentials.requested_attrs.iter() {
            let attr_info = proof_req.requested_attrs.get(attr_referent.as_str()).unwrap();
            let req_attr_info = RequestedAttributeInfo {
                attr_referent: attr_referent.clone(),
                attr_info: attr_info.clone(),
                revealed: requested_attr.revealed.clone()
            };

            match credentials_for_proving.entry(ProvingCredentialKey { cred_id: requested_attr.cred_id.clone(), timestamp: requested_attr.timestamp.clone() }) {
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
            let predicate_info = proof_req.requested_predicates.get(predicate_referent.as_str()).unwrap();
            let req_predicate_info = RequestedPredicateInfo {
                predicate_referent: predicate_referent.clone(),
                predicate_info: predicate_info.clone()
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

        credentials_for_proving
    }

    fn _credential_value_for_attribute(credential_attrs: &HashMap<String, String>,
                                       requested_attr: &str) -> Option<String> {
        trace!("_credential_value_for_attribute >>> credential_attrs: {:?}, requested_attr: {:?}", credential_attrs, requested_attr);

        let res = credential_attrs.iter()
            .find(|&(ref key, _)| attr_common_view(key) == attr_common_view(&requested_attr))
            .map(|(_, value)| value.to_string());

        trace!("_credential_value_for_attribute <<< res: {:?}", res);

        res
    }


    fn _get_credential_values_for_attribute(credential_attrs: &HashMap<String, AttributeValues>,
                                            requested_attr: &str) -> Option<AttributeValues> {
        trace!("_get_credential_values_for_attribute >>> credential_attrs: {:?}, requested_attr: {:?}", credential_attrs, requested_attr);

        let res = credential_attrs.iter()
            .find(|&(ref key, _)| attr_common_view(key) == attr_common_view(&requested_attr))
            .map(|(_, values)| values.clone());

        trace!("_get_credential_values_for_attribute <<< res: {:?}", res);

        res
    }

    fn _credential_satisfy_restrictions(&self,
                                        credential_info: &CredentialInfo,
                                        restrictions: &Option<Vec<Filter>>) -> bool {
        trace!("_credential_satisfy_restrictions >>> credential_info: {:?}, restrictions: {:?}", credential_info, restrictions);

        let res = match restrictions {
            &Some(ref restrictions) => restrictions.iter().any(|restriction|
                self.satisfy_restriction(credential_info, &restriction)),
            &None => true
        };

        trace!("_credential_satisfy_restrictions <<< res: {:?}", res);

        res
    }

    pub fn satisfy_restriction<T>(&self,
                                  object: &T,
                                  restriction: &Filter) -> bool where T: Filtering {
        trace!("satisfy_restriction >>> restriction: {:?}", restriction);

        let mut res = true;
        {
            let mut check_condition = |expected: Option<&str>, actual: &str|
                if let Some(ex) = expected {
                    res = res && actual.eq(ex);
                };

            check_condition(restriction.schema_id.as_ref().map(String::as_str), &object.schema_id());
            check_condition(restriction.schema_name.as_ref().map(String::as_str), &object.schema_name());
            check_condition(restriction.schema_version.as_ref().map(String::as_str), &object.schema_version());
            check_condition(restriction.schema_did.as_ref().map(String::as_str), &object.schema_did());
            check_condition(restriction.issuer_did.as_ref().map(String::as_str), &object.issuer_did());
            check_condition(restriction.cred_def_id.as_ref().map(String::as_str), &object.cred_def_id());
        }

        trace!("satisfy_restriction >>> res: {:?}", res);

        res
    }

    fn _attribute_satisfy_predicate(predicate: &PredicateInfo,
                                    attribute_value: &String) -> Result<bool, CommonError> {
        trace!("_attribute_satisfy_predicate >>> predicate: {:?}, attribute_value: {:?}", predicate, attribute_value);

        let res = match predicate.p_type.as_str() {
            ">=" => Ok({
                let attribute_value = attribute_value.parse::<i32>()
                    .map_err(|_| CommonError::InvalidStructure(format!("Invalid format of predicate attribute: {}", attribute_value)))?;
                attribute_value >= predicate.value
            }),
            _ => return Err(CommonError::InvalidStructure(format!("Invalid predicate type: {:?}", predicate.p_type)))
        };

        trace!("_attribute_satisfy_predicate <<< res: {:?}", res);

        res
    }

    fn _update_requested_proof(req_attrs_for_credential: &Vec<RequestedAttributeInfo>,
                               req_predicates_for_credential: &Vec<RequestedPredicateInfo>,
                               proof_req: &ProofRequest,
                               credential: &Credential,
                               sub_proof_id: &str,
                               requested_proof: &mut RequestedProof) -> Result<(), CommonError> {
        trace!("_update_requested_proof >>> req_attrs_for_credential: {:?}, req_predicates_for_credential: {:?}, proof_req: {:?}, credential: {:?}, \
               sub_proof_id: {:?}, requested_proof: {:?}",
               req_attrs_for_credential, req_predicates_for_credential, proof_req, credential, sub_proof_id, requested_proof);

        for attr_info in req_attrs_for_credential {
            if attr_info.revealed.clone() {
                let attribute = &proof_req.requested_attrs[&attr_info.attr_referent];
                let attribute_values =
                    Prover::_get_credential_values_for_attribute(&credential.values, &attribute.name)
                        .ok_or(CommonError::InvalidStructure(format!("Credential value not found for attribute {:?}", attribute.name)))?;

                requested_proof.revealed_attrs.insert(attr_info.attr_referent.clone(),
                                                      RevealedAttributeInfo {
                                                          referent: sub_proof_id.to_string(),
                                                          raw: attribute_values.raw.clone(),
                                                          encoded: attribute_values.encoded.clone()
                                                      });
            } else {
                requested_proof.unrevealed_attrs.insert(attr_info.attr_referent.clone(), sub_proof_id.to_string());
            }
        }

        for predicate_info in req_predicates_for_credential {
            requested_proof.predicates.insert(predicate_info.predicate_referent.clone(), sub_proof_id.to_string());
        }

        trace!("_update_requested_proof <<<");

        Ok(())
    }

    fn _build_sub_proof_request(req_attrs_for_credential: &Vec<RequestedAttributeInfo>,
                                req_predicates_for_credential: &Vec<RequestedPredicateInfo>) -> Result<SubProofRequest, CommonError> {
        trace!("_build_sub_proof_request <<< req_attrs_for_credential: {:?}, req_predicates_for_credential: {:?}",
               req_attrs_for_credential, req_predicates_for_credential);

        let mut sub_proof_request_builder = verifier::Verifier::new_sub_proof_request_builder()?;

        for attr in req_attrs_for_credential {
            if attr.revealed {
                sub_proof_request_builder.add_revealed_attr(&attr_common_view(&attr.attr_info.name))?
            }
        }

        for predicate in req_predicates_for_credential {
            sub_proof_request_builder.add_predicate(&attr_common_view(&predicate.predicate_info.attr_name), "GE", predicate.predicate_info.value)?;
        }

        let sub_proof_request = sub_proof_request_builder.finalize()?;

        trace!("_build_sub_proof_request <<< sub_proof_request: {:?}", sub_proof_request);


        Ok(sub_proof_request)
    }
}