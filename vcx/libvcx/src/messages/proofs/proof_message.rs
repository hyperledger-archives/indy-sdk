use serde_json;
use serde_json::Value;
use error::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofMessage {
    version: Option<String>,
    to_did: Option<String>,
    from_did: Option<String>,
    proof_request_id: Option<String>,
    pub libindy_proof: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CredInfo {
    pub schema_id: String,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>,
    pub timestamp: Option<u64>,
}

impl ProofMessage {
    pub fn new() -> ProofMessage {
        ProofMessage {
            version: None,
            to_did: None,
            from_did: None,
            proof_request_id: None,
            libindy_proof: String::new(),
        }
    }

    pub fn to_string(&self) -> VcxResult<String> {
        serde_json::to_string(&self)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidProof, format!("Cannot serialize proof: {}", err)))
    }

    pub fn from_str(payload: &str) -> VcxResult<ProofMessage> {
        serde_json::from_str(payload)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidProof, format!("Cannot deserialize proof: {}", err)))
    }

    pub fn get_credential_info(&self) -> VcxResult<Vec<CredInfo>> {
        let mut rtn = Vec::new();

        let credentials: Value = serde_json::from_str(&self.libindy_proof)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize liibndy proof: {}", err)))?;

        if let Value::Array(ref identifiers) = credentials["identifiers"] {
            for identifier in identifiers {
                if let (Some(schema_id), Some(cred_def_id)) = (identifier["schema_id"].as_str(),
                                                               identifier["cred_def_id"].as_str()) {
                    let rev_reg_id = identifier["rev_reg_id"]
                        .as_str()
                        .map(|x| x.to_string());

                    let timestamp = identifier["timestamp"].as_u64();
                    rtn.push(
                        CredInfo {
                            schema_id: schema_id.to_string(),
                            cred_def_id: cred_def_id.to_string(),
                            rev_reg_id,
                            timestamp,
                        }
                    );
                } else { return Err(VcxError::from_msg(VcxErrorKind::InvalidProofCredentialData, "Cannot get identifiers")); }
            }
        }

        Ok(rtn)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use ::utils::constants::{SCHEMA_ID, CRED_DEF_ID, REV_REG_ID};

    static TEMP_REQUESTER_DID: &'static str = "GxtnGN6ypZYgEqcftSQFnC";
    static MSG_FROM_API: &str = r#"{"proofs":{"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"sex":"5944657099558967239210949258394887428692050081607692519917050011144233115103","name":"1139481716457488690172217916278103335"},"a_prime":"55115757663642844902979276276581544287881791112969892277372135316353511833640150801244335663890109536491278379177551666081054765286807563008348637104046950934828407012194403360724040287698135607556244297972578864339500981366412262454282194811242239615009347165118318516694216754501345324782597475927199400880006212632553233049354866295429520527445980181939247828351677971991914388778860092824318440481574181300185829423762990910739241691289976584754979812272223819007422499654272590946235912914032826994670588466080422906806402660885408376207875827950805200378568062518210110828954480363081643567615791016011737856977","e":"34976147138641338975844073241645969211530343885520088294714132974884138611036204288689212378023649179372520412699253155486970203797562324","v":"961473607552945346906354315658276499450491951690969023699851664262072769313929148332129868528140265952852653009499943891795293148107502144091334703992581737220352761140064276811372868396353572957613845323343723271098601244774874235526135299483412285009916812621185291842845156342501611029106982811773616231232684804116984093651972537804480090649736612551759833591251845595059217608938213987633789344584340351801507541774726753840600143685051258161251666953243698589585559347435011414292427590918153421953579895479604685390401357681887618798200391305919594609949167659780330698000168295871428737686822637913218269005987492318466661186509308179489615192663542904993253626728197630057096161118638090776180812895097232529119979970798938360220605280817954648588493778338816318524451785027916181454650102696493927306340658666852294316562458212054696739343800993703515542777264448535624584845146378512183572107830260813929222999","m":{},"m1":"75548120024969192086664289521241751069844239013520403238642886571169851979005373784309432586593371476370934469326730539754613694936161784687213609047455188306625204249706249661640538349287762196100659095340756990269587317065862046598569445591945049204366911309949910119711238973099702616527117177036784698661","m2":"287944186286321709724396773443214682376883853676549188669693055373059354657799325692443906346632814001611911026063358134413175852024773765930829079850890920811398176944587192618"},"ge_proofs":[{"u":{"1":"1","0":"0","3":"3","2":"4"},"r":{"1":"1","0":"2","DELTA":"3","3":"4","2":"5"},"mj":"6","alpha":"7","t":{"1":"8","3":"3","0":"2","DELTA":"1","2":"2"},"predicate":{"attr_name":"age","p_type":"GE","value":18,"schema_seq_no":14,"issuer_did":"33UDR9R7fjwELRvH9JT6HH"}}]},"non_revoc_proof":null},"schema_seq_no":103,"issuer_did":"V4SGRU86Z58d6TV7PBUe6f"}},"aggregated_proof":{"c_hash":"63330487197040957750863022608534150304998351350639315143102570772502292901825","c_list":[[1,180,153,212,162,132,5,189,14,181,140,112,236,109,182,76,91,6,161,215,62,207,205,135,86,211,49,197,215,198,104,201,14,22,48,6,112,170,31,191,110,118,121,15,62,114,126,249,221,107,114,161,163,234,19,233,150,236,182,217,195,6,218,217,193,6,94,160,33,23,103,147,109,221,81,38,138,20,225,141,68,37,142,10,225,79,164,119,168,250,188,186,47,229,165,8,237,230,14,35,53,176,97,28,82,105,87,210,117,16,154,222,66,11,96,172,90,13,239,190,29,71,11,88,53,36,219,139,67,21,136,58,161,164,97,106,56,230,55,157,59,35,187,235,154,194,111,93,168,135,67,15,97,136,38,169,87,142,32,255,50,247,111,83,44,88,251,99,6,226,182,170,146,229,118,164,118,228,235,51,137,168,135,50,1,14,1,201,72,175,102,241,149,117,88,83,84,37,205,130,26,155,124,158,211,89,112,33,46,24,94,93,202,8,127,172,214,178,6,156,79,188,132,223,239,127,200,158,95,247,139,101,51,162,168,175,74,1,67,201,94,108,192,14,130,109,217,248,193,10,142,37,95,231,227,251,209]]},"requested_proof":{"revealed_attrs":{"attr2_uuid":["claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","male","5944657099558967239210949258394887428692050081607692519917050011144233115103"],"attr1_uuid":["claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","Alex","1139481716457488690172217916278103335"]},"unrevealed_attrs":{},"self_attested_attrs":{"self_attr":"self_value"},"predicates":{"predicate_id":"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4"}},"remoteDid":"KP8AaEBc368CMK1PqZaEzX","userPairwiseDid":"PofTCeegEXT7S2aAePhM6a"}"#;
    static TEST_ATTRS: &str = r#"[{"schema_seq_no":14,"issuer_did":"33UDR9R7fjwELRvH9JT6HH","credential_uuid":"claim::f33cc7c8-924f-4541-aeff-29a9aed9c46b","proof_attrs":[{"name":"state","value":"UT","revealed":true}]},{"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH","credential_uuid":"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b","proof_attrs":[{"name":"state","value":"UT","revealed":true}]}]"#;

    pub fn create_default_proof() -> ProofMessage {
        let mut proof = ProofMessage::new();
        proof.libindy_proof = ::utils::constants::INDY_PROOF_JSON.to_string();
        proof.from_did = Some(::settings::get_config_value(::settings::CONFIG_INSTITUTION_DID).unwrap());
        proof
    }

    #[test]
    fn test_proof_struct() {
        init!("true");
        let offer = create_default_proof();
        assert_eq!(offer.from_did, Some(::settings::get_config_value(::settings::CONFIG_INSTITUTION_DID).unwrap()));
    }

    #[test]
    fn test_serialize_deserialize() {
        let proof = create_default_proof();
        let serialized = proof.to_string().unwrap();
        let proof2 = ProofMessage::from_str(&serialized).unwrap();
        assert_eq!(proof, proof2);
    }

    #[test]
    fn test_get_credential_data() {
        init!("true");
        let mut proof = ProofMessage::new();
        proof.libindy_proof = "".to_string();
        assert_eq!(proof.get_credential_info().unwrap_err().kind(), VcxErrorKind::InvalidJson);

        proof.libindy_proof = "{}".to_string();
        assert_eq!(proof.get_credential_info().unwrap(), Vec::new());

        proof.libindy_proof = json!({"identifiers": []}).to_string();
        assert_eq!(proof.get_credential_info().unwrap(), Vec::new());

        proof.libindy_proof = json!({"identifiers": [{}]}).to_string();
        assert_eq!(proof.get_credential_info().unwrap_err().kind(), VcxErrorKind::InvalidProofCredentialData);

        proof.libindy_proof = json!({"identifiers": [{
            "schema_id": null,
            "cred_def_id": null,
        }]}).to_string();
        assert_eq!(proof.get_credential_info().unwrap_err().kind(), VcxErrorKind::InvalidProofCredentialData);

        proof.libindy_proof = json!({"identifiers": [{
            "schema_id": SCHEMA_ID,
            "cred_def_id": CRED_DEF_ID,
        }]}).to_string();
        let mut cred_info = CredInfo {
            schema_id: SCHEMA_ID.to_string(),
            cred_def_id: CRED_DEF_ID.to_string(),
            rev_reg_id: None,
            timestamp: None
        };
        assert_eq!(&proof.get_credential_info().unwrap()[0], &cred_info);

        proof.libindy_proof = json!({"identifiers": [{
            "schema_id": SCHEMA_ID,
            "cred_def_id": CRED_DEF_ID,
            "rev_reg_id": REV_REG_ID,
            "timestamp": 123
        }]}).to_string();
        cred_info.rev_reg_id = Some(REV_REG_ID.to_string());
        cred_info.timestamp = Some(123);
        assert_eq!(&proof.get_credential_info().unwrap()[0], &cred_info);
    }
}
