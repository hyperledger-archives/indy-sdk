use sovrin::api::ErrorCode;
use sovrin::api::anoncreds::{
    sovrin_issuer_create_and_store_claim_def,
    sovrin_issuer_create_claim,
    sovrin_prover_create_master_secret,
    sovrin_prover_create_and_store_claim_req,
    sovrin_prover_store_claim,
    sovrin_prover_get_claims_for_proof_req,
    sovrin_prover_create_proof,
    sovrin_prover_store_claim_offer,
    sovrin_prover_get_claim_offers,
    sovrin_verifier_verify_proof
};

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use utils::wallet::WalletUtils;

use std::ffi::CString;
use std::ptr::null;
use std::sync::mpsc::channel;
use std::collections::{HashMap, HashSet};

pub struct AnoncredsUtils {}

impl AnoncredsUtils {
    pub fn create_claim_definition_and_set_link(wallet_handle: i32, schema: &str, claim_def_seq_no: i32) -> Result<String, ErrorCode> {
        let (claim_def_json, claim_def_uuid) = AnoncredsUtils::issuer_create_claim_definition(wallet_handle, &schema)?;
        WalletUtils::wallet_set_seq_no_for_value(wallet_handle, &claim_def_uuid, claim_def_seq_no)?;
        Ok(claim_def_json)
    }

    pub fn issuer_create_claim_definition(wallet_handle: i32, schema: &str) -> Result<(String, String), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claim_def_json, claim_def_uuid| {
            sender.send((err, claim_def_json, claim_def_uuid)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_claim_definition_cb(cb);

        let schema = CString::new(schema).unwrap();

        let err =
            sovrin_issuer_create_and_store_claim_def(command_handle,
                                                     wallet_handle,
                                                     schema.as_ptr(),
                                                     null(),
                                                     false,
                                                     cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claim_def_json, claim_def_uuid) = receiver.recv().unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((claim_def_json, claim_def_uuid))
    }

    pub fn prover_create_master_secret(wallet_handle: i32, master_secret_name: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_create_master_secret_cb(cb);

        let master_secret_name = CString::new(master_secret_name).unwrap();

        let err = sovrin_prover_create_master_secret(command_handle,
                                                     wallet_handle,
                                                     master_secret_name.as_ptr(),
                                                     cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn prover_store_claim_offer(wallet_handle: i32, claim_offer_json: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_claim_offer_json_cb(cb);

        let claim_offer_json = CString::new(claim_offer_json).unwrap();

        let err = sovrin_prover_store_claim_offer(command_handle,
                                                  wallet_handle,
                                                  claim_offer_json.as_ptr(),
                                                  cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn prover_get_claim_offers(wallet_handle: i32, filter_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claim_offers_json| {
            sender.send((err, claim_offers_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_get_claim_offers_cb(cb);

        let filter_json = CString::new(filter_json).unwrap();

        let err = sovrin_prover_get_claim_offers(command_handle,
                                                 wallet_handle,
                                                 filter_json.as_ptr(),
                                                 cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claim_offers_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claim_offers_json)
    }

    pub fn prover_create_and_store_claim_req(wallet_handle: i32, prover_did: &str, claim_offer_json: &str,
                                             claim_def_json: &str, master_secret_name: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claim_req_json| {
            sender.send((err, claim_req_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_create_claim_req_cb(cb);

        let prover_did = CString::new(prover_did).unwrap();
        let claim_offer_json = CString::new(claim_offer_json).unwrap();
        let claim_def_json = CString::new(claim_def_json).unwrap();
        let master_secret_name = CString::new(master_secret_name).unwrap();

        let err = sovrin_prover_create_and_store_claim_req(command_handle,
                                                           wallet_handle,
                                                           prover_did.as_ptr(),
                                                           claim_offer_json.as_ptr(),
                                                           claim_def_json.as_ptr(),
                                                           master_secret_name.as_ptr(),
                                                           cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claim_req_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claim_req_json)
    }

    pub fn issuer_create_claim(wallet_handle: i32, claim_req_json: &str, claim_json: &str) -> Result<(String, String), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, revoc_reg_update_json, xclaim_json| {
            sender.send((err, revoc_reg_update_json, xclaim_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_claim_cb(cb);

        let claim_req_json = CString::new(claim_req_json).unwrap();
        let claim_json = CString::new(claim_json).unwrap();

        let err = sovrin_issuer_create_claim(command_handle,
                                             wallet_handle,
                                             claim_req_json.as_ptr(),
                                             claim_json.as_ptr(),
                                             None,
                                             None,
                                             cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, revoc_reg_update_json, xclaim_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((revoc_reg_update_json, xclaim_json))
    }

    pub fn prover_store_claim(wallet_handle: i32, claims_json: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_store_claim_cb(cb);

        let claims_json = CString::new(claims_json).unwrap();

        let err = sovrin_prover_store_claim(command_handle,
                                            wallet_handle,
                                            claims_json.as_ptr(),
                                            cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn prover_get_claims_for_proof_req(wallet_handle: i32, proof_request_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claims_json| {
            sender.send((err, claims_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_get_claims_for_proof_req_cb(cb);

        let proof_request_json = CString::new(proof_request_json).unwrap();

        let err = sovrin_prover_get_claims_for_proof_req(command_handle,
                                                         wallet_handle,
                                                         proof_request_json.as_ptr(),
                                                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claims_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claims_json)
    }

    pub fn prover_create_proof(wallet_handle: i32, proof_req_json: &str, requested_claims_json: &str,
                               schemas_json: &str, master_secret_name: &str, claim_defs_json: &str,
                               revoc_regs_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, proof_json| {
            sender.send((err, proof_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_get_claims_for_proof_req_cb(cb);

        let proof_req_json = CString::new(proof_req_json).unwrap();
        let requested_claims_json = CString::new(requested_claims_json).unwrap();
        let schemas_json = CString::new(schemas_json).unwrap();
        let master_secret_name = CString::new(master_secret_name).unwrap();
        let claim_defs_json = CString::new(claim_defs_json).unwrap();
        let revoc_regs_json = CString::new(revoc_regs_json).unwrap();

        let err = sovrin_prover_create_proof(command_handle,
                                             wallet_handle,
                                             proof_req_json.as_ptr(),
                                             requested_claims_json.as_ptr(),
                                             schemas_json.as_ptr(),
                                             master_secret_name.as_ptr(),
                                             claim_defs_json.as_ptr(),
                                             revoc_regs_json.as_ptr(),
                                             cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, proof_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(proof_json)
    }

    pub fn verifier_verify_proof(proof_request_json: &str, proof_json: &str,
                                 schemas_json: &str, claim_defs_json: &str, revoc_regs_json: &str) -> Result<bool, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, valid| {
            sender.send((err, valid)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_verifier_verify_proof_cb(cb);

        let proof_request_json = CString::new(proof_request_json).unwrap();
        let proof_json = CString::new(proof_json).unwrap();
        let schemas_json = CString::new(schemas_json).unwrap();
        let claim_defs_json = CString::new(claim_defs_json).unwrap();
        let revoc_regs_json = CString::new(revoc_regs_json).unwrap();

        let err = sovrin_verifier_verify_proof(command_handle,
                                               proof_request_json.as_ptr(),
                                               proof_json.as_ptr(),
                                               schemas_json.as_ptr(),
                                               claim_defs_json.as_ptr(),
                                               revoc_regs_json.as_ptr(),
                                               cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, valid) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(valid)
    }

    pub fn get_gvt_schema_json(schema_seq_no: i32) -> String {
        format!("{{\
                    \"name\":\"gvt\",\
                    \"version\":\"1.0\",\
                    \"attribute_names\":[\"age\",\"sex\",\"height\",\"name\"],\
                    \"seq_no\":{}\
                 }}", schema_seq_no)
    }

    pub fn get_xyz_schema_json(schema_seq_no: i32) -> String {
        format!("{{\
                    \"name\":\"xyz\",\
                    \"version\":\"1.0\",\
                    \"attribute_names\":[\"status\",\"period\"],\
                    \"seq_no\":{}\
                 }}", schema_seq_no)
    }

    pub fn get_claim_offer(issuer_did: &str, claim_def_seq_no: i32) -> String {
        format!("{{ \"issuer_did\":\"{}\", \"claim_def_seq_no\":{} }}",
                issuer_did, claim_def_seq_no)
    }

    pub fn get_gvt_claim_json() -> String {
        "{\
               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\
               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\
               \"height\":[\"175\",\"175\"],\
               \"age\":[\"28\",\"28\"]\
        }".to_string()
    }

    pub fn get_xyz_claim_json() -> String {
        "{\
               \"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\
               \"period\":[\"8\",\"8\"]\
        }".to_string()
    }

    pub fn get_unique_claims(proof_claims: &ProofClaimsJson) -> Vec<ClaimInfo> {
        let attrs_claims =
            proof_claims.attrs
                .values()
                .flat_map(|claims| claims)
                .map(|claim| claim.clone())
                .collect::<Vec<ClaimInfo>>();

        let predicates_claims =
            proof_claims.predicates
                .values()
                .flat_map(|claims| claims)
                .map(|claim| claim.clone())
                .collect::<Vec<ClaimInfo>>();

        attrs_claims.into_iter().collect::<HashSet<ClaimInfo>>()
            .union(&predicates_claims.into_iter().collect::<HashSet<ClaimInfo>>())
            .map(|v| v.clone()).collect::<Vec<ClaimInfo>>()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClaimOffer {
    pub issuer_did: String,
    pub claim_def_seq_no: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofClaimsJson {
    pub attrs: HashMap<String, Vec<ClaimInfo>>,
    pub predicates: HashMap<String, Vec<ClaimInfo>>
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ClaimInfo {
    pub claim_uuid: String,
    pub claim_def_seq_no: i32,
    pub revoc_reg_seq_no: Option<i32>,
    pub schema_seq_no: i32
}