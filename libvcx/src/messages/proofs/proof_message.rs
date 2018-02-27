extern crate serde_json;

use utils::error;
use serde_json::Value;
use std::collections::{ HashMap, HashSet };

static ISSUER_DID: &'static str = "issuer_did";
static SEQUENCE_NUMBER: &'static str = "schema_seq_no";
static PROVER_DID: &'static str = "prover_did";
static MSG_FROM_API: &str = r#"{"proofs":{"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"sex":"5944657099558967239210949258394887428692050081607692519917050011144233115103","name":"1139481716457488690172217916278103335"},"a_prime":"55115757663642844902979276276581544287881791112969892277372135316353511833640150801244335663890109536491278379177551666081054765286807563008348637104046950934828407012194403360724040287698135607556244297972578864339500981366412262454282194811242239615009347165118318516694216754501345324782597475927199400880006212632553233049354866295429520527445980181939247828351677971991914388778860092824318440481574181300185829423762990910739241691289976584754979812272223819007422499654272590946235912914032826994670588466080422906806402660885408376207875827950805200378568062518210110828954480363081643567615791016011737856977","e":"34976147138641338975844073241645969211530343885520088294714132974884138611036204288689212378023649179372520412699253155486970203797562324","v":"961473607552945346906354315658276499450491951690969023699851664262072769313929148332129868528140265952852653009499943891795293148107502144091334703992581737220352761140064276811372868396353572957613845323343723271098601244774874235526135299483412285009916812621185291842845156342501611029106982811773616231232684804116984093651972537804480090649736612551759833591251845595059217608938213987633789344584340351801507541774726753840600143685051258161251666953243698589585559347435011414292427590918153421953579895479604685390401357681887618798200391305919594609949167659780330698000168295871428737686822637913218269005987492318466661186509308179489615192663542904993253626728197630057096161118638090776180812895097232529119979970798938360220605280817954648588493778338816318524451785027916181454650102696493927306340658666852294316562458212054696739343800993703515542777264448535624584845146378512183572107830260813929222999","m":{},"m1":"75548120024969192086664289521241751069844239013520403238642886571169851979005373784309432586593371476370934469326730539754613694936161784687213609047455188306625204249706249661640538349287762196100659095340756990269587317065862046598569445591945049204366911309949910119711238973099702616527117177036784698661","m2":"287944186286321709724396773443214682376883853676549188669693055373059354657799325692443906346632814001611911026063358134413175852024773765930829079850890920811398176944587192618"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":103,"issuer_did":"V4SGRU86Z58d6TV7PBUe6f"}},"aggregated_proof":{"c_hash":"63330487197040957750863022608534150304998351350639315143102570772502292901825","c_list":[[1,180,153,212,162,132,5,189,14,181,140,112,236,109,182,76,91,6,161,215,62,207,205,135,86,211,49,197,215,198,104,201,14,22,48,6,112,170,31,191,110,118,121,15,62,114,126,249,221,107,114,161,163,234,19,233,150,236,182,217,195,6,218,217,193,6,94,160,33,23,103,147,109,221,81,38,138,20,225,141,68,37,142,10,225,79,164,119,168,250,188,186,47,229,165,8,237,230,14,35,53,176,97,28,82,105,87,210,117,16,154,222,66,11,96,172,90,13,239,190,29,71,11,88,53,36,219,139,67,21,136,58,161,164,97,106,56,230,55,157,59,35,187,235,154,194,111,93,168,135,67,15,97,136,38,169,87,142,32,255,50,247,111,83,44,88,251,99,6,226,182,170,146,229,118,164,118,228,235,51,137,168,135,50,1,14,1,201,72,175,102,241,149,117,88,83,84,37,205,130,26,155,124,158,211,89,112,33,46,24,94,93,202,8,127,172,214,178,6,156,79,188,132,223,239,127,200,158,95,247,139,101,51,162,168,175,74,1,67,201,94,108,192,14,130,109,217,248,193,10,142,37,95,231,227,251,209]]},"requested_proof":{"revealed_attrs":{"attr2_uuid":["claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","male","5944657099558967239210949258394887428692050081607692519917050011144233115103"],"attr1_uuid":["claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","Alex","1139481716457488690172217916278103335"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{}},"remoteDid":"KP8AaEBc368CMK1PqZaEzX","userPairwiseDid":"PofTCeegEXT7S2aAePhM6a"}"#;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofMessage{
    version: Option<String>,
    to_did: Option<String>,
    from_did: Option<String>,
    proof_request_id: Option<String>,
    pub proofs: HashMap<String, Proofs>,
    pub aggregated_proof: AggregatedProof,
    pub requested_proof: RequestedProof,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Predicate {
    pub attr_name: String,
    pub p_type: String,
    pub value: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_seq_no: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_did: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AggregatedProof {
    c_hash: String,
    c_list: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequestedProof {
    pub revealed_attrs: HashMap<String, Vec<Value>>,
    pub unrevealed_attrs: HashMap<String, Value>,
    pub self_attested_attrs: HashMap<String, String>,
    pub predicates: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Proofs{
    pub proof: ProofOptions,
    pub schema_seq_no: u32,
    pub issuer_did: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofOptions{
    pub primary_proof: EqAndGeProof,
    non_revoc_proof: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EqAndGeProof{
    eq_proof: EqProof,
    ge_proofs: Vec<PrimaryPredicateGEProof>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EqProof{
    revealed_attrs: HashMap<String, Value>,
    a_prime: String,
    e: String,
    v: String,
    m: HashMap<String, String>,
    m1: String,
    m2: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PrimaryPredicateGEProof {
    pub u: HashMap<String, Value>,
    pub r: HashMap<String, Value>,
    pub mj: Value,
    pub alpha: Value,
    pub t: HashMap<String, Value>,
    pub predicate: Predicate
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ClaimData{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_seq_no: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_did: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr_info: Option<Attr>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Attr{
    pub name: String,
    pub value: Value,
    #[serde(rename = "type")]
    pub attr_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate_type: Option<String>,
}

impl ProofMessage {
    pub fn new(did: &str) -> ProofMessage {
        ProofMessage {
            version: None,
            to_did: None,
            from_did: Some(String::from(did)),
            proof_request_id: None,
            proofs: HashMap::new(),
            aggregated_proof: AggregatedProof::new(),
            requested_proof: RequestedProof::new(),
        }
    }

    pub fn to_string(&self) -> Result<String, u32> {
        serde_json::to_string(&self).map_err(|err| {
            error!("{} with: {}", error::INVALID_PROOF.message, err);
        error::INVALID_PROOF.code_num
        })
    }

    pub fn from_str(payload:&str) -> Result<ProofMessage, u32> {
        serde_json::from_str(payload)
            .map_err(|err| {
                error!("{} with serde error: {}",error::INVALID_PROOF.message, err);
                error::INVALID_PROOF.code_num
            })
    }

    pub fn get_proof_attributes(&self) -> Result<String, u32> {
        debug!("retrieving proof attributes");
        let mut claim_attrs = self.get_revealed_attrs()?;
        claim_attrs.append(self.get_predicates()?.as_mut());
        claim_attrs.append(self.get_self_attested_attrs()?.as_mut());
        //Todo: retrieve unrevealed attributes
        serde_json::to_string(&claim_attrs).map_err(|err| {
            error!("{}. Proof Attributes had invalid json.", err);
            error::INVALID_JSON.code_num
        })
    }

    pub fn get_claim_info(&self) -> Result<Vec<ClaimData>, u32> {
        self.proofs.iter().map(|(claim_uuid, proof_data)|
                Ok(ClaimData::create(Some(proof_data.schema_seq_no),
                                  Some(proof_data.issuer_did.clone()),
                                  Some(claim_uuid.to_string()),
                                  None)))
            .collect::<Result<Vec<ClaimData>, u32>>()
    }

    fn get_revealed_attrs(&self) -> Result<Vec<ClaimData>, u32> {
        debug!("retrieving revealed attributes");

        self.requested_proof.revealed_attrs.iter().map(|(attr_id, attr_data)| {
            let claim_uuid: String = serde_json::from_value(attr_data[0].clone())
                .map_err(|err| {
                    error!("{} with serde error: {}",error::INVALID_PROOF_CLAIM_DATA.message, err);
                    error::INVALID_PROOF_CLAIM_DATA.code_num
                })?;
            let claim_info = match self.proofs.get(&claim_uuid) {
                Some(x) => x,
                None => return Err(error::INVALID_PROOF_CLAIM_DATA.code_num)
            };
            let revealed_attr_value = attr_data[1].clone();
            let revealed_attr_name = claim_info.proof.primary_proof
                .retrieve_revealed_attr_name(&attr_data[2])?;

            Ok(ClaimData::create( Some(claim_info.schema_seq_no),
                               Some(claim_info.issuer_did.to_string()),
                               Some(claim_uuid.to_string()),
                               Some(Attr::create(&revealed_attr_name,
                                                 &revealed_attr_value,
                                                 "revealed",
                                                 None))))
        }).collect::<Result<Vec<ClaimData>, u32>>()
    }

    fn get_predicates(&self) -> Result<Vec<ClaimData>, u32> {
        debug!("retrieving predicates");
        // Collect all unique claim_uuid's which contain predicate values
        let mut claims_with_predicates: HashSet<String> = HashSet::new();
        let mut claim_data: Vec<ClaimData> = Vec::new();

        for claim_uuid in self.requested_proof.predicates.values() {
            if !claims_with_predicates.contains(claim_uuid) {
                claims_with_predicates.insert(claim_uuid.to_string());
                let claim_info = match self.proofs.get(claim_uuid) {
                    Some(x) => x,
                    None => return Err(error::INVALID_PROOF_CLAIM_DATA.code_num)
                };
                claim_data.append(claim_info.proof.primary_proof
                    .get_predicates_from_claim(claim_uuid)?.as_mut())
            }
        }
        Ok(claim_data)
    }

    fn get_self_attested_attrs(&self) -> Result<Vec<ClaimData>, u32> {
        debug!("retrieving self attested attributes");

        self.requested_proof.self_attested_attrs.iter().map(|(key, val)| {
            let revealed_val = serde_json::to_value(val).map_err(|err|{
                error!("{} with serde error: {}",error::INVALID_SELF_ATTESTED_VAL.message, err);
                error::INVALID_SELF_ATTESTED_VAL.code_num
            })?;
            Ok(ClaimData::create(None, None, None, Some(Attr::create(key,
                                                                     &revealed_val,
                                                                     "self_attested",
                                                                     None))))
        }).collect::<Result<Vec<ClaimData>, u32>>()
    }
}

impl AggregatedProof {
    pub fn new() -> AggregatedProof {
        AggregatedProof {
            c_hash: String::new(),
            c_list: Vec::new(),
        }
    }
}

impl RequestedProof {
    pub fn new() -> RequestedProof {
        RequestedProof {
            revealed_attrs: HashMap::new(),
            unrevealed_attrs: HashMap::new(),
            self_attested_attrs: HashMap::new(),
            predicates: HashMap::new(),
        }
    }

}


impl Proofs {
    pub fn new() -> Proofs {
        Proofs {
            proof: ProofOptions::new(),
            schema_seq_no: 0,
            issuer_did: String::new(),
        }
    }
}


impl ProofOptions {
    pub fn new() -> ProofOptions {
        ProofOptions {
            primary_proof: EqAndGeProof::new(),
            non_revoc_proof: serde_json::Value::Null,
        }
    }
}


impl EqAndGeProof {
    pub fn new() -> EqAndGeProof {
        EqAndGeProof {
            eq_proof: EqProof::new(),
            ge_proofs: Vec::new(),
        }
    }

    pub fn retrieve_revealed_attr_name(&self, attr_value: &Value) -> Result<String, u32> {
        for (name, cmp_attr) in &self.eq_proof.revealed_attrs {
            if attr_value == cmp_attr { return Ok(name.to_string()) }
        }
        Err(error::INVALID_PROOF_CLAIM_DATA.code_num)
    }

    pub fn get_predicates_from_claim(&self, uuid: &str) -> Result<Vec<ClaimData>, u32> {
        self.ge_proofs.iter().map(|ge_proof| {
            let predicate = &ge_proof.predicate;
            let value = serde_json::to_value(predicate.value)
                .map_err(|err| {
                    error!("{} with: {}", error::INVALID_PREDICATE.message, err);
                    error::INVALID_PREDICATE.code_num
                })?;

            Ok(ClaimData::create(predicate.schema_seq_no,
                                 predicate.issuer_did.clone(),
                                 Some(uuid.to_string()),
                                 Some(Attr::create(&predicate.attr_name,
                                                   &value,
                                                   "predicate",
                                                   Some(predicate.p_type.to_string())))))
        }).collect::<Result<Vec<ClaimData>, u32>>()
    }
}


impl EqProof {
    pub fn new() -> EqProof {
        EqProof {
            revealed_attrs: HashMap::new(),
            a_prime: String::new(),
            e: String::new(),
            v: String::new(),
            m: HashMap::new(),
            m1: String::new(),
            m2: String::new(),
        }
    }
}

impl ClaimData {
    pub fn new() -> ClaimData {
        ClaimData{
            schema_seq_no: None,
            issuer_did: None,
            claim_uuid: None,
            attr_info: None,
        }
    }

    pub fn create(schema_seq_no: Option<u32>,
                  issuer_did: Option<String>,
                  claim_uuid: Option<String>,
                  attr_info: Option<Attr>) -> ClaimData {
        ClaimData {
            schema_seq_no,
            issuer_did,
            claim_uuid,
            attr_info
        }
    }
}

impl Attr {

    pub fn new() -> Attr {
        Attr {
            name: String::new(),
            value: Value::Null,
            attr_type: String::new(),
            predicate_type: None
        }
    }

    pub fn create(name: &str, value: &Value, attr_type: &str, predicate_type: Option<String>) -> Attr {
        Attr {
            name: name.to_string(),
            value: value.clone(),
            attr_type: attr_type.to_string(),
            predicate_type
        }
    }
}

fn create_from_message(s: &str) -> Result<ProofMessage, u32>{
   match serde_json::from_str(s) {
       Ok(p) => Ok(p),
       Err(_) => {
           warn!("{}",error::INVALID_PROOF.message);
           Err(error::INVALID_PROOF.code_num)},
   }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    static TEMP_REQUESTER_DID: &'static str = "GxtnGN6ypZYgEqcftSQFnC";
    static MSG_FROM_API: &str = r#"{"proofs":{"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"sex":"5944657099558967239210949258394887428692050081607692519917050011144233115103","name":"1139481716457488690172217916278103335"},"a_prime":"55115757663642844902979276276581544287881791112969892277372135316353511833640150801244335663890109536491278379177551666081054765286807563008348637104046950934828407012194403360724040287698135607556244297972578864339500981366412262454282194811242239615009347165118318516694216754501345324782597475927199400880006212632553233049354866295429520527445980181939247828351677971991914388778860092824318440481574181300185829423762990910739241691289976584754979812272223819007422499654272590946235912914032826994670588466080422906806402660885408376207875827950805200378568062518210110828954480363081643567615791016011737856977","e":"34976147138641338975844073241645969211530343885520088294714132974884138611036204288689212378023649179372520412699253155486970203797562324","v":"961473607552945346906354315658276499450491951690969023699851664262072769313929148332129868528140265952852653009499943891795293148107502144091334703992581737220352761140064276811372868396353572957613845323343723271098601244774874235526135299483412285009916812621185291842845156342501611029106982811773616231232684804116984093651972537804480090649736612551759833591251845595059217608938213987633789344584340351801507541774726753840600143685051258161251666953243698589585559347435011414292427590918153421953579895479604685390401357681887618798200391305919594609949167659780330698000168295871428737686822637913218269005987492318466661186509308179489615192663542904993253626728197630057096161118638090776180812895097232529119979970798938360220605280817954648588493778338816318524451785027916181454650102696493927306340658666852294316562458212054696739343800993703515542777264448535624584845146378512183572107830260813929222999","m":{},"m1":"75548120024969192086664289521241751069844239013520403238642886571169851979005373784309432586593371476370934469326730539754613694936161784687213609047455188306625204249706249661640538349287762196100659095340756990269587317065862046598569445591945049204366911309949910119711238973099702616527117177036784698661","m2":"287944186286321709724396773443214682376883853676549188669693055373059354657799325692443906346632814001611911026063358134413175852024773765930829079850890920811398176944587192618"},"ge_proofs":[{"u":{"1":"1","0":"0","3":"3","2":"4"},"r":{"1":"1","0":"2","DELTA":"3","3":"4","2":"5"},"mj":"6","alpha":"7","t":{"1":"8","3":"3","0":"2","DELTA":"1","2":"2"},"predicate":{"attr_name":"age","p_type":"GE","value":18,"schema_seq_no":14,"issuer_did":"33UDR9R7fjwELRvH9JT6HH"}}]},"non_revoc_proof":null},"schema_seq_no":103,"issuer_did":"V4SGRU86Z58d6TV7PBUe6f"}},"aggregated_proof":{"c_hash":"63330487197040957750863022608534150304998351350639315143102570772502292901825","c_list":[[1,180,153,212,162,132,5,189,14,181,140,112,236,109,182,76,91,6,161,215,62,207,205,135,86,211,49,197,215,198,104,201,14,22,48,6,112,170,31,191,110,118,121,15,62,114,126,249,221,107,114,161,163,234,19,233,150,236,182,217,195,6,218,217,193,6,94,160,33,23,103,147,109,221,81,38,138,20,225,141,68,37,142,10,225,79,164,119,168,250,188,186,47,229,165,8,237,230,14,35,53,176,97,28,82,105,87,210,117,16,154,222,66,11,96,172,90,13,239,190,29,71,11,88,53,36,219,139,67,21,136,58,161,164,97,106,56,230,55,157,59,35,187,235,154,194,111,93,168,135,67,15,97,136,38,169,87,142,32,255,50,247,111,83,44,88,251,99,6,226,182,170,146,229,118,164,118,228,235,51,137,168,135,50,1,14,1,201,72,175,102,241,149,117,88,83,84,37,205,130,26,155,124,158,211,89,112,33,46,24,94,93,202,8,127,172,214,178,6,156,79,188,132,223,239,127,200,158,95,247,139,101,51,162,168,175,74,1,67,201,94,108,192,14,130,109,217,248,193,10,142,37,95,231,227,251,209]]},"requested_proof":{"revealed_attrs":{"attr2_uuid":["claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","male","5944657099558967239210949258394887428692050081607692519917050011144233115103"],"attr1_uuid":["claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","Alex","1139481716457488690172217916278103335"]},"unrevealed_attrs":{},"self_attested_attrs":{"self_attr":"self_value"},"predicates":{"predicate_id":"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4"}},"remoteDid":"KP8AaEBc368CMK1PqZaEzX","userPairwiseDid":"PofTCeegEXT7S2aAePhM6a"}"#;
    static TEST_ATTRS: &str = r#"[{"schema_seq_no":14,"issuer_did":"33UDR9R7fjwELRvH9JT6HH","claim_uuid":"claim::f33cc7c8-924f-4541-aeff-29a9aed9c46b","proof_attrs":[{"name":"state","value":"UT","revealed":true}]},{"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH","claim_uuid":"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b","proof_attrs":[{"name":"state","value":"UT","revealed":true}]}]"#;
    pub fn create_default_proof()-> ProofMessage {
        match ProofMessage::from_str(MSG_FROM_API){
            Ok(x) => x,
            Err(y) => {
                panic!("Had error unpacking ProofMessage: {}", y)
            }
        }
    }

    #[test]
    fn test_proof_struct(){
        let offer = create_default_proof();
        assert_eq!(offer.from_did, None);
    }

    #[test]
    fn test_eq_proof_struct_from_string(){
        let eq_proof_str = r#"{"revealed_attrs":{"sex":"5944657099558967239210949258394887428692050081607692519917050011144233115103","name":"1139481716457488690172217916278103335"},"a_prime":"55115757663642844902979276276581544287881791112969892277372135316353511833640150801244335663890109536491278379177551666081054765286807563008348637104046950934828407012194403360724040287698135607556244297972578864339500981366412262454282194811242239615009347165118318516694216754501345324782597475927199400880006212632553233049354866295429520527445980181939247828351677971991914388778860092824318440481574181300185829423762990910739241691289976584754979812272223819007422499654272590946235912914032826994670588466080422906806402660885408376207875827950805200378568062518210110828954480363081643567615791016011737856977","e":"34976147138641338975844073241645969211530343885520088294714132974884138611036204288689212378023649179372520412699253155486970203797562324","v":"961473607552945346906354315658276499450491951690969023699851664262072769313929148332129868528140265952852653009499943891795293148107502144091334703992581737220352761140064276811372868396353572957613845323343723271098601244774874235526135299483412285009916812621185291842845156342501611029106982811773616231232684804116984093651972537804480090649736612551759833591251845595059217608938213987633789344584340351801507541774726753840600143685051258161251666953243698589585559347435011414292427590918153421953579895479604685390401357681887618798200391305919594609949167659780330698000168295871428737686822637913218269005987492318466661186509308179489615192663542904993253626728197630057096161118638090776180812895097232529119979970798938360220605280817954648588493778338816318524451785027916181454650102696493927306340658666852294316562458212054696739343800993703515542777264448535624584845146378512183572107830260813929222999","m":{},"m1":"75548120024969192086664289521241751069844239013520403238642886571169851979005373784309432586593371476370934469326730539754613694936161784687213609047455188306625204249706249661640538349287762196100659095340756990269587317065862046598569445591945049204366911309949910119711238973099702616527117177036784698661","m2":"287944186286321709724396773443214682376883853676549188669693055373059354657799325692443906346632814001611911026063358134413175852024773765930829079850890920811398176944587192618"}"#;
        let eq_proof: EqProof = serde_json::from_str(eq_proof_str).unwrap();
        assert_eq!(eq_proof.revealed_attrs.get("sex").unwrap(), "5944657099558967239210949258394887428692050081607692519917050011144233115103");
    }

    #[test]
    fn test_eq_and_ge_struct_from_string(){
        let eq_and_ge_str = r#"{"eq_proof":{"revealed_attrs":{"sex":"5944657099558967239210949258394887428692050081607692519917050011144233115103","name":"1139481716457488690172217916278103335"},"a_prime":"55115757663642844902979276276581544287881791112969892277372135316353511833640150801244335663890109536491278379177551666081054765286807563008348637104046950934828407012194403360724040287698135607556244297972578864339500981366412262454282194811242239615009347165118318516694216754501345324782597475927199400880006212632553233049354866295429520527445980181939247828351677971991914388778860092824318440481574181300185829423762990910739241691289976584754979812272223819007422499654272590946235912914032826994670588466080422906806402660885408376207875827950805200378568062518210110828954480363081643567615791016011737856977","e":"34976147138641338975844073241645969211530343885520088294714132974884138611036204288689212378023649179372520412699253155486970203797562324","v":"961473607552945346906354315658276499450491951690969023699851664262072769313929148332129868528140265952852653009499943891795293148107502144091334703992581737220352761140064276811372868396353572957613845323343723271098601244774874235526135299483412285009916812621185291842845156342501611029106982811773616231232684804116984093651972537804480090649736612551759833591251845595059217608938213987633789344584340351801507541774726753840600143685051258161251666953243698589585559347435011414292427590918153421953579895479604685390401357681887618798200391305919594609949167659780330698000168295871428737686822637913218269005987492318466661186509308179489615192663542904993253626728197630057096161118638090776180812895097232529119979970798938360220605280817954648588493778338816318524451785027916181454650102696493927306340658666852294316562458212054696739343800993703515542777264448535624584845146378512183572107830260813929222999","m":{},"m1":"75548120024969192086664289521241751069844239013520403238642886571169851979005373784309432586593371476370934469326730539754613694936161784687213609047455188306625204249706249661640538349287762196100659095340756990269587317065862046598569445591945049204366911309949910119711238973099702616527117177036784698661","m2":"287944186286321709724396773443214682376883853676549188669693055373059354657799325692443906346632814001611911026063358134413175852024773765930829079850890920811398176944587192618"},"ge_proofs":[]}"#;
        let eq_ge: EqAndGeProof = serde_json::from_str(eq_and_ge_str).unwrap();
        assert_eq!(eq_ge.eq_proof.revealed_attrs.get("name").unwrap(), "1139481716457488690172217916278103335");
        assert_eq!(eq_ge.eq_proof.a_prime, "55115757663642844902979276276581544287881791112969892277372135316353511833640150801244335663890109536491278379177551666081054765286807563008348637104046950934828407012194403360724040287698135607556244297972578864339500981366412262454282194811242239615009347165118318516694216754501345324782597475927199400880006212632553233049354866295429520527445980181939247828351677971991914388778860092824318440481574181300185829423762990910739241691289976584754979812272223819007422499654272590946235912914032826994670588466080422906806402660885408376207875827950805200378568062518210110828954480363081643567615791016011737856977");
        assert_eq!(eq_ge.ge_proofs, Vec::new());
    }

    #[test]
    fn test_proof_options_struct_from_string(){
        let proof_options_str = r#"{"primary_proof":{"eq_proof":{"revealed_attrs":{"sex":"5944657099558967239210949258394887428692050081607692519917050011144233115103","name":"1139481716457488690172217916278103335"},"a_prime":"55115757663642844902979276276581544287881791112969892277372135316353511833640150801244335663890109536491278379177551666081054765286807563008348637104046950934828407012194403360724040287698135607556244297972578864339500981366412262454282194811242239615009347165118318516694216754501345324782597475927199400880006212632553233049354866295429520527445980181939247828351677971991914388778860092824318440481574181300185829423762990910739241691289976584754979812272223819007422499654272590946235912914032826994670588466080422906806402660885408376207875827950805200378568062518210110828954480363081643567615791016011737856977","e":"34976147138641338975844073241645969211530343885520088294714132974884138611036204288689212378023649179372520412699253155486970203797562324","v":"961473607552945346906354315658276499450491951690969023699851664262072769313929148332129868528140265952852653009499943891795293148107502144091334703992581737220352761140064276811372868396353572957613845323343723271098601244774874235526135299483412285009916812621185291842845156342501611029106982811773616231232684804116984093651972537804480090649736612551759833591251845595059217608938213987633789344584340351801507541774726753840600143685051258161251666953243698589585559347435011414292427590918153421953579895479604685390401357681887618798200391305919594609949167659780330698000168295871428737686822637913218269005987492318466661186509308179489615192663542904993253626728197630057096161118638090776180812895097232529119979970798938360220605280817954648588493778338816318524451785027916181454650102696493927306340658666852294316562458212054696739343800993703515542777264448535624584845146378512183572107830260813929222999","m":{},"m1":"75548120024969192086664289521241751069844239013520403238642886571169851979005373784309432586593371476370934469326730539754613694936161784687213609047455188306625204249706249661640538349287762196100659095340756990269587317065862046598569445591945049204366911309949910119711238973099702616527117177036784698661","m2":"287944186286321709724396773443214682376883853676549188669693055373059354657799325692443906346632814001611911026063358134413175852024773765930829079850890920811398176944587192618"},"ge_proofs":[]},"non_revoc_proof":null}"#;
        let proof_options: ProofOptions = serde_json::from_str(proof_options_str).unwrap();
        assert_eq!(proof_options.primary_proof.eq_proof.revealed_attrs.get("sex").unwrap(), "5944657099558967239210949258394887428692050081607692519917050011144233115103");
        assert_eq!(proof_options.non_revoc_proof, serde_json::Value::Null);
    }

    #[test]
    fn test_proofs_struct_from_string(){
        let proofs_str = r#"{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"sex":"5944657099558967239210949258394887428692050081607692519917050011144233115103","name":"1139481716457488690172217916278103335"},"a_prime":"55115757663642844902979276276581544287881791112969892277372135316353511833640150801244335663890109536491278379177551666081054765286807563008348637104046950934828407012194403360724040287698135607556244297972578864339500981366412262454282194811242239615009347165118318516694216754501345324782597475927199400880006212632553233049354866295429520527445980181939247828351677971991914388778860092824318440481574181300185829423762990910739241691289976584754979812272223819007422499654272590946235912914032826994670588466080422906806402660885408376207875827950805200378568062518210110828954480363081643567615791016011737856977","e":"34976147138641338975844073241645969211530343885520088294714132974884138611036204288689212378023649179372520412699253155486970203797562324","v":"961473607552945346906354315658276499450491951690969023699851664262072769313929148332129868528140265952852653009499943891795293148107502144091334703992581737220352761140064276811372868396353572957613845323343723271098601244774874235526135299483412285009916812621185291842845156342501611029106982811773616231232684804116984093651972537804480090649736612551759833591251845595059217608938213987633789344584340351801507541774726753840600143685051258161251666953243698589585559347435011414292427590918153421953579895479604685390401357681887618798200391305919594609949167659780330698000168295871428737686822637913218269005987492318466661186509308179489615192663542904993253626728197630057096161118638090776180812895097232529119979970798938360220605280817954648588493778338816318524451785027916181454650102696493927306340658666852294316562458212054696739343800993703515542777264448535624584845146378512183572107830260813929222999","m":{},"m1":"75548120024969192086664289521241751069844239013520403238642886571169851979005373784309432586593371476370934469326730539754613694936161784687213609047455188306625204249706249661640538349287762196100659095340756990269587317065862046598569445591945049204366911309949910119711238973099702616527117177036784698661","m2":"287944186286321709724396773443214682376883853676549188669693055373059354657799325692443906346632814001611911026063358134413175852024773765930829079850890920811398176944587192618"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":103,"issuer_did":"V4SGRU86Z58d6TV7PBUe6f"}"#;
        let proofs: Proofs = serde_json::from_str(proofs_str).unwrap();
        assert_eq!(proofs.proof.primary_proof.eq_proof.revealed_attrs.get("name").unwrap(), "1139481716457488690172217916278103335");
        assert_eq!(proofs.issuer_did, "V4SGRU86Z58d6TV7PBUe6f");
        assert_eq!(proofs.schema_seq_no, 103);
    }

    #[test]
    fn test_requested_proof_struct_from_string(){
        let requested_proof_str = r#"{"revealed_attrs":{"attr2_uuid":["claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","male","5944657099558967239210949258394887428692050081607692519917050011144233115103"],"attr1_uuid":["claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","Alex","1139481716457488690172217916278103335"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{}}"#;
        let req_proof: RequestedProof = serde_json::from_str(requested_proof_str).unwrap();
        assert_eq!(req_proof.revealed_attrs.get("attr1_uuid").unwrap()[1], serde_json::to_value("Alex").unwrap());
        assert_eq!(req_proof.self_attested_attrs, HashMap::new());
    }

    #[test]
    fn test_aggregated_proof_struct_from_str(){
        let agg_proof_str = r#"{"c_hash":"63330487197040957750863022608534150304998351350639315143102570772502292901825","c_list":[[1,180,153,212,162,132,5,189,14,181,140,112,236,109,182,76,91,6,161,215,62,207,205,135,86,211,49,197,215,198,104,201,14,22,48,6,112,170,31,191,110,118,121,15,62,114,126,249,221,107,114,161,163,234,19,233,150,236,182,217,195,6,218,217,193,6,94,160,33,23,103,147,109,221,81,38,138,20,225,141,68,37,142,10,225,79,164,119,168,250,188,186,47,229,165,8,237,230,14,35,53,176,97,28,82,105,87,210,117,16,154,222,66,11,96,172,90,13,239,190,29,71,11,88,53,36,219,139,67,21,136,58,161,164,97,106,56,230,55,157,59,35,187,235,154,194,111,93,168,135,67,15,97,136,38,169,87,142,32,255,50,247,111,83,44,88,251,99,6,226,182,170,146,229,118,164,118,228,235,51,137,168,135,50,1,14,1,201,72,175,102,241,149,117,88,83,84,37,205,130,26,155,124,158,211,89,112,33,46,24,94,93,202,8,127,172,214,178,6,156,79,188,132,223,239,127,200,158,95,247,139,101,51,162,168,175,74,1,67,201,94,108,192,14,130,109,217,248,193,10,142,37,95,231,227,251,209]]}"#;
        let agg_proof: AggregatedProof = serde_json::from_str(agg_proof_str).unwrap();
        assert_eq!(agg_proof.c_hash, "63330487197040957750863022608534150304998351350639315143102570772502292901825");
    }

    #[test]
    fn test_proof_from_str(){
        let proof = create_default_proof();
        assert_eq!(proof.proofs.get("claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4").unwrap().schema_seq_no, 103);
        assert_eq!(proof.requested_proof.revealed_attrs.get("attr1_uuid").unwrap()[1], serde_json::to_value("Alex").unwrap());
    }

    #[test]
    fn test_serialize_deserialize(){
        let proof = create_default_proof();
        let serialized = proof.to_string().unwrap();
        let proof2 = ProofMessage::from_str(&serialized).unwrap();
        assert_eq!(proof,proof2);
    }

    #[test]
    fn test_get_claim_data() {
        let proof = create_default_proof();
        let claim_data = proof.get_claim_info().unwrap();
        assert_eq!(claim_data[0].claim_uuid.clone().unwrap(), "claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4");
        assert_eq!(claim_data[0].issuer_did.clone().unwrap(), "V4SGRU86Z58d6TV7PBUe6f".to_string());
        assert_eq!(claim_data[0].schema_seq_no.clone().unwrap(), 103);
    }

    #[test]
    fn test_get_proof_attrs_multiple_claims() {
        let add_claim: Proofs = serde_json::from_str(r#"{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"t2":"Hash for 2"},"a_prime":"3","e":"2","v":"5","m":{"t2":"2"},"m1":"2","m2":"2"},"ge_proofs":[{"u":{"2":"2","0":"2","3":"2","1":"2"},"r":{"1":"2","3":"2","DELTA":"2","2":"2","0":"2"},"mj":"3","alpha":"2","t":{"0":"2","2":"2","DELTA":"4","1":"5","3":"3"},"predicate":{"attr_name":"predicate2","p_type":"LE","value":99,"schema_seq_no":778,"issuer_did":"12345"}}]},"non_revoc_proof":null},"schema_seq_no":778,"issuer_did":"12345"}"#).unwrap();
        let mut proof = create_default_proof();
        proof.proofs.insert("claim2_uuid".to_string(), add_claim);
        proof.requested_proof.predicates.insert("attr2".to_string(), "claim2_uuid".to_string());
        let revealed_list:  Vec<Value> = serde_json::from_str(r#"["claim2_uuid","t2_val","Hash for 2"]"#).unwrap();
        proof.requested_proof.revealed_attrs.insert("revealed_attr".to_string(), revealed_list);
        let attrs_str = proof.get_proof_attributes().unwrap();

        //Check revealed_attrs
        assert!(attrs_str.contains(r#"{"schema_seq_no":103,"issuer_did":"V4SGRU86Z58d6TV7PBUe6f","claim_uuid":"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","attr_info":{"name":"name","value":"Alex","type":"revealed"}}"#));
        assert!(attrs_str.contains(r#"{"schema_seq_no":778,"issuer_did":"12345","claim_uuid":"claim2_uuid","attr_info":{"name":"t2","value":"t2_val","type":"revealed"}}"#));

        //Check predicates
        assert!(attrs_str.contains(r#"{"schema_seq_no":778,"issuer_did":"12345","claim_uuid":"claim2_uuid","attr_info":{"name":"predicate2","value":99,"type":"predicate","predicate_type":"LE"}}"#));
        assert!(attrs_str.contains(r#"{"schema_seq_no":14,"issuer_did":"33UDR9R7fjwELRvH9JT6HH","claim_uuid":"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","attr_info":{"name":"age","value":18,"type":"predicate","predicate_type":"GE"}}"#));

        //Check self_attested
        assert!(attrs_str.contains(r#"{"name":"self_attr","value":"self_value","type":"self_attested"}"#));

        //Todo: Assert case with unrevealed
    }

    #[test]
    fn test_get_revealed_attrs_fails_with_inconsistent_hash() {
        let add_claim: Proofs = serde_json::from_str(r#"{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"t2":"Hash for 2"},"a_prime":"3","e":"2","v":"5","m":{"t2":"2"},"m1":"2","m2":"2"},"ge_proofs":[{"u":{"2":"2","0":"2","3":"2","1":"2"},"r":{"1":"2","3":"2","DELTA":"2","2":"2","0":"2"},"mj":"3","alpha":"2","t":{"0":"2","2":"2","DELTA":"4","1":"5","3":"3"},"predicate":{"attr_name":"predicate2","p_type":"LE","value":99,"schema_seq_no":778,"issuer_did":"12345"}}]},"non_revoc_proof":null},"schema_seq_no":778,"issuer_did":"12345"}"#).unwrap();
        let mut proof = create_default_proof();
        proof.proofs.insert("claim2_uuid".to_string(), add_claim);
        let revealed_list:  Vec<Value> = serde_json::from_str(r#"["claim2_uuid","t2_val","Wrong Hash for 2"]"#).unwrap();
        proof.requested_proof.revealed_attrs.insert("revealed_attr".to_string(), revealed_list);
        assert_eq!(proof.get_proof_attributes(), Err(error::INVALID_PROOF_CLAIM_DATA.code_num));
    }

    #[test]
    fn test_self_attested_attrs() {
        let mut proof = create_default_proof();
        proof.requested_proof.self_attested_attrs.insert("dog".to_string(), "ralph".to_string());
        proof.requested_proof.self_attested_attrs.insert("cat".to_string(), "sam".to_string());
        let attrs_str = proof.get_proof_attributes().unwrap();
        assert!(attrs_str.contains(r#""attr_info":{"name":"dog","value":"ralph","type":"self_attested"}"#));
        assert!(attrs_str.contains(r#""attr_info":{"name":"cat","value":"sam","type":"self_attested"}"#));
    }

    #[test]
    fn test_predicates() {
        let proof = create_default_proof();
        let attrs_str = proof.get_proof_attributes().unwrap();
        assert!(attrs_str.contains(r#"{"schema_seq_no":14,"issuer_did":"33UDR9R7fjwELRvH9JT6HH","claim_uuid":"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","attr_info":{"name":"age","value":18,"type":"predicate","predicate_type":"GE"}}"#));
    }

    #[test]
    fn test_predicate_fails_with_no_claim() {
        let mut proof = create_default_proof();
        proof.requested_proof.predicates.insert("attr1".to_string(), "NO_CLAIM".to_string());
        assert_eq!(proof.get_proof_attributes(), Err(error::INVALID_PROOF_CLAIM_DATA.code_num));
    }
}
