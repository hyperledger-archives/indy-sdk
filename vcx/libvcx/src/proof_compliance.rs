/*use messages::proofs::proof_message::{ ProofMessage, Attr, Identifier };
use messages::proofs::proof_request::{ Filter, ProofRequestData };
use std::collections::HashMap;
use error::proof::ProofError;


pub fn proof_compliance(request: &ProofRequestData, proof: &ProofMessage) -> Result<(), ProofError> {
    verify_requested_attributes(request, proof)?;
    verify_requested_predicates(request, proof)
}

fn verify_requested_predicates(request: &ProofRequestData, proof: &ProofMessage) -> Result<(), ProofError> {
    let provided_predicates = &proof.requested_proof.predicates;
    let requested_predicates = &request.requested_predicates;

    for (predicate_uuid, requested_predicate) in requested_predicates.iter() {
        let proof_id = match provided_predicates.get(predicate_uuid) {
            Some(uuid) => uuid,
            None => {
                warn!("Proof Compliance: requested predicate id not found in proof");
                return Err(ProofError::FailedProofCompliance())
            }
        };

        let proof_data = match  proof.proof.proofs.get(proof_id) {
            Some(x) => x,
            None => {
                warn!("Proof Compliance: proof id not found in proofs");
                return Err(ProofError::FailedProofCompliance())
            }
        };
        let proved_predicates = proof.get_predicates_from_credential(proof_id)
            .map_err(|ec| ProofError::ProofMessageError(ec))?;
        let predicate = proved_predicates.iter().find(|predicate| {
            predicate.attr_info.clone().unwrap_or(Attr::new()).name == requested_predicate.attr_name
        });

        match predicate {
            Some(x) => {
                let identifier: &Identifier = proof.identifiers.get(proof_id)
                    .ok_or(ProofError::FailedProofCompliance())?;

                if !compare_specs(requested_predicate.restrictions.clone(),
                               &identifier) {
                    return Err(ProofError::FailedProofCompliance())
                }
            },

            None => return Err(ProofError::FailedProofCompliance())
        }
    }
    Ok(())
}

fn verify_requested_attributes(request: &ProofRequestData, proof: &ProofMessage) -> Result<(), ProofError> {
    let proof_revealed_attrs = &proof.requested_proof.revealed_attrs;
    let self_attested_attrs = &proof.requested_proof.self_attested_attrs;
    let requested_attrs = &request.requested_attrs;

    for (key, val) in requested_attrs.iter() {
        let proof_attr_data = match proof_revealed_attrs.get(key) {
            Some(data) => data,
            None => {
                if val.restrictions.is_none() && self_attested(&val.name, self_attested_attrs)? {
                    debug!("attribute: {} was self attested", val.name);
                    continue
                }
                warn!("Proof Compliance: attr_id not found in proof");
                return Err(ProofError::FailedProofCompliance())
            }
        };
        let proof_id = &proof_attr_data.0;
        let identifier = proof.identifiers.get(proof_id)
            .ok_or(ProofError::FailedProofCompliance())?;

        if !compare_specs(val.restrictions.clone(),
                          identifier) {
            return Err(ProofError::FailedProofCompliance())
        }
    }

    Ok(())
}

fn self_attested(attr_name: &str, self_attested_attrs: &HashMap<String, String>) -> Result<bool, ProofError> {
    match self_attested_attrs.get(attr_name) {
        Some(_) => Ok(true),
        None => {
            warn!("Proof Compliance: attr_id not found in proof");
            Err(ProofError::FailedProofCompliance())
        }
    }
}

fn compare_specs(requested_specs: Option<Vec<Filter>>, provided_spec: &Identifier) -> bool {
    if requested_specs.is_none() {
        return true;
    }
    let requested_specs: Vec<Filter> = requested_specs.unwrap_or_default();
    let provided_schema_key = &provided_spec.schema_key;

    for cmp_spec in requested_specs.iter() {
        let same_issuer_did = check_value(cmp_spec.issuer_did.clone(), &provided_spec.issuer_did);
        let same_schema_key = match cmp_spec.schema_key.clone() {
            Some(SchemaKeyFilter{ did, version, name }) => {
                let same_did = check_value(did, &provided_schema_key.did);
                let same_name = check_value(name, &provided_schema_key.name);
                let same_version = check_value(version, &provided_schema_key.version);
                same_did && same_name && same_version
            },
            None => true
        };

        if same_issuer_did && same_schema_key {
            return true;
        }
    }
    return false;
}

fn check_value<T: PartialEq>(control: Option<T>, val: &T) -> bool {
    if control.is_none() {
        return true;
    }
    let rtn = control.unwrap().eq(val);

    rtn
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;
    use ::proof_compliance::{proof_compliance};
    use ::messages::proofs::proof_message::{ ProofMessage };
    use messages::proofs::proof_request::{ AttrInfo, PredicateInfo, ProofRequestData };
    use serde_json::{ from_str };
    use ::proof_compliance::check_value;
    use utils::types::SchemaKey;

    static PROOF: &'static str = r#"{
   "proof":{
      "proofs":{
         "claim::bb929325-e8e6-4637-ba26-b19807b1f618":{
            "primary_proof":{
               "eq_proof":{
                  "revealed_attrs":{
                     "name":"1139481716457488690172217916278103335"
                  },
                  "a_prime":"123",
                  "e":"456",
                  "v":"5",
                  "m":{
                     "age":"456",
                     "height":"4532",
                     "sex":"444"
                  },
                  "m1":"5432",
                  "m2":"211"
               },
               "ge_proofs":[
                  {
                     "u":{
                        "2":"6",
                        "1":"5",
                        "0":"7",
                        "3":"8"
                     },
                     "r":{
                        "1":"9",
                        "3":"0",
                        "DELTA":"8",
                        "2":"6",
                        "0":"9"
                     },
                     "mj":"2",
                     "alpha":"3",
                     "t":{
                        "DELTA":"4",
                        "1":"5",
                        "0":"6",
                        "2":"7",
                        "3":"8"
                     },
                     "predicate":{
                        "attr_name":"age",
                        "p_type":"GE",
                        "value":18
                     }
                  }
               ]
            },
            "non_revoc_proof":null
         }
      },
      "aggregated_proof":{
         "c_hash":"31470331269146455873134287006934967606471534525199171477580349873046877989406",
         "c_list":[
            [
               182
            ],
            [
               96,
               49
            ],
            [
               1
            ]
         ]
      }
   },
   "requested_proof":{
      "revealed_attrs":{
         "name_1":[
            "claim::bb929325-e8e6-4637-ba26-b19807b1f618",
            "Alex",
            "1139481716457488690172217916278103335"
         ]
      },
      "unrevealed_attrs":{

      },
      "self_attested_attrs":{

      },
      "predicates":{
         "age_2":"claim::bb929325-e8e6-4637-ba26-b19807b1f618"
      }
   },
   "identifiers":{
      "claim::bb929325-e8e6-4637-ba26-b19807b1f618":{
         "issuer_did":"NcYxiDXkpYi6ov5FcYDi1e",
         "schema_key":{
            "name":"gvt",
            "version":"1.0",
            "did":"NcYxiDXkpYi6ov5FcYDi1e"
         },
         "rev_reg_seq_no":null
      }
   }
}"#;

    static REQUEST: &'static str = r#"
    {
      "nonce":"123432421212",
      "name":"Home Address",
      "version":"0.1",
      "requested_attrs":{
        "name_1":{
              "name":"name",
              "restrictions":[
                 {
                    "issuer_did":"NcYxiDXkpYi6ov5FcYDi1e",
                    "schema_key":{
                       "name":"gvt",
                       "version":"1.0",
                       "did":"NcYxiDXkpYi6ov5FcYDi1e"
                    }
                 },
                 {
                    "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB",
                    "schema_key":{
                       "name":"BYU Student Info",
                       "version":"1.0",
                       "did":"5XFh8yBzrpJQmNyZzgoTqB"
                    }
                 }
              ]
           }
      },
      "requested_predicates":{
        "age_2": {
            "attr_name":"age",
            "p_type":"GE",
            "value":22,
            "restrictions":[
            {
                "issuer_did":"NcYxiDXkpYi6ov5FcYDi1e",
                "schema_key":{
                   "name":"gvt",
                   "version":"1.0",
                   "did":"NcYxiDXkpYi6ov5FcYDi1e"
                }
            }]
        }
      }
    }
    "#;

#[test]
fn test_check_value(){
        //Test equal
        let control = "sdf".to_string();
        let val = "sdf".to_string();
        assert!(check_value(Some(control), &val));

        //Test not equal
        let control = "eee".to_string();
        assert!(!check_value(Some(control), &val));

        //Test None control
        assert!(check_value(None, &val));
    }

    #[test]
    fn test_compare_specs() {
        let identifier = Identifier {
            issuer_did: "123".to_string(),
            schema_key: SchemaKey {
                name: "schema_name".to_string(),
                version: "0.1".to_string(),
                did: "234".to_string()
            },
            rev_reg_seq_no: None
        };

        let filter1 = Filter {
            issuer_did: Some("123".to_string()),
            schema_key: Some(SchemaKeyFilter {
                name: Some("schema_name".to_string()),
                version: Some("0.1".to_string()),
                did: Some("234".to_string())
            })
        };
        let filter2 = Filter {
            issuer_did: Some("456".to_string()),
            schema_key: Some(SchemaKeyFilter {
                name: Some("schema_name2".to_string()),
                version: Some("0.2".to_string()),
                did: Some("567".to_string())
            })
        };
        let filter3 = Filter {
            issuer_did: Some("123".to_string()),
            schema_key: None
        };
        let mut filters: Vec<Filter> = Vec::new();

        // No specs in Request
        assert!(compare_specs(None,&identifier));

        // Only issuer_did specified in request
        assert!(compare_specs(Some(vec![filter3.clone()]), &identifier));

        // Proof doesn't contain specified schema_key and issuer_did
        filters.push(filter2);
        assert!(!compare_specs(Some(filters.clone()), &identifier));

        // Proof contains specified schema_key and issuer_did
        filters.push(filter1);
        assert!(compare_specs(Some(filters.clone()), &identifier));
    }

    #[test]
    fn test_proof_with_predicates() {
        let proof = ProofMessage::from_str(PROOF).unwrap();

        let proof_req: ProofRequestData = from_str(REQUEST).unwrap();

        proof_compliance(&proof_req, &proof).unwrap();
    }

    #[test]
    fn test_compliance_failed_with_missing_predicate() {
        let pred_str = r#"{ "attr_name":"missing_pred", "p_type":"GE", "value":22, "restrictions":[ { "issuer_did":"NcYxiDXkpYi6ov5FcYDi1e", "schema_key":{ "name":"gvt", "version":"1.0", "did":"NcYxiDXkpYi6ov5FcYDi1e" } }] }"#;
        let proof = ProofMessage::from_str(PROOF).unwrap();

        let mut proof_req: ProofRequestData = from_str(REQUEST).unwrap();

        let predicate: PredicateInfo = serde_json::from_str(pred_str).unwrap();
        proof_req.requested_predicates.insert("missing_pred_3".to_string(), predicate);
        assert_eq!(proof_compliance(&proof_req, &proof), Err(ProofError::FailedProofCompliance()));
    }

    #[test]
    fn test_proof_fails_with_differing_issuer_dids_schema_key_values() {
        let mut proof = ProofMessage::from_str(PROOF).unwrap();
        let proof_req: ProofRequestData = from_str(REQUEST).unwrap();
        assert!(proof_compliance(&proof_req, &proof).is_ok());
        let original_id: Identifier = proof.identifiers.get("claim::bb929325-e8e6-4637-ba26-b19807b1f618").unwrap().clone();
        proof.identifiers.remove("claim::bb929325-e8e6-4637-ba26-b19807b1f618");

        // Different issuer_did
        let mut change_issuer_did = original_id.clone();
        change_issuer_did .issuer_did = "changed_did".to_string();
        proof.identifiers.insert("claim::bb929325-e8e6-4637-ba26-b19807b1f618".to_string(), change_issuer_did );
        assert_eq!(proof_compliance(&proof_req, &proof), Err(ProofError::FailedProofCompliance()));
        proof.identifiers.remove("claim::bb929325-e8e6-4637-ba26-b19807b1f618");

        // Different schema_did
        let mut change_schema_did = original_id.clone();
        change_schema_did.schema_key.did = "changed_did".to_string();
        proof.identifiers.insert("claim::bb929325-e8e6-4637-ba26-b19807b1f618".to_string(), change_schema_did);
        assert_eq!(proof_compliance(&proof_req, &proof), Err(ProofError::FailedProofCompliance()));
        proof.identifiers.remove("claim::bb929325-e8e6-4637-ba26-b19807b1f618");

        // Different schema_name
        let mut change_schema_name = original_id.clone();
        change_schema_name.schema_key.name = "changed_name".to_string();
        proof.identifiers.insert("claim::bb929325-e8e6-4637-ba26-b19807b1f618".to_string(), change_schema_name);
        assert_eq!(proof_compliance(&proof_req, &proof), Err(ProofError::FailedProofCompliance()));
        proof.identifiers.remove("claim::bb929325-e8e6-4637-ba26-b19807b1f618");
    }

    #[test]
    fn test_proof_with_self_attested_values(){
        let mut proof = ProofMessage::from_str(PROOF).unwrap();

        let mut proof_req: ProofRequestData = from_str(REQUEST).unwrap();

        let self_attested: AttrInfo = AttrInfo {
            name: "self_attested_name".to_string(),
            restrictions: None,
        };
        proof_req.requested_attrs.insert("self_attested_3".to_string(), self_attested);
        proof.requested_proof.self_attested_attrs.insert("self_attested_name".to_string(), "value".to_string());
        proof_compliance(&proof_req, &proof).unwrap();
    }

    #[test]
    fn test_self_attested_fails_when_issuer_did_expected(){
        let mut proof = ProofMessage::from_str(PROOF).unwrap();

        let mut proof_req: ProofRequestData = from_str(REQUEST).unwrap();

        let self_attested: AttrInfo = AttrInfo {
            name: "self_attested_name".to_string(),
            restrictions: Some(vec![
                Filter {
                    issuer_did: Some("Some_did".to_string()),
                    schema_key: None,
                }
            ]),
        };
        proof_req.requested_attrs.insert("self_attested_3".to_string(), self_attested);
        proof.requested_proof.self_attested_attrs.insert("self_attested_name".to_string(), "value".to_string());
        assert_eq!(proof_compliance(&proof_req, &proof), Err(ProofError::FailedProofCompliance()));
    }
}*/
