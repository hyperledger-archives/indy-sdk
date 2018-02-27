extern crate indy_crypto;

use errors::common::CommonError;
use errors::anoncreds::AnoncredsError;
use services::anoncreds::types::*;
use std::collections::HashMap;
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
        Ok(CryptoProver::new_master_secret()?)
    }

    pub fn new_credential_request(&self,
                                  credential_def: &CredentialDefinition,
                                  master_secret: &MasterSecret,
                                  credential_offer: &CredentialOffer) -> Result<(BlindedMasterSecret,
                                                                                 MasterSecretBlindingData,
                                                                                 BlindedMasterSecretCorrectnessProof), CommonError> {
        info!("new_credential_request >>> credential_def: {:?}, master_secret: {:?}, credential_offer: {:?}",
              credential_def, master_secret, credential_offer);

        let credential_pub_key = CredentialPublicKey::build_from_parts(&credential_def.value.primary, credential_def.value.revocation.as_ref())?;

        let (blinded_ms, master_secret_blinding_data, blinded_ms_correctness_proof) =
            CryptoProver::blind_master_secret(&credential_pub_key,
                                              &credential_offer.key_correctness_proof,
                                              &master_secret,
                                              &credential_offer.nonce)?;

        info!("new_credential_request <<< blinded_ms: {:?}, master_secret_blinding_data: {:?}, blinded_ms_correctness_proof: {:?}",
              blinded_ms, master_secret_blinding_data, blinded_ms_correctness_proof);

        Ok((blinded_ms, master_secret_blinding_data, blinded_ms_correctness_proof))
    }

    pub fn process_credential(&self,
                              credential: &mut Credential,
                              credential_request_metadata: &CredentialRequestMetadata,
                              master_secret: &MasterSecret,
                              credential_def: &CredentialDefinition,
                              rev_reg_def: Option<&RevocationRegistryDefinitionValue>,
                              rev_reg: Option<&RevocationRegistry>,
                              witness: Option<&Witness>) -> Result<(), CommonError> {
        info!("process_credential >>> credential: {:?}, credential_request_metadata: {:?}, master_secret: {:?}, credential_def: {:?}, rev_reg_def: {:?}, \
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
                                                   rev_reg_def.as_ref().map(|r_reg_def| &r_reg_def.public_keys.accum_key),
                                                   rev_reg,
                                                   witness)?;

        info!("process_credential <<< ");

        Ok(())
    }

    pub fn get_credentials_for_proof_req(&self,
                                         proof_request: &ProofRequest,
                                         credentials: &Vec<CredentialInfo>) -> Result<CredentialsForProofRequest, CommonError> {
        info!("get_credentials_for_proof_req >>> proof_request: {:?}, credentials: {:?}", proof_request, credentials);

        let mut found_attributes: HashMap<String, Vec<CredentialInfo>> = HashMap::new();
        let mut found_predicates: HashMap<String, Vec<CredentialInfo>> = HashMap::new();

        for (attr_id, requested_attr) in &proof_request.requested_attrs {
            let credentials_for_attribute = credentials
                .iter()
                .filter(|credential|
                    Prover::_credential_value_for_attribute(&credential.attrs, &requested_attr.name).is_some() &&
                        self._credential_satisfy_restrictions(credential, &requested_attr.restrictions))
                .cloned()
                .collect::<Vec<CredentialInfo>>();

            found_attributes.insert(attr_id.clone(), credentials_for_attribute);
        }

        for (predicate_id, requested_predicate) in &proof_request.requested_predicates {
            let mut credentials_for_predicate: Vec<CredentialInfo> = Vec::new();

            for credential in credentials {
                let mut satisfy = match Prover::_credential_value_for_attribute(&credential.attrs, &requested_predicate.attr_name) {
                    Some(attribute_value) => Prover::_attribute_satisfy_predicate(&requested_predicate, &attribute_value)?,
                    None => false
                };

                satisfy = satisfy && self._credential_satisfy_restrictions(credential, &requested_predicate.restrictions);

                if satisfy { credentials_for_predicate.push(credential.clone()); }
            }

            found_predicates.insert(predicate_id.clone(), credentials_for_predicate);
        }

        let credentials_for_proof_request = CredentialsForProofRequest {
            attrs: found_attributes,
            predicates: found_predicates
        };

        info!("get_credentials_for_proof_req <<< credentials_for_proof_requerst: {:?}", credentials_for_proof_request);
        Ok(credentials_for_proof_request)
    }

    pub fn create_proof(&self,
                        credentials: &HashMap<String, Credential>,
                        proof_req: &ProofRequest,
                        requested_credentials: &RequestedCredentials,
                        master_secret: &MasterSecret,
                        schemas: &HashMap<String, Schema>,
                        credential_defs: &HashMap<String, CredentialDefinition>,
                        rev_regs: &HashMap<String, RevocationRegistry>,
                        witnesses: &HashMap<String, Witness>) -> Result<FullProof, AnoncredsError> {
        info!("create_proof >>> credentials: {:?}, proof_req: {:?}, schemas: {:?}, credential_defs: {:?}, rev_regs: {:?}, \
               requested_credentials: {:?}, master_secret: {:?}",
              credentials, proof_req, schemas, credential_defs, rev_regs, requested_credentials, master_secret);

        let mut proof_builder = CryptoProver::new_proof_builder()?;

        let mut identifiers: HashMap<String, Identifier> = HashMap::new();

        for (referent, credential) in credentials {
            let schema = schemas.get(referent.as_str())
                .ok_or(CommonError::InvalidStructure(format!("Schema not found")))?;
            let credential_definition: &CredentialDefinition = credential_defs.get(referent.as_str())
                .ok_or(CommonError::InvalidStructure(format!("CredentialDefinition not found")))?;

            let (rev_reg, witness) = if credential_definition.value.revocation.is_some() {
                let rev_reg = Some(rev_regs.get(referent.as_str())
                    .ok_or(CommonError::InvalidStructure(format!("RevocationRegistryEntry not found")))?);
                let witness = Some(witnesses.get(referent.as_str())
                    .ok_or(CommonError::InvalidState(format!("Witness not found")))?);
                (rev_reg, witness)
            } else { (None, None) };

            let credential_pub_key = CredentialPublicKey::build_from_parts(&credential_definition.value.primary, credential_definition.value.revocation.as_ref())?;

            let attrs_for_credential = Prover::_get_revealed_attributes_for_credential(referent.as_str(), requested_credentials, proof_req)?;
            let predicates_for_credential = Prover::_get_predicates_for_credential(referent.as_str(), requested_credentials, proof_req)?;

            let credential_schema = build_credential_schema(&schema.attr_names)?;
            let credential_values = build_credential_values(&credential.values)?;
            let sub_proof_request = build_sub_proof_request(&attrs_for_credential, &predicates_for_credential)?;

            proof_builder.add_sub_proof_request(referent.as_str(),
                                                &sub_proof_request,
                                                &credential_schema,
                                                &credential.signature,
                                                &credential_values,
                                                &credential_pub_key,
                                                rev_reg,
                                                witness)?;

            identifiers.insert(referent.to_string(), Identifier {
                schema_id: credential.schema_id.clone(),
                cred_def_id: credential.cred_def_id.clone(),
                rev_reg_id: credential.rev_reg_id.clone()
            });
        }

        let proof = proof_builder.finalize(&proof_req.nonce, &master_secret)?;

        let (revealed_attrs, unrevealed_attrs) =
            Prover::_split_attributes(&proof_req, requested_credentials, credentials)?;

        let requested_proof = RequestedProof {
            self_attested_attrs: requested_credentials.self_attested_attributes.clone(),
            revealed_attrs,
            unrevealed_attrs,
            predicates: requested_credentials.requested_predicates.clone()
        };

        let full_proof = FullProof {
            proof,
            requested_proof,
            identifiers
        };

        info!("create_proof <<< full_proof: {:?}", full_proof);

        Ok(full_proof)
    }

    fn _credential_value_for_attribute(credential_attrs: &HashMap<String, String>,
                                       requested_attr: &str) -> Option<String> {
        let _attr_common_view = |attr: &str|
            attr.replace(" ", "").to_lowercase();

        credential_attrs.iter()
            .find(|&(ref key, _)| _attr_common_view(key) == _attr_common_view(&requested_attr))
            .map(|(_, value)| value.to_string())
    }

    fn _credential_satisfy_restrictions(&self,
                                        credential_info: &CredentialInfo,
                                        restrictions: &Option<Vec<Filter>>) -> bool {
        info!("_credential_satisfy_restrictions >>> credential_info: {:?}, restrictions: {:?}", credential_info, restrictions);

        let res = match restrictions {
            &Some(ref restrictions) => restrictions.iter().any(|restriction|
                self.satisfy_restriction(credential_info, &restriction)),
            &None => true
        };

        info!("_credential_satisfy_restrictions <<< res: {:?}", res);

        res
    }

    pub fn satisfy_restriction<T>(&self,
                                  object: &T,
                                  restriction: &Filter) -> bool where T: Filtering {
        info!("satisfy_restriction >>> restriction: {:?}", restriction);

        let mut res = true;
        {
            let mut check_condition = |expected: Option<&str>, actual: &str|
                if let Some(ex) = expected {
                    res = res && actual.eq(ex);
                };

            check_condition(restriction.schema_id.as_ref().map(String::as_str), &object.schema_id());
            check_condition(restriction.cred_def_id.as_ref().map(String::as_str), &object.cred_def_id());

//            check_condition(restriction.issuer_did.as_ref().map(String::as_str), &object.issuer_did());
//            if let Some(ref schema_key) = restriction.schema_key {
//                check_condition(schema_key.name.as_ref().map(String::as_str), &object.schema_key().name);
//                check_condition(schema_key.version.as_ref().map(String::as_str), &object.schema_key().version);
//                check_condition(schema_key.did.as_ref().map(String::as_str), &object.schema_key().did);
//            }
        }

        info!("satisfy_restriction >>> res: {:?}", res);

        res
    }

    fn _attribute_satisfy_predicate(predicate: &PredicateInfo,
                                    attribute_value: &String) -> Result<bool, CommonError> {
        info!("_attribute_satisfy_predicate >>> predicate: {:?}, attribute_value: {:?}", predicate, attribute_value);

        let res = match predicate.p_type.as_str() {
            ">=" => Ok({
                let attribute_value = attribute_value.parse::<i32>()
                    .map_err(|err| CommonError::InvalidStructure(format!("Invalid format of predicate attribute: {}", attribute_value)))?;
                attribute_value >= predicate.value
            }),
            _ => return Err(CommonError::InvalidStructure(format!("Invalid predicate type: {:?}", predicate.p_type)))
        };

        info!("_attribute_satisfy_predicate <<< res: {:?}", res);

        res
    }

    fn _get_revealed_attributes_for_credential(referent: &str,
                                               requested_credentials: &RequestedCredentials,
                                               proof_req: &ProofRequest) -> Result<Vec<String>, CommonError> {
        info!("_get_revealed_attributes_for_credential >>> referent: {:?}, requested_credentials: {:?}, proof_req: {:?}",
              referent, requested_credentials, proof_req);

        let revealed_attrs_for_credential = requested_credentials.requested_attrs
            .iter()
            .filter(|&(attr_referent, &(ref requested_referent, ref revealed))|
                referent.eq(requested_referent) && revealed.clone() && proof_req.requested_attrs.contains_key(attr_referent))
            .map(|(attr_referent, _)|
                proof_req.requested_attrs[attr_referent].name.clone())
            .collect::<Vec<String>>();

        info!("_get_revealed_attributes_for_credential <<< revealed_attrs_for_credential: {:?}", revealed_attrs_for_credential);

        Ok(revealed_attrs_for_credential)
    }

    fn _get_predicates_for_credential(referent: &str,
                                      requested_credentials: &RequestedCredentials,
                                      proof_req: &ProofRequest) -> Result<Vec<PredicateInfo>, CommonError> {
        info!("_get_predicates_for_credential >>> referent: {:?}, requested_credentials: {:?}, proof_req: {:?}",
              referent, requested_credentials, proof_req);

        let predicates_for_credential = requested_credentials.requested_predicates
            .iter()
            .filter(|&(attr_referent, requested_referent)|
                referent.eq(requested_referent) && proof_req.requested_predicates.contains_key(attr_referent))
            .map(|(attr_referent, _)|
                proof_req.requested_predicates[attr_referent].clone())
            .collect::<Vec<PredicateInfo>>();

        info!("_get_predicates_for_credential <<< predicates_for_credential: {:?}", predicates_for_credential);

        Ok(predicates_for_credential)
    }


    pub fn _split_attributes(proof_req: &ProofRequest,
                             requested_credentials: &RequestedCredentials,
                             credentials: &HashMap<String, Credential>) -> Result<(HashMap<String, (String, String, String)>,
                                                                                   HashMap<String, String>), CommonError> {
        info!("_split_attributes >>> proof_req: {:?}, requested_credentials: {:?}, credentials: {:?}",
              proof_req, requested_credentials, credentials);

        let mut revealed_attrs: HashMap<String, (String, String, String)> = HashMap::new();
        let mut unrevealed_attrs: HashMap<String, String> = HashMap::new();

        for (attr_referent, &(ref referent, ref revealed)) in &requested_credentials.requested_attrs {
            let credential = &credentials[referent];
            let attribute = &proof_req.requested_attrs[attr_referent];

            if revealed.clone() {
                let attribute_values = &credential.values[&attribute.name];
                let raw_value = &attribute_values[0];
                let encoded_value = &attribute_values[1];

                revealed_attrs.insert(attr_referent.clone(), (referent.clone(), raw_value.clone(), encoded_value.clone()));
            } else {
                unrevealed_attrs.insert(attr_referent.clone(), referent.clone());
            }
        }

        info!("_split_attributes <<< revealed_attrs: {:?}, unrevealed_attrs: {:?}", revealed_attrs, unrevealed_attrs);

        Ok((revealed_attrs, unrevealed_attrs))
    }
}