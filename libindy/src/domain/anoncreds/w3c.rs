use indy_api_types::errors::{IndyResult, IndyErrorKind, IndyResultExt};
use rust_base58::ToBase58;
use super::proof::Proof;

/// Embodies the "proof" property at the root of a verifiable presentation.
/// This comes from the "aggregated_proof" field in the CL-based Ursa
/// struct. It essentially proves that all the credentials are associated
/// with the same link secret.
#[derive(Debug, Serialize, Deserialize)]
struct W3cPresentationProof {
    #[serde(rename = "type")]
    typ: String,
    #[serde(rename = "aggregateProof")]
    aggregate_proof: Option<String>,
}

impl W3cPresentationProof {
    fn from_proof(proof: &Proof) -> W3cPresentationProof {
        //let crypto_proof: &CryptoProof = &proof.proof;
        //let ag_result = serde_json::to_string(&crypto_proof.aggregated_proof);
        let mut ag: Option<String> = None;
        // Kludge. We need the JSON text for aggregated_proof. We'd like to just serialize
        // it here. However, it comes from a data member of our struct that is private. The
        // correct fix is probably to make that member public, but that requires a change to Ursa.
        // For now, just serialize the containing data structure, then pick out from it the part
        // we need. This is highly inefficient at runtime, but it's efficient in terms of
        // developer effort right now. TODO: go back and fix this tech debt by modifying Ursa.
        let crypto_proof_json = serde_json::to_string(&proof.proof).unwrap();
        use serde_json::Value;
        let v: Value = serde_json::from_str(&crypto_proof_json).unwrap(); // <-- yuck! expensive
        let ap = &v["aggregated_proof"];
        if ap.is_object() {
            ag = serde_json::to_string(ap).ok();
        }
        if ag.is_some() {
            ag = Some(ag.unwrap().as_bytes().to_base58());
        }
        W3cPresentationProof {
            typ: "SafeVPProof-v1".to_string(),
            aggregate_proof: ag
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaForDerivedCredential {
    id: String,
    #[serde(rename = "type")]
    typ: String
}

/// Embodies a VC inside a verifiable presentation, as opposed to a VC
/// as issued. Disclosed fields in the CL proof become fields under
/// "credentialSubject" in this credential. The "proof" field for each
/// of these credentials contains a primaryProof as well as a
/// nonRevocationProof.
#[derive(Debug, Serialize, Deserialize)]
struct DerivedCredential {
    #[serde(rename = "@context")]
    context: Vec<String>,
    #[serde(rename = "type")]
    typ: Vec<String>,
    issuer: String,
    #[serde(rename = "issuanceDate")]
    issuance_date: String,
    #[serde(rename = "credentialSchema")]
    credential_schema: SchemaForDerivedCredential,
    #[serde(rename = "credentialSubject")]
    credential_subject: Vec<serde_json::Value>,
}

impl DerivedCredential {
    fn from_proof(proof: &Proof, subproof_index: usize, ledger: &impl LedgerLookup) -> DerivedCredential {

        // Non-safe credentials often have a timestamp in the issuanceDate field.
        // This value can be quite precise and function as a strong correlator all on its
        // own. The VC spec says that the meaning of this field is "when the credential
        // begins to be valid." Thus, we can do some work here to make the derived credential
        // less identifying if we create a random issuance date for the derived credential,
        // sometime in the previous year. As long as the value we pick is before right now,
        // the derived credential will look different every time it is seen.
        use chrono::{Duration, SecondsFormat};
        use crate::rand::Rng;
        let mut rng = rand::thread_rng();
        let random_time_offset = rng.gen_range(0, 365*86400);
        let seconds_format = if random_time_offset % 2 == 1 { SecondsFormat::Secs } else { SecondsFormat::Millis };
        let mut fake_issuance_date = chrono::offset::Utc::now();
        if let Some(new_date) = fake_issuance_date.checked_sub_signed(
            Duration::seconds(random_time_offset)) {
            fake_issuance_date = new_date;
        }

        let primary_cred_def = proof.identifiers[0].cred_def_id.0.clone();
        DerivedCredential {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_string(),
            ],
            typ: vec![
                "VerifiableCredential".to_string(),
                "DerivedFromZKP".to_string()
            ],
            issuer: ledger.get_issuer_did_for_cred_def(&primary_cred_def),
            issuance_date: fake_issuance_date.to_rfc3339_opts(seconds_format, true),
            credential_schema: SchemaForDerivedCredential {
                id: primary_cred_def,
                typ: "DerivedSchema".to_string()
            },
            credential_subject: get_derived_cred_attribs(&proof, subproof_index),
        }
    }
}

fn get_derived_cred_attribs(_proof: &Proof, subproof_index: usize) -> Vec<serde_json::Value> {
    let attribs: Vec<serde_json::Value> = vec![];
    // TODO: the code below violates encapsulation because it peers inside private member
    // variables to get at the data it needs. I had to hack Ursa to get it to compile. I don't
    // think we want that, long-term. What we may want, instead, is to change from consuming
    // a Rust data structure to consuming simple JSON text. We could then pull out of the JSON
    // text the specific substructure we need, without worrrying about how that text maps to
    // internal structs in Rust, and without worrying about whether they are managed in Ursa
    // or libindy, or whether the specific pieces of data are private or not. This is how code
    // would work if it were converting anoncreds data to W3C format without any view into
    // libindy or Ursa internals. It would mean that we have to rewrite this function and some
    // other functions in this module so they take text instead of Proof objects.
    //let revealed = &proof.proof.proofs[subproof_index].primary_proof.eq_proof.revealed_attrs;
    //for (key, encoding) in &revealed {
    //
    //}
    attribs
}

/// Embodies a verifiable presentation containing one or more derived
/// credentials, plus an aggregate proof for the presentation as a whole.
/// This is the data structure that should be sent and received for
/// standards-based interoperability.
#[derive(Debug, Serialize, Deserialize)]
struct VerifiablePresentation {
    #[serde(rename = "@context")]
    context: Vec<String>,
    #[serde(rename = "type")]
    typ: String,
    #[serde(rename = "verifiableCredential")]
    creds: Vec<DerivedCredential>,
    proof: Option<W3cPresentationProof>,
}

/// Convert the JSON for a ZKP into the JSON for a W3C Verifiable
/// Presentation.
#[allow(dead_code)]
pub fn to_vp(proof: &Proof, ledger: &impl LedgerLookup) -> IndyResult<String> {
    let preso = VerifiablePresentation {
        context: vec![
            "https://www.w3.org/2018/credentials/v1".to_string(),
            "https://github.com/hyperledger/ursa/libzmix/docs/safecreds-context-v1.9.jsonld".to_string()
        ],
        typ: "VerifiablePresentation".to_string(),
        creds: get_derived_creds(proof, ledger),
        proof: Some(W3cPresentationProof::from_proof(proof)),
    };
    serde_json::to_string(&preso)
        .to_indy(IndyErrorKind::InvalidState, "Cannot serialize FullProof")
}

fn get_derived_creds(proof: &Proof, ledger: &impl LedgerLookup) -> Vec<DerivedCredential> {
    let mut items: Vec<DerivedCredential> = vec![];
    for i in 0..proof.proof.proofs.len() {
        items.push(DerivedCredential::from_proof(proof, i, ledger));
    }
    items
}

/// The purpose of this trait is to break a tight coupling between this module and the guts of ledger
/// functionality. In production, we will certainly use the ledger to look up cred defs, schemas,
/// and so forth. However, if we were to directly expose ledger lookup calls to this layer, much
/// of our code would have to change to take parameters like a command handle, a pool handle,
/// callbacks, and so forth. Instead, we define an interface that can be satisfied by a struct that
/// holds all the necessary parameters, without cluttering up this module. Then, in tests, we can
/// implement the functionality in a mock to avoid tight coupling.
pub trait LedgerLookup {
    fn get_issuer_did_for_cred_def(&self, cred_def_id: &str) -> String;
}

pub struct Ledger {
}

impl LedgerLookup for Ledger {
    fn get_issuer_did_for_cred_def(&self, _cred_def_id: &str) -> String {
        "did:sov:9TifxAwchk6tRrZRvzh5ya".to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn to_vp_works() {
        let mock_ledger = MockLedger {};
        let proof: Proof = serde_json::from_str(FAKE_PROOF_JSON).unwrap();
        let vp = to_vp(&proof, &mock_ledger).unwrap();
        let mut errors: Vec<String> = Vec::new();
        let v: Value = serde_json::from_str(&vp).unwrap();

        check_structure(&v, "@context", "HAS https://www.w3.org/2018/credentials/v1", &mut errors);
        check_structure(&v, "@context", "HAS https://github.com/hyperledger/ursa/.*safecreds.*context.*[.]jsonld", &mut errors);
        check_structure(&v, "type", "LIKE VerifiablePresentation", &mut errors);
        if check_structure(&v, "verifiableCredential", "is array", &mut errors) {
            let vcs = v["verifiableCredential"].as_array().unwrap();
            let mut i: usize = 0;
            for vc in vcs {
                check_vc(&vc, i, &mut errors);
                i += 1;
            }
        }
        if check_structure(&v, "proof", "is object", &mut errors) {
            check_structure(&v["proof"], "proof/type", r"LIKE SafeVPProof-v\d", &mut errors);
            check_structure(&v["proof"], "proof/aggregateProof", "LIKE ^[123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz]{50,}$", &mut errors);
        }

        if !errors.is_empty() {
            panic!("W3C presentation structure has errors: {}.\n\nPresentation was: {}",
                   &errors.join(". "), &vp);
        }
    }

    fn array_has_value(candidate: &Value, value: &str) -> bool {
        if candidate.is_array() {
            let ar = candidate.as_array().unwrap();
            for i in 0..ar.len() {
                let item = ar[i].to_string();
                // Ignore the delimiting quotes around str value. Compare inner only.
                let mut txt = item.as_str();
                let bytes = txt.as_bytes();
                if bytes.len() >= 2 && (bytes[0] == b'"') && (bytes[bytes.len() - 1] == b'"') {
                    txt = &item.as_str()[1..item.len() - 1];
                }
                use regex::Regex;
                let pat = Regex::new(value).unwrap();
                if pat.is_match(txt) {
                    return true;
                }
            }
        }
        false
    }

    fn text_matches_regex(candidate: &Value, regex: &str) -> bool {
        if candidate.is_string() {
            use regex::Regex;
            let pat = Regex::new(regex).unwrap();
            if pat.is_match(candidate.as_str().unwrap()) {
                return true;
            }
        }
        false
    }

    fn check_structure(container: &Value, path: &str, expected: &str, errors: &mut Vec<String>) -> bool {
        let mut ok = false;
        let i = path.rfind('/');
        let subitem = if i.is_some() { &path[i.unwrap() + 1..] } else { &path[..] };
        let item = &container[subitem];
        if !item.is_null() {
            match expected {
                "is array" => ok = item.is_array(),
                "is object" => ok = item.is_object(),
                "is number" => ok = item.is_number(),
                "is string" => ok = item.is_string(),
                _ => {
                    if expected[0..4].eq("HAS ") {
                        ok = array_has_value(item, &expected[4..]);
                    } else if expected[0..5].eq("LIKE ") {
                        ok = text_matches_regex(item, &expected[5..]);
                    }
                }
            }
        }
        if !ok {
            errors.push(format!("Expected {} {}", path.to_string(), expected));
        }
        ok
    }

    fn check_vc(vc: &Value, i: usize, errors: &mut Vec<String>) {
        let prefix = format!("verifiableCredential[{}]", i);
        // Make this function a bit less verbose/repetitive.
        macro_rules! check {( $item:expr, $path:expr, $ex:expr ) => {
            check_structure($item, format!($path, &prefix).as_str(), $ex, errors) }}
        check!(&vc, "{}/type", "HAS VerifiableCredential");
        check!(&vc, "{}/type", "HAS DerivedFromZKP");
        check!(&vc, "{}/@context", "HAS https://www.w3.org/2018/credentials/v1");
        if check!(&vc, "{}/credentialSchema", "is object") {
            let sch = &vc["credentialSchema"];
            check!(&sch, "{}/credentialSchema/id", "LIKE :gvt:1.0:TAG_1$");
            check!(&sch, "{}/credentialSchema/type", "LIKE ^DerivedSchema");
        }
        check!(&vc, "{}/issuer", "LIKE ^did:");
        check!(&vc, "{}/issuanceDate", r"LIKE ^20\d\d-\d\d-\d\dT\d\d:\d\d.*Z");
        if check!(&vc, "{}/credentialSubject", "is array") {
            //let vcs = v["verifiableCredential"].as_array().unwrap();
            //let mut i: usize = 0;
            //for vc in vcs {
            //    check_vc(&vc, i, &mut errors);
            //    i += 1;
            //}
        }
    }

    struct MockLedger {
    }

    impl LedgerLookup for MockLedger {
        fn get_issuer_did_for_cred_def(&self, _cred_def_id: &str) -> String {
            "did:sov:9TifxAwchk6tRrZRvzh5ya".to_string()
        }
    }

    // This JSON exhibits the actual structure of a proof, but numeric values
    // are wrong and strings have been shortened. Thus, it should deserialize
    // correctly but will not validate.
    const FAKE_PROOF_JSON: &'static str = r#"{
  "proof":{
    "proofs":[
      {
        "primary_proof":{
          "eq_proof":{
            "revealed_attrs":{
              "height":"175",
              "name":"1139481716457488690172217916278103335"
            },
            "a_prime":"5817705...096889",
            "e":"1270938...756380",
            "v":"1138...39984052",
            "m":{
              "master_secret":"375275...0939395",
              "sex":"3511483...897083518",
              "age":"13430...63372249"
            },
            "m2":"1444497...2278453"
          },
          "ge_proofs":[
            {
              "u":{
                "1":"152500...3999140",
                "2":"147748...2005753",
                "0":"8806...77968",
                "3":"10403...8538260"
              },
              "r":{
                "2":"15706...781609",
                "3":"343...4378642",
                "0":"59003...702140",
                "DELTA":"9607...28201020",
                "1":"180097...96766"
              },
              "mj":"134300...249",
              "alpha":"827896...52261",
              "t":{
                "2":"7132...47794",
                "3":"38051...27372",
                "DELTA":"68025...508719",
                "1":"32924...41082",
                "0":"74906...07857"
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
    ],
    "aggregated_proof":{
      "c_hash":"108743...92564",
      "c_list":[
        [0,1,2,3,4,255],
        [0,1,2,3,4,255],
        [0,1,2,3,4,255],
        [0,1,2,3,4,255],
        [0,1,2,3,4,255],
        [0,1,2,3,4,255]
      ]
    }
  },
  "requested_proof":{
    "revealed_attrs":{
      "attr1_referent":{
        "sub_proof_index":0,
        "raw":"Alex",
        "encoded":"1139481716457488690172217916278103335"
      }
    },
    "revealed_attr_groups":{
      "attr4_referent":{
        "sub_proof_index":0,
        "values":{
          "name":{
            "raw":"Alex",
            "encoded":"1139481716457488690172217916278103335"
          },
          "height":{
            "raw":"175",
            "encoded":"175"
          }
        }
      }
    },
    "self_attested_attrs":{
      "attr3_referent":"8-800-300"
    },
    "unrevealed_attrs":{
      "attr2_referent":{
        "sub_proof_index":0
      }
    },
    "predicates":{
      "predicate1_referent":{
        "sub_proof_index":0
      }
    }
  },
  "identifiers":[
    {
      "schema_id":"NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0",
      "cred_def_id":"NcYxi...cYDi1e:2:gvt:1.0:TAG_1",
      "rev_reg_id":null,
      "timestamp":null
    }
  ]
}"#;

}
