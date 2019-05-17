use serde_json;
use futures::Future;
use indy::ledger;

use settings;
use utils::libindy::pool::get_pool_handle;
use utils::libindy::wallet::get_wallet_handle;
use utils::libindy::error_codes::map_rust_indy_sdk_error;
use error::prelude::*;

pub fn multisign_request(did: &str, request: &str) -> VcxResult<String> {
    ledger::multi_sign_request(get_wallet_handle(), did, request)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_sign_request(did: &str, request: &str) -> VcxResult<String> {
    ledger::sign_request(get_wallet_handle(), did, request)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_sign_and_submit_request(issuer_did: &str, request_json: &str) -> VcxResult<String> {
    if settings::test_indy_mode_enabled() { return Ok(r#"{"rc":"success"}"#.to_string()); }

    let pool_handle = get_pool_handle()?;
    let wallet_handle = get_wallet_handle();

    ledger::sign_and_submit_request(pool_handle, wallet_handle, issuer_did, request_json)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_submit_request(request_json: &str) -> VcxResult<String> {
    let pool_handle = get_pool_handle()?;

    //TODO there was timeout here (before future-based Rust wrapper)
    ledger::submit_request(pool_handle, request_json)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_build_get_txn_request(submitter_did: &str, sequence_num: i32) -> VcxResult<String> {
    ledger::build_get_txn_request(Some(submitter_did), None, sequence_num)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_build_schema_request(submitter_did: &str, data: &str) -> VcxResult<String> {
    ledger::build_schema_request(submitter_did, data)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_build_get_schema_request(submitter_did: &str, schema_id: &str) -> VcxResult<String> {
    ledger::build_get_schema_request(Some(submitter_did), schema_id)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_parse_get_schema_response(get_schema_response: &str) -> VcxResult<(String, String)> {
    ledger::parse_get_schema_response(get_schema_response)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_parse_get_cred_def_response(get_cred_def_response: &str) -> VcxResult<(String, String)> {
    ledger::parse_get_cred_def_response(get_cred_def_response)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_build_get_credential_def_txn(cred_def_id: &str) -> VcxResult<String> {
    let submitter_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;
    ledger::build_get_cred_def_request(Some(&submitter_did), cred_def_id)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_build_create_credential_def_txn(submitter_did: &str,
                                               credential_def_json: &str) -> VcxResult<String> {
    ledger::build_cred_def_request(submitter_did, credential_def_json)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_get_txn_author_agreement() -> VcxResult<String> {
    trace!("libindy_get_txn_author_agreement >>>");

    if settings::test_indy_mode_enabled() { return Ok(::utils::constants::DEFAULT_AUTHOR_AGREEMENT.to_string()); }

    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;

    let get_author_agreement_request = ledger::build_get_txn_author_agreement_request(Some(&did), None)
        .wait()
        .map_err(map_rust_indy_sdk_error)?;

    let get_author_agreement_response = libindy_submit_request(&get_author_agreement_request)?;

    let get_author_agreement_response = serde_json::from_str::<serde_json::Value>(&get_author_agreement_response)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidLedgerResponse, format!("{:?}", err)))?;

    let mut author_agreement_data = get_author_agreement_response["result"]["data"].as_object()
        .map_or(json!({}), |data| json!(data));

    let get_acceptance_mechanism_request = ledger::build_get_acceptance_mechanism_request(Some(&did), None, None)
        .wait()
        .map_err(map_rust_indy_sdk_error)?;

    let get_acceptance_mechanism_response = libindy_submit_request(&get_acceptance_mechanism_request)?;

    let get_acceptance_mechanism_response = serde_json::from_str::<serde_json::Value>(&get_acceptance_mechanism_response)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidLedgerResponse, format!("{:?}", err)))?;

    if let Some(aml) = get_acceptance_mechanism_response["result"]["data"]["aml"].as_object() {
        author_agreement_data["aml"] = json!(aml);
    }

    Ok(author_agreement_data.to_string())
}

pub fn append_txn_author_agreement_to_request(request_json: &str) -> VcxResult<String> {
    if let Some(author_agreement) = ::utils::author_agreement::get_txn_author_agreement().unwrap() {
        ledger::append_txn_author_agreement_acceptance_to_request(request_json,
                                                                  author_agreement.text.as_ref().map(String::as_str),
                                                                  author_agreement.version.as_ref().map(String::as_str),
                                                                  author_agreement.taa_digest.as_ref().map(String::as_str),
                                                                  &author_agreement.acceptance_mechanism_type,
                                                                  author_agreement.time_of_acceptance)
            .wait()
            .map_err(map_rust_indy_sdk_error)
    } else {
        Ok(request_json.to_string())
    }
}

pub fn libindy_build_auth_rule_request(submitter_did: &str, txn_type: &str, action: &str, field: &str,
                                       old_value: Option<&str>, new_value: Option<&str>, constraint_json: &str) -> VcxResult<String> {
    ledger::build_auth_rule_request(submitter_did, txn_type, action, field, old_value, new_value, constraint_json)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_build_get_auth_rule_request(submitter_did: Option<&str>, txn_type: Option<&str>, action: Option<&str>, field: Option<&str>,
                                           old_value: Option<&str>, new_value: Option<&str>) -> VcxResult<String> {
    ledger::build_get_auth_rule_request(submitter_did, txn_type, action, field, old_value, new_value)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub mod auth_rule {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use std::collections::hash_map::Entry;
    use std::sync::{Once, ONCE_INIT};
    use std::sync::Mutex;

    use indy::future::Future;

    /**
    Structure for parsing GET_AUTH_RULE response
     # parameters
        result - the payload containing data relevant to the GET_AUTH_RULE transaction
    */
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetAuthRuleResponse {
        pub result: GetAuthRuleResult
    }

    /**
       Structure of the result value within the GAT_AUTH_RULE response
        # parameters
       identifier - The DID this request was submitted from
       req_id - Unique ID number of the request with transaction
       txn_type - the type of transaction that was submitted
       data - A key:value map with the action id as the key and the auth rule as the value
   */
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetAuthRuleResult {
        pub identifier: String,
        pub req_id: u64,
        // This is to change the json key to adhear to the functionality on ledger
        #[serde(rename = "type")]
        pub txn_type: String,
        pub data: HashMap<String, Constraint>,
    }

    /**
       Enum of the constraint type within the GAT_AUTH_RULE result data
        # parameters
       ROLE - The final constraint
       AND - Combine multiple constraints all of them must be met
       OR - Combine multiple constraints any of them must be met
   */
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "constraint_id")]
    pub enum Constraint {
        #[serde(rename = "OR")]
        OrConstraint(CombinationConstraint),
        #[serde(rename = "AND")]
        AndConstraint(CombinationConstraint),
        #[serde(rename = "ROLE")]
        RoleConstraint(RoleConstraint),
    }

    /**
       The final constraint
        # parameters
       sig_count - The number of signatures required to execution action
       role - The role which the user must have to execute the action.
       metadata -  An additional parameters of the constraint (contains transaction FEE cost).
       need_to_be_owner - The flag specifying if a user must be an owner of the transaction.
   */
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct RoleConstraint {
        pub sig_count: Option<u32>,
        pub role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metadata: Option<Metadata>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub need_to_be_owner: Option<bool>,
    }

    /**
       The constraint metadata
        # parameters
       fees - The action cost
   */
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Metadata {
        pub fees: Option<String>,
    }

    /**
       Combine multiple constraints
        # parameters
       auth_constraints - The type of the combination
   */
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CombinationConstraint {
        pub auth_constraints: Vec<Constraint>
    }

    /* Map contains default Auth Rules set on the Ledger*/
    lazy_static! {
        static ref AUTH_RULES: Mutex<HashMap<String, Vec<AuthRule>>> = Default::default();
    }

    /* Helper structure to store auth rule set on the Ledger */
    #[derive(Debug)]
    struct AuthRule {
        action: String,
        txn_type: String,
        field: String,
        old_value: Option<String>,
        new_value: Option<String>,
        constraint: Constraint
    }

    // Helpers to set fee alias for auth rules
    pub fn set_actions_fee_aliases(submitter_did: &str, rules_fee: &str) -> VcxResult<()> {
        _get_default_ledger_auth_rules();

        let auth_rules = AUTH_RULES.lock().unwrap();

        let fees: HashMap<String, String> = ::serde_json::from_str(rules_fee)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize fees: {:?}", err)))?;

        let mut responses: Vec<Box<Future<Item=String, Error=::indy::IndyError>>> = Vec::new();

        for (txn_, fee_alias) in fees {
            if let Some(rules) = auth_rules.get(&txn_) {
                for auth_rule in rules {
                    let mut constraint = auth_rule.constraint.clone();
                    _set_fee_to_constraint(&mut constraint, &fee_alias);
                    responses.push(_send_auth_rule(submitter_did, auth_rule, &constraint)?);
                }
            }
        }

        responses
            .into_iter()
            .map(|response| _check_auth_rule_responses(response))
            .collect::<VcxResult<Vec<()>>>()?;

        Ok(())
    }

    fn _send_auth_rule(submitter_did: &str, auth_rule: &AuthRule, constraint: &Constraint) -> VcxResult<Box<Future<Item=String, Error=::indy::IndyError>>> {
        let constraint_json = ::serde_json::to_string(&constraint)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidLedgerResponse, format!("{:?}", err)))?;

        let auth_rule_request = libindy_build_auth_rule_request(submitter_did,
                                                                &auth_rule.txn_type,
                                                                &auth_rule.action,
                                                                &auth_rule.field,
                                                                auth_rule.old_value.as_ref().map(String::as_str),
                                                                auth_rule.new_value.as_ref().map(String::as_str),
                                                                &constraint_json)?;

        let response = ledger::sign_and_submit_request(get_pool_handle()?, get_wallet_handle(), submitter_did, &auth_rule_request);
        Ok(response)
    }

    fn _check_auth_rule_responses(response: Box<Future<Item=String, Error=::indy::IndyError>>) -> VcxResult<()> {
        let response = response.wait().map_err(map_rust_indy_sdk_error)?;

        let response: serde_json::Value = ::serde_json::from_str(&response)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidLedgerResponse, format!("{:?}", err)))?;

        match response["op"].as_str().unwrap_or_default() {
            "REPLY" => Ok(()),
            _ => Err(VcxError::from(VcxErrorKind::InvalidLedgerResponse))
        }
    }

    fn _get_default_ledger_auth_rules() {
        lazy_static! {
            static ref GET_DEFAULT_AUTH_CONSTRAINTS: Once = ONCE_INIT;

        }

        GET_DEFAULT_AUTH_CONSTRAINTS.call_once(|| {
            let get_auth_rule_request = ::indy::ledger::build_get_auth_rule_request(None, None, None, None, None, None).wait().unwrap();
            let get_auth_rule_response = ::utils::libindy::ledger::libindy_submit_request(&get_auth_rule_request).unwrap();

            let mut response: super::auth_rule::GetAuthRuleResponse = ::serde_json::from_str(&get_auth_rule_response)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidLedgerResponse, format!("{:?}", err))).unwrap();

            for (constraint_id, constraint) in response.result.data.iter_mut() {
                let parts: Vec<&str> = constraint_id.split("--").collect();

                let txn_type = parts[0].to_string();
                let action = parts[1].to_string();
                let field = parts[2].to_string();
                let old_value = if action == "ADD" { None } else { Some(parts[3].to_string()) };
                let new_value = if parts[4] == "" { None } else { Some(parts[4].to_string()) };

                let mut map = AUTH_RULES.lock().unwrap();

                let rule = AuthRule { action, txn_type: txn_type.clone(), field, old_value, new_value, constraint: constraint.clone() };

                match map.entry(txn_type) {
                    Entry::Occupied(rules) => {
                        let &mut ref mut rules = rules.into_mut();
                        rules.push(rule);
                    }
                    Entry::Vacant(rules) => {
                        rules.insert(vec![rule]);
                    }
                };
            }
        })
    }

    fn _set_fee_to_constraint(constraint: &mut Constraint, fee_alias: &str) {
        match constraint {
            Constraint::RoleConstraint(constraint) => {
                constraint.metadata.as_mut().map(|meta| meta.fees = Some(fee_alias.to_string()));
            }
            Constraint::AndConstraint(constraint) | Constraint::OrConstraint(constraint) => {
                for mut constraint in constraint.auth_constraints.iter_mut() {
                    _set_fee_to_constraint(&mut constraint, fee_alias)
                }
            }
        }
    }

    pub fn get_action_fee_alias(action: (&str, &str, &str, Option<&str>, &str)) -> VcxResult<Option<String>> {
        let (txn_type, action, field, old_value, new_value) = action;

        if settings::test_indy_mode_enabled() { return Ok(Some(txn_type.to_string())); }

        let constraint = _get_action_constraint(txn_type, action, field, old_value, Some(new_value))?;
        _extract_fee_alias_from_constraint(&constraint, None)
    }

    fn _get_action_constraint(txn_type: &str, action: &str, field: &str,
                              old_value: Option<&str>, new_value: Option<&str>) -> VcxResult<Constraint> {
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;

        let request = libindy_build_get_auth_rule_request(Some(&did), Some(txn_type), Some(action), Some(field), old_value, new_value)?;

        let response = libindy_submit_request(&request)?;

        let mut response: GetAuthRuleResponse = ::serde_json::from_str(&response)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidLedgerResponse, format!("{:?}", err)))?;

        let constraint_id = _build_constraint_id(txn_type, action, field, old_value, new_value);

        response.result.data.remove(&constraint_id)
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidLedgerResponse, format!("Auth Rule not found for id: {}", constraint_id)))
    }

    fn _extract_fee_alias_from_constraint(constraint: &Constraint, cur_fee: Option<String>) -> VcxResult<Option<String>> {
        let fee = match constraint {
            Constraint::RoleConstraint(constraint) => {
                constraint.metadata.as_ref().and_then(|metadata| metadata.fees.clone())
            }
            Constraint::AndConstraint(constraint) | Constraint::OrConstraint(constraint) => {
                let fees: HashSet<Option<String>> = constraint.auth_constraints
                    .iter()
                    .map(|constraint| _extract_fee_alias_from_constraint(constraint, cur_fee.clone()))
                    .collect::<VcxResult<HashSet<Option<String>>>>()?;
                if fees.len() != 1 {
                    return Err(VcxError::from_msg(VcxErrorKind::InvalidLedgerResponse, format!("There are multiple different fees: {:?}", fees)));
                }

                fees.into_iter().next().unwrap()
            }
        };

        match (cur_fee, fee) {
            (None, None) => Ok(None),
            (Some(cur_fee_), None) => Ok(Some(cur_fee_)),
            (None, Some(fee)) => Ok(Some(fee)),
            (Some(cur_fee_), Some(fee)) => {
                if cur_fee_ != fee {
                    return Err(VcxError::from_msg(VcxErrorKind::InvalidLedgerResponse, format!("Fee values are different. current fee: {}, new fee: {}", cur_fee_, fee)));
                } else {
                    Ok(Some(cur_fee_))
                }
            }
        }
    }

    fn _build_constraint_id(auth_type: &str,
                            auth_action: &str,
                            field: &str,
                            old_value: Option<&str>,
                            new_value: Option<&str>) -> String {
        let default_old_value = if auth_action == "ADD" { "*" } else { "" };
        format!("{}--{}--{}--{}--{}", auth_type, auth_action, field, old_value.unwrap_or(default_old_value), new_value.unwrap_or(""))
    }
}

pub fn parse_response(response: &str) -> VcxResult<Response> {
    serde_json::from_str::<Response>(response)
        .to_vcx(VcxErrorKind::InvalidJson, "Cannot deserialize transaction response")
}

#[serde(tag = "op")]
#[derive(Deserialize, Debug)]
pub enum Response {
    #[serde(rename = "REQNACK")]
    ReqNACK(Reject),
    #[serde(rename = "REJECT")]
    Reject(Reject),
    #[serde(rename = "REPLY")]
    Reply(Reply),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reject {
    pub reason: String
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Reply {
    ReplyV0(ReplyV0),
    ReplyV1(ReplyV1)
}

#[derive(Debug, Deserialize)]
pub struct ReplyV0 {
    pub result: serde_json::Value
}

#[derive(Debug, Deserialize)]
pub struct ReplyV1 {
    pub data: ReplyDataV1
}

#[derive(Debug, Deserialize)]
pub struct ReplyDataV1 {
    pub  result: serde_json::Value
}
