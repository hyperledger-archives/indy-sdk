use messages::proofs::proof_message::{ ProofMessage, Attr };
use messages::proofs::proof_request::{ ProofRequestData };
use std::collections::HashMap;
use utils::error;


pub fn proof_compliance(request: &ProofRequestData, proof: &ProofMessage) -> Result<(), u32> {
    debug!("starting vcx proof verification");

    verify_requested_attributes(request, proof)?;
    verify_requested_predicates(request, proof)
}

fn verify_requested_predicates(request: &ProofRequestData, proof: &ProofMessage) -> Result<(), u32> {
    let provided_predicates = &proof.requested_proof.predicates;
    let requested_predicates = &request.requested_predicates;

    for (predicate_uuid, requested_predicate) in requested_predicates.iter() {
        let proof_id = match provided_predicates.get(predicate_uuid) {
            Some(uuid) => uuid,
            None => {
                warn!("Proof Compliance: requested predicate id not found in proof");
                return Err(error::FAILED_PROOF_COMPLIANCE.code_num);
            }
        };

        let proof_data = match  proof.proofs.get(proof_id) {
            Some(x) => x,
            None => {
                warn!("Proof Compliance: proof id not found in proofs");
                return Err(error::FAILED_PROOF_COMPLIANCE.code_num);
            }
        };
        let proved_predicates = proof_data.proof.primary_proof.get_predicates_from_claim(proof_id)?;
        let predicate = proved_predicates.iter().find(|predicate| {
            predicate.attr_info.clone().unwrap_or(Attr::new()).name == requested_predicate.attr_name
        });

        match predicate {
            Some(x) => {
                // Todo: Which issuer did and schema do I use?? The one in the GE.predicate or the one in the claim
                // Todo: currently using the one for the entire claim

                if !check_value(requested_predicate.issuer_did.clone(),
                               &proof_data.issuer_did) {
                    return Err(error::FAILED_PROOF_COMPLIANCE.code_num)
                }

                if !check_value(requested_predicate.schema_seq_no,
                                &proof_data.schema_seq_no) {
                    return Err(error::FAILED_PROOF_COMPLIANCE.code_num)
                }
            },
            None => return Err(error::FAILED_PROOF_COMPLIANCE.code_num)
        }
    }
    Ok(())
}

fn verify_requested_attributes(request: &ProofRequestData, proof: &ProofMessage) -> Result<(), u32> {
    let proof_revealed_attrs = &proof.requested_proof.revealed_attrs;
    let self_attested_attrs = &proof.requested_proof.self_attested_attrs;
    let requested_attrs = &request.requested_attrs;

    for (key, val) in requested_attrs.iter() {
        let issuer_did = val.issuer_did.clone();
        let schema_seq_no = val.schema_seq_no;

        let proof_attr_data = match proof_revealed_attrs.get(key) {
            Some(data) => data,
            None => {
                if issuer_did.is_none() && schema_seq_no.is_none() &&
                    self_attested(&val.name, self_attested_attrs)? {
                    debug!("attribute: {} was self attested", val.name);
                    continue
                }
                warn!("Proof Compliance: attr_id not found in proof");
                return Err(error::FAILED_PROOF_COMPLIANCE.code_num);
            }
        };
        let proof_id = match proof_attr_data.get(0){
            Some(id) => match id.as_str(){
                Some(id_str) => id_str,
                None => {
                    warn!("Proof Compliance: proof_id is not a string");
                    return Err(error::FAILED_PROOF_COMPLIANCE.code_num);
                }
            },
            None => {
                warn!("Proof Compliance: no data found in the revealed_attr");
                return Err(error::FAILED_PROOF_COMPLIANCE.code_num);
            }
        };

        let proof_data = match proof.proofs.get(proof_id) {
            Some(data) => data,
            None => {
                warn!("Proof Compliance: proof id not found in proofs");
                return Err(error::FAILED_PROOF_COMPLIANCE.code_num);
            }
        };

        let proof_issuer_did = &proof_data.issuer_did;
        let proof_schema_seq_no = proof_data.schema_seq_no;

        if !check_value(issuer_did,
                        proof_issuer_did) {
            return Err(error::FAILED_PROOF_COMPLIANCE.code_num)
        }

        if !check_value(schema_seq_no,
                        &proof_schema_seq_no) {
            return Err(error::FAILED_PROOF_COMPLIANCE.code_num)
        }
    }

    Ok(())
}

fn self_attested(attr_name: &str, self_attested_attrs: &HashMap<String, String>) -> Result<bool, u32> {
    match self_attested_attrs.get(attr_name) {
        Some(_) => Ok(true),
        None => {
            warn!("Proof Compliance: attr_id not found in proof");
            Err(error::FAILED_PROOF_COMPLIANCE.code_num)
        }
    }
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
    use ::proof_compliance::{proof_compliance, self_attested};
    use ::messages::proofs::proof_message::{ ProofMessage, Proofs };
    use messages::proofs::proof_request::{ ProofRequestData, Attr, Predicate };
    use serde_json::{ from_str };
    use ::utils::error;
    use ::proof_compliance::check_value;
    use ::std::collections::HashMap;

    static PROOF: &'static str = r#"{
  "msg_type":"proof",
  "version":"0.1",
  "to_did":"BnRXf8yDMUwGyZVDkSENeq",
  "from_did":"GxtnGN6ypZYgEqcftSQFnC",
  "proof_request_id":"cCanHnpFAD",
  "proofs":{
    "claim::e5fec91f-d03d-4513-813c-ab6db5715d55":{
      "proof":{
        "primary_proof":{
          "eq_proof":{
            "revealed_attrs":{
              "zip":"84000",
              "state":"96473275571522321025213415717206189191162"
            },
            "a_prime":"10274651773153565722019210659207676572786335404518266913765618750872251823554266771822150409180931579253929652863638372644112956092102925575845581017235903004421802634661338410770042644965749885398539525165605058923843522517314319840169601474150326115348421658477627554758820452250362472979147621827279182788510071804600951186983045732872156151301262535530393104859719899455306056009204604509535671725071288687771415046920016495031604620601140526898140237670090520049130704276263166955871037517804193488545341689624216777231978320458856859420756772128879180397115187197423546073998913794609594023509615701153141253514",
            "e":"30636729991264407698337826610170865432022871701652146972211791800949197822288998184033793148938094290436129457638717424098716280872544184",
            "v":"339808471108499392415454514206457300014547427273809203915255729769779326552867203982642872758658191709655708420066969487105458985465305011777504818378395826610625827021819273418962033254642361841752007815490666049367119969591625584310226296171829808835423919115498136596018856060072027375208112630486311538259864737894664484097729012932040355241966282059119553478197561664706870040652331391566931962325940529976797874725136522436013538542947931781531873435874385959627947080186533900241438285998055152688477374246163395329943054765504704962912223844145078849430621105548408642115467874982183378224947722200243378326282218927779750001231346125297597919351717168810930037964821929558954432451264094396020244533759898694666729240429990608573800933743295039479237188206925264827638548703574025716583484072861441856126220698956158318893615486128708037136950147445871012115996268430285476878204566703974887214605136509933648900",
            "m":{
              "address1":"5048023936382169703152740644279906986582604852623409871677780299224759591354465028089250557876943731162113560696328901890641772977314690929135989868274995759453070577310137956311",
              "city":"16068149975235987662176422192638023738937421436442466244863909753815872750511411934265030504709715873550783858015089180004489091640079613105083695776698501898590056359976699121884",
              "address2":"1537259373496398852470241555872721714170021506947223541092220800036170140527714251039219590817743133159684530743850708966323297606032400560271422873188265123485835804700730251835"
            },
            "m1":"59845303128978082570518456477216094137348786390939456684626458489330736025773361708349950104859105526212248441886880699536117336026468133613133626770556495499140344376818118854441771571256613884206725991163900134640132426935029027360466848334478725276354161567194869437545270134327626978755348648850529937728",
            "m2":"13711567854757423374855416277769534695980082736178460214379319083981185796735317844805006556164416597800907337868601033896083963747064067320267502756961943496241891281473074931755"
          },
          "ge_proofs":[

          ]
        },
        "non_revoc_proof":null
      },
      "schema_seq_no":15,
      "issuer_did":"4fUDR9R7fjwELRvH9JT6HH"
    }
  },
  "aggregated_proof":{
    "c_hash":"104666538285890717073429448699988610488895224340783956930929484474453451599895",
    "c_list":[
      [
        1
      ]
    ]
  },
  "requested_proof":{
    "revealed_attrs":{
      "ddd":[
        "claim::e5fec91f-d03d-4513-813c-ab6db5715d55",
        "84000",
        "84000"
      ],
      "sdf":[
        "claim::e5fec91f-d03d-4513-813c-ab6db5715d55",
        "UT",
        "96473275571522321025213415717206189191162"
      ]
    },
    "unrevealed_attrs":{

    },
    "self_attested_attrs":{

    },
    "predicates":{

    }
  }
}"#;
    static REQUEST: &'static str = r#"{
  "nonce":"123432421212",
  "name":"Home Address",
  "version":"0.1",
  "requested_attrs":{
    "sdf":{
      "schema_seq_no":15,
      "name":"state"
    },
    "ddd":{
      "schema_seq_no":15,
      "name":"zip"
    }

  },
  "requested_predicates":{

  }
}"#;
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
    fn test(){
        ::utils::logger::LoggerUtils::init();
        let proof_obj = ProofMessage::from_str(PROOF).unwrap();
        let proof_req: ProofRequestData = from_str(REQUEST).unwrap();
        proof_compliance(&proof_req, &proof_obj).unwrap();
    }

    #[test]
    fn test_proof_with_predicates() {
        let add_claim: Proofs = from_str(r#"{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"t2":"Hash for 2"},"a_prime":"3","e":"2","v":"5","m":{"t2":"2"},"m1":"2","m2":"2"},"ge_proofs":[{"u":{"2":"2","0":"2","3":"2","1":"2"},"r":{"1":"2","3":"2","DELTA":"2","2":"2","0":"2"},"mj":"3","alpha":"2","t":{"0":"2","2":"2","DELTA":"4","1":"5","3":"3"},"predicate":{"attr_name":"predicate2","p_type":"LE","value":99,"schema_seq_no":778,"issuer_did":"12345"}}]},"non_revoc_proof":null},"schema_seq_no":778,"issuer_did":"12345"}"#).unwrap();
        let mut proof = ProofMessage::from_str(PROOF).unwrap();
        proof.proofs.insert("claim2_uuid".to_string(), add_claim);
        proof.requested_proof.predicates.insert("pred_uuid".to_string(), "claim2_uuid".to_string());
        let mut proof_req: ProofRequestData = from_str(REQUEST).unwrap();
        let added_predicate: Predicate = from_str(r#"{"attr_name":"predicate2","p_type":"LE","value":99,"schema_seq_no":778,"issuer_did":"12345"}"#).unwrap();
        proof_req.requested_predicates.insert("pred_uuid".to_string(), added_predicate.clone());
        proof_compliance(&proof_req, &proof).unwrap();
    }

    #[test]
    fn test_compliance_failed_with_missing_predicate() {
        let add_claim: Proofs = from_str(r#"{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"t2":"Hash for 2"},"a_prime":"3","e":"2","v":"5","m":{"t2":"2"},"m1":"2","m2":"2"},"ge_proofs":[{"u":{"2":"2","0":"2","3":"2","1":"2"},"r":{"1":"2","3":"2","DELTA":"2","2":"2","0":"2"},"mj":"3","alpha":"2","t":{"0":"2","2":"2","DELTA":"4","1":"5","3":"3"},"predicate":{"attr_name":"predicate2","p_type":"LE","value":99,"schema_seq_no":778,"issuer_did":"12345"}}]},"non_revoc_proof":null},"schema_seq_no":778,"issuer_did":"12345"}"#).unwrap();
        let mut proof = ProofMessage::from_str(PROOF).unwrap();
        proof.proofs.insert("claim2_uuid".to_string(), add_claim);
        proof.requested_proof.predicates.insert("pred_uuid".to_string(), "claim2_uuid".to_string());
        let mut proof_req: ProofRequestData = from_str(REQUEST).unwrap();
        let added_predicate: Predicate = from_str(r#"{"attr_name":"predicate2","p_type":"LE","value":99,"schema_seq_no":778,"issuer_did":"12345"}"#).unwrap();
        let failing_predicate: Predicate = from_str(r#"{"attr_name":"missing","p_type":"GE","value":22,"schema_seq_no":664,"issuer_did":"456"}"#).unwrap();
        proof_req.requested_predicates.insert("pred_uuid".to_string(), added_predicate.clone());
        proof_req.requested_predicates.insert("fail_uuid".to_string(), failing_predicate.clone());
        let err = proof_compliance(&proof_req, &proof);
        assert_eq!(Err(error::FAILED_PROOF_COMPLIANCE.code_num), err);
    }

    #[test]
    fn test_proof_with_predicates_fails_with_differing_dids() {
        let add_claim: Proofs = from_str(r#"{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"t2":"Hash for 2"},"a_prime":"3","e":"2","v":"5","m":{"t2":"2"},"m1":"2","m2":"2"},"ge_proofs":[{"u":{"2":"2","0":"2","3":"2","1":"2"},"r":{"1":"2","3":"2","DELTA":"2","2":"2","0":"2"},"mj":"3","alpha":"2","t":{"0":"2","2":"2","DELTA":"4","1":"5","3":"3"},"predicate":{"attr_name":"predicate2","p_type":"LE","value":99,"schema_seq_no":778,"issuer_did":"12345"}}]},"non_revoc_proof":null},"schema_seq_no":778,"issuer_did":"98765"}"#).unwrap();
        let mut proof = ProofMessage::from_str(PROOF).unwrap();
        proof.proofs.insert("claim2_uuid".to_string(), add_claim);
        proof.requested_proof.predicates.insert("pred_uuid".to_string(), "claim2_uuid".to_string());
        let mut proof_req: ProofRequestData = from_str(REQUEST).unwrap();
        let added_predicate: Predicate = from_str(r#"{"attr_name":"predicate2","p_type":"LE","value":99,"schema_seq_no":778,"issuer_did":"12345"}"#).unwrap();
        proof_req.requested_predicates.insert("pred_uuid".to_string(), added_predicate.clone());
        let err = proof_compliance(&proof_req, &proof);
        assert_eq!(Err(error::FAILED_PROOF_COMPLIANCE.code_num), err);
    }

    #[test]
    fn test_proof_with_predicates_fails_with_differing_schema_no() {
        let add_claim: Proofs = from_str(r#"{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"t2":"Hash for 2"},"a_prime":"3","e":"2","v":"5","m":{"t2":"2"},"m1":"2","m2":"2"},"ge_proofs":[{"u":{"2":"2","0":"2","3":"2","1":"2"},"r":{"1":"2","3":"2","DELTA":"2","2":"2","0":"2"},"mj":"3","alpha":"2","t":{"0":"2","2":"2","DELTA":"4","1":"5","3":"3"},"predicate":{"attr_name":"predicate2","p_type":"LE","value":99,"schema_seq_no":null,"issuer_did":null}}]},"non_revoc_proof":null},"schema_seq_no":321,"issuer_did":"98765"}"#).unwrap();
        let mut proof = ProofMessage::from_str(PROOF).unwrap();
        proof.proofs.insert("claim2_uuid".to_string(), add_claim);
        proof.requested_proof.predicates.insert("pred_uuid".to_string(), "claim2_uuid".to_string());
        let mut proof_req: ProofRequestData = from_str(REQUEST).unwrap();
        let added_predicate: Predicate = from_str(r#"{"attr_name":"predicate2","p_type":"LE","value":99,"schema_seq_no":778,"issuer_did":"12345"}"#).unwrap();
        proof_req.requested_predicates.insert("pred_uuid".to_string(), added_predicate.clone());
        let err = proof_compliance(&proof_req, &proof);
        assert_eq!(Err(error::FAILED_PROOF_COMPLIANCE.code_num), err);
    }

    #[test]
    fn test_proof_with_self_attested_values(){
        ::utils::logger::LoggerUtils::init();
        let mut proof_obj = ProofMessage::from_str(PROOF).unwrap();
        let mut proof_req: ProofRequestData = from_str(REQUEST).unwrap();
        proof_obj.requested_proof.self_attested_attrs.insert("dog".to_string(), "ralph".to_string());
        proof_obj.requested_proof.self_attested_attrs.insert("cat".to_string(), "spot".to_string());
        proof_req.requested_attrs.insert("ccc".to_string(),
                                         Attr{ name: "dog".to_string(), issuer_did: None, schema_seq_no: None});
        proof_req.requested_attrs.insert("bbb".to_string(),
                                         Attr{ name: "cat".to_string(), issuer_did: None, schema_seq_no: None});
        proof_compliance(&proof_req, &proof_obj).unwrap();
    }

    #[test]
    fn test_self_attested_fails_when_issuer_did_expected(){
        ::utils::logger::LoggerUtils::init();
        let mut proof_obj = ProofMessage::from_str(PROOF).unwrap();
        let mut proof_req: ProofRequestData = from_str(REQUEST).unwrap();
        proof_obj.requested_proof.self_attested_attrs.insert("dog".to_string(), "ralph".to_string());
        proof_obj.requested_proof.self_attested_attrs.insert("cat".to_string(), "spot".to_string());
        proof_req.requested_attrs.insert("ccc".to_string(),
                                         Attr{ name: "dog".to_string(), issuer_did: None, schema_seq_no: None});
        proof_req.requested_attrs.insert("bbb".to_string(),
                                         Attr{ name: "cat".to_string(), issuer_did: Some("123".to_string()), schema_seq_no: None});

        let err = proof_compliance(&proof_req, &proof_obj);
        assert_eq!(Err(error::FAILED_PROOF_COMPLIANCE.code_num), err);
    }

    #[test]
    fn test_self_attested() {
        ::utils::logger::LoggerUtils::init();
        let mut self_attested_vals: HashMap<String, String> = HashMap::new();
        self_attested_vals.insert("dog".to_string(), "sally".to_string());
        self_attested_vals.insert("cat".to_string(), "matt".to_string());
        assert_eq!(true, self_attested("dog", &self_attested_vals).unwrap());
        let err = self_attested("random", &self_attested_vals);
        assert_eq!(err, Err(error::FAILED_PROOF_COMPLIANCE.code_num));
    }
}
