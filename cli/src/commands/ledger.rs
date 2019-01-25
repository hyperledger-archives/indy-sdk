extern crate regex;
extern crate chrono;

use command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;
use commands::payment_address::handle_payment_error;

use indy::{ErrorCode, IndyError};
use libindy::ledger::Ledger;
use libindy::payment::Payment;

use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use std::collections::{HashMap, BTreeMap};
use utils::table::{print_table, print_list_table};

use self::regex::Regex;
use self::chrono::prelude::*;

pub const DELIMITER: &'static str = ":";
pub const SCHEMA_MARKER: &'static str = "2";
pub const CRED_DEF_MARKER: &'static str = "3";

pub fn build_schema_id(did: &str, name: &str, version: &str) -> String {
    format!("{}{}{}{}{}{}{}", did, DELIMITER, SCHEMA_MARKER, DELIMITER, name, DELIMITER, version)
}

pub fn build_cred_def_id(did: &str, schema_id: &str, signature_type: &str, tag: &str) -> String {
    format!("{}{}{}{}{}{}{}{}{}", did, DELIMITER, CRED_DEF_MARKER, DELIMITER, signature_type, DELIMITER, schema_id, DELIMITER, tag)
}

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("ledger", "Ledger management commands"));
}

pub mod nym_command {
    use super::*;

    command!(CommandMetadata::build("nym", "Send NYM transaction to the Ledger.")
                .add_required_param("did", "DID of new identity")
                .add_optional_param("verkey", "Verification key of new identity")
                .add_optional_param("role", "Role of identity. One of: STEWARD, TRUSTEE, TRUST_ANCHOR, NETWORK_MONITOR or empty in case of blacklisting NYM")
                .add_optional_param("fees_inputs","The list of source inputs")
                .add_optional_param("fees_outputs","The list of outputs in the following format: (recipient, amount)")
                .add_optional_param("extra","Optional information for fees payment operation")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX verkey=GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX role=TRUSTEE")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX role=")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX fees_inputs=pay:null:111_rBuQo2A1sc9jrJg fees_outputs=(pay:null:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100)")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let verkey = get_opt_str_param("verkey", params).map_err(error_err!())?;
        let role = get_opt_empty_str_param("role", params).map_err(error_err!())?;
        let fees_inputs = get_opt_str_array_param("fees_inputs", params).map_err(error_err!())?;
        let fees_outputs = get_opt_str_tuple_array_param("fees_outputs", params).map_err(error_err!())?;
        let extra = get_opt_str_param("extra", params).map_err(error_err!())?;

        let mut request = Ledger::build_nym_request(&submitter_did, target_did, verkey, None, role)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let payment_method = set_request_fees(&mut request, wallet_handle, Some(&submitter_did), &fees_inputs, &fees_outputs, extra)?;

        let response_json = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request)
            .map_err(|err| handle_indy_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

        let mut response: Response<serde_json::Value> = serde_json::from_str::<Response<serde_json::Value>>(&response_json)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        if let Some(result) = response.result.as_mut() {
            result["txn"]["data"]["role"] = get_role_title(&result["txn"]["data"]["role"]);
            result["role"] = get_role_title(&result["role"]);
        }

        handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Nym request has been sent to Ledger.",
                                                     None,
                                                     &mut vec![("dest", "Did"),
                                                               ("verkey", "Verkey"),
                                                               ("role", "Role")]))?;

        let receipts = parse_response_with_fees(&response_json, payment_method)?;

        let res = print_response_receipts(receipts);

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_nym_command {
    use super::*;

    command!(CommandMetadata::build("get-nym", "Get NYM from Ledger.")
                .add_required_param("did","DID of identity presented in Ledger")
                .add_example("ledger get-nym did=VsKV7grR1BUE29mG2Fm2kX")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let submitter_did = get_active_did(&ctx);

        let target_did = get_str_param("did", params).map_err(error_err!())?;

        let response = Ledger::build_get_nym_request(submitter_did.as_ref().map(String::as_str), target_did)
            .and_then(|request| Ledger::submit_request(pool_handle, &request))
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let mut response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        if let Some(result) = response.result.as_mut() {
            let data = serde_json::from_str::<serde_json::Value>(&result["data"].as_str().unwrap_or(""));
            match data {
                Ok(mut data) => {
                    data["role"] = get_role_title(&data["role"]);
                    result["data"] = data;
                }
                Err(_) => return Err(println_err!("NYM not found"))
            };
        };

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Following NYM has been received.",
                                                     Some("data"),
                                                     &[("identifier", "Identifier"),
                                                         ("dest", "Dest"),
                                                         ("verkey", "Verkey"),
                                                         ("role", "Role")]));
        trace!("execute << {:?}", res);
        res
    }
}

pub mod attrib_command {
    use super::*;

    command!(CommandMetadata::build("attrib", "Send Attribute transaction to the Ledger for exists NYM.")
                .add_required_param("did",  "DID of identity presented in Ledger")
                .add_optional_param("hash", "Hash of attribute data")
                .add_optional_param("raw", "JSON representation of attribute data")
                .add_optional_param("enc", "Encrypted attribute data")
                .add_optional_param("fees_inputs","The list of source inputs")
                .add_optional_param("fees_outputs","The list of outputs in the following format: (recipient, amount)")
                .add_optional_param("extra","Optional information for fees payment operation")
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX raw={"endpoint":{"ha":"127.0.0.1:5555"}}"#)
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX hash=83d907821df1c87db829e96569a11f6fc2e7880acba5e43d07ab786959e13bd3"#)
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX enc=aa3f41f619aa7e5e6b6d0d"#)
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX enc=aa3f41f619aa7e5e6b6d0d fees_inputs=pay:null:111_rBuQo2A1sc9jrJg fees_outputs=(pay:null:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100)"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let hash = get_opt_str_param("hash", params).map_err(error_err!())?;
        let raw = get_opt_str_param("raw", params).map_err(error_err!())?;
        let enc = get_opt_str_param("enc", params).map_err(error_err!())?;
        let fees_inputs = get_opt_str_array_param("fees_inputs", params).map_err(error_err!())?;
        let fees_outputs = get_opt_str_tuple_array_param("fees_outputs", params).map_err(error_err!())?;
        let extra = get_opt_str_param("extra", params).map_err(error_err!())?;

        let mut request = Ledger::build_attrib_request(&submitter_did, target_did, hash, raw, enc)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let payment_method = set_request_fees(&mut request, wallet_handle, Some(&submitter_did), &fees_inputs, &fees_outputs, extra)?;

        let response_json = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request)
            .map_err(|err| handle_indy_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response_json)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        let attribute =
            if raw.is_some() {
                ("raw", "Raw value")
            } else if hash.is_some() {
                ("hash", "Hashed value")
            } else { ("enc", "Encrypted value") };

        handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Attrib request has been sent to Ledger.",
                                                     None,
                                                     &[attribute]))?;

        let receipts = parse_response_with_fees(&response_json, payment_method)?;

        let res = print_response_receipts(receipts);

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_attrib_command {
    use super::*;

    command!(CommandMetadata::build("get-attrib", "Get ATTRIB from Ledger.")
                .add_required_param("did", "DID of identity presented in Ledger")
                .add_optional_param("raw", "Name of attribute")
                .add_optional_param("hash", "Hash of attribute data")
                .add_optional_param("enc", "Encrypted value of attribute data")
                .add_example("ledger get-attrib did=VsKV7grR1BUE29mG2Fm2kX raw=endpoint")
                .add_example("ledger get-attrib did=VsKV7grR1BUE29mG2Fm2kX hash=83d907821df1c87db829e96569a11f6fc2e7880acba5e43d07ab786959e13bd3")
                .add_example("ledger get-attrib did=VsKV7grR1BUE29mG2Fm2kX enc=aa3f41f619aa7e5e6b6d0d")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let submitter_did = get_active_did(&ctx);

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let raw = get_opt_str_param("raw", params).map_err(error_err!())?;
        let hash = get_opt_str_param("hash", params).map_err(error_err!())?;
        let enc = get_opt_str_param("enc", params).map_err(error_err!())?;

        let response = Ledger::build_get_attrib_request(submitter_did.as_ref().map(String::as_str), target_did, raw, hash, enc)
            .and_then(|request| Ledger::submit_request(pool_handle, &request))
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let mut response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        if let Some(result) = response.result.as_mut() {
            let data = result["data"].as_str().map(|data| serde_json::Value::String(data.to_string()));
            match data {
                Some(data) => { result["data"] = data; }
                None => return Err(println_err!("Attribute not found"))
            };
        };

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Following ATTRIB has been received.",
                                                     None,
                                                     &[("data", "Data")]));
        trace!("execute << {:?}", res);
        res
    }
}

pub mod schema_command {
    use super::*;

    command!(CommandMetadata::build("schema", "Send Schema transaction to the Ledger.")
                .add_required_param("name", "Schema name")
                .add_required_param("version", "Schema version")
                .add_required_param("attr_names", "Schema attributes split by comma (the number of attributes should be less or equal than 125)")
                .add_optional_param("fees_inputs","The list of source inputs")
                .add_optional_param("fees_outputs","The list of outputs in the following format: (recipient, amount)")
                .add_optional_param("extra","Optional information for fees payment operation")
                .add_example("ledger schema name=gvt version=1.0 attr_names=name,age")
                .add_example("ledger schema name=gvt version=1.0 attr_names=name,age fees_inputs=pay:null:111_rBuQo2A1sc9jrJg fees_outputs=(pay:null:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100)")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let name = get_str_param("name", params).map_err(error_err!())?;
        let version = get_str_param("version", params).map_err(error_err!())?;
        let attr_names = get_str_array_param("attr_names", params).map_err(error_err!())?;
        let fees_inputs = get_opt_str_array_param("fees_inputs", params).map_err(error_err!())?;
        let fees_outputs = get_opt_str_tuple_array_param("fees_outputs", params).map_err(error_err!())?;
        let extra = get_opt_str_param("extra", params).map_err(error_err!())?;

        let id = build_schema_id(&submitter_did, name, version);

        let schema_data = {
            let mut json = JSONMap::new();
            json.insert("ver".to_string(), JSONValue::from("1.0"));
            json.insert("id".to_string(), JSONValue::from(id));
            json.insert("name".to_string(), JSONValue::from(name));
            json.insert("version".to_string(), JSONValue::from(version));
            json.insert("attrNames".to_string(), JSONValue::from(attr_names));
            JSONValue::from(json).to_string()
        };

        let mut request = Ledger::build_schema_request(&submitter_did, &schema_data)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let payment_method = set_request_fees(&mut request, wallet_handle, Some(&submitter_did), &fees_inputs, &fees_outputs, extra)?;

        let response_json = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request)
            .map_err(|err| handle_indy_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response_json)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Schema request has been sent to Ledger.",
                                                     Some("data"),
                                                     &[("name", "Name"),
                                                         ("version", "Version"),
                                                         ("attr_names", "Attributes")]))?;

        let receipts = parse_response_with_fees(&response_json, payment_method)?;

        let res = print_response_receipts(receipts);

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_validator_info_command {
    use super::*;

    command!(CommandMetadata::build("get-validator-info", "Get validator info from all nodes.")
                .add_optional_param("nodes","The list of node names to send the request")
                .add_optional_param("timeout"," Time to wait respond from nodes")
                .add_example(r#"ledger get-validator-info"#)
                .add_example(r#"ledger get-validator-info nodes=Node1,Node2"#)
                .add_example(r#"ledger get-validator-info nodes=Node1,Node2 timeout=150"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let nodes = get_opt_str_array_param("nodes", params).map_err(error_err!())?;
        let timeout = get_opt_number_param::<i32>("timeout", params).map_err(error_err!())?;

        let request = Ledger::build_get_validator_info_request(&submitter_did)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let response = if nodes.is_some() || timeout.is_some() {
            sign_and_submit_action(wallet_handle, pool_handle, &submitter_did, &request, nodes, timeout)
                .map_err(|err| handle_indy_error(err, None, None, None))?
        } else {
            Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request)
                .map_err(|err| handle_indy_error(err, None, None, None))?
        };

        let responses = match serde_json::from_str::<BTreeMap<String, String>>(&response) {
            Ok(responses) => responses,
            Err(_) => {
                let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
                    .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;
                return handle_transaction_response(response).map(|result| println_succ!("{}", result));
            }
        };

        println_succ!("Validator Info:");

        let mut lines: Vec<String> = Vec::new();

        for (node, response) in responses {
            if response.eq("timeout") {
                lines.push(format!("\t{:?}: {:?}", node, "Timeout"));
                continue
            }
            let response = match serde_json::from_str::<Response<serde_json::Value>>(&response) {
                Ok(resp) => resp,
                Err(err) => {
                    lines.push(format!("\t{:?}: \"Invalid data has been received: {:?}\"", node, err));
                    continue
                }
            };

            match handle_transaction_response(response) {
                Ok(result) => lines.push(format!("\t{:?}: {}", node, result)),
                Err(_) => {}
            };
        }

        println!("{{\n{}\n}}", lines.join(",\n"));

        let res = Ok(());

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_schema_command {
    use super::*;

    command!(CommandMetadata::build("get-schema", "Get Schema from Ledger.")
                .add_required_param("did", "DID of identity presented in Ledger")
                .add_required_param("name", "Schema name")
                .add_required_param("version", "Schema version")
                .add_example("ledger get-schema did=VsKV7grR1BUE29mG2Fm2kX name=gvt version=1.0")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let submitter_did = get_active_did(&ctx);

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let name = get_str_param("name", params).map_err(error_err!())?;
        let version = get_str_param("version", params).map_err(error_err!())?;

        let id = build_schema_id(target_did, name, version);

        let response = Ledger::build_get_schema_request(submitter_did.as_ref().map(String::as_str), &id)
            .and_then(|request| Ledger::submit_request(pool_handle, &request))
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        if let Some(result) = response.result.as_ref() {
            if !result["seqNo"].is_i64() {
                return Err(println_err!("Schema not found"));
            }
        };

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Following Schema has been received.",
                                                     Some("data"),
                                                     &[("name", "Name"),
                                                         ("version", "Version"),
                                                         ("attr_names", "Attributes")]));
        trace!("execute << {:?}", res);
        res
    }
}

pub mod cred_def_command {
    use super::*;

    command!(CommandMetadata::build("cred-def", "Send Cred Def transaction to the Ledger.")
                .add_required_param("schema_id", "Sequence number of schema")
                .add_required_param("signature_type", "Signature type (only CL supported now)")
                .add_optional_param("tag", "Allows to distinct between credential definitions for the same issuer and schema. Note that it is mandatory for indy-node version 1.4.x and higher")
                .add_required_param("primary", "Primary key in json format")
                .add_optional_param("revocation", "Revocation key in json format")
                .add_optional_param("fees_inputs","The list of source inputs")
                .add_optional_param("fees_outputs","The list of outputs in the following format: (recipient, amount)")
                .add_optional_param("extra","Optional information for fees payment operation")
                .add_example(r#"ledger cred-def schema_id=1 signature_type=CL tag=1 primary={"n":"1","s":"2","rms":"3","r":{"age":"4","name":"5"},"rctxt":"6","z":"7"}"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let schema_id = get_str_param("schema_id", params).map_err(error_err!())?;
        let signature_type = get_str_param("signature_type", params).map_err(error_err!())?;
        let tag = get_opt_str_param("tag", params).map_err(error_err!())?.unwrap_or("");
        let primary = get_object_param("primary", params).map_err(error_err!())?;
        let revocation = get_opt_str_param("revocation", params).map_err(error_err!())?;
        let fees_inputs = get_opt_str_array_param("fees_inputs", params).map_err(error_err!())?;
        let fees_outputs = get_opt_str_tuple_array_param("fees_outputs", params).map_err(error_err!())?;
        let extra = get_opt_str_param("extra", params).map_err(error_err!())?;

        let id = build_cred_def_id(&submitter_did, schema_id, signature_type, tag);

        let cred_def_value = {
            let mut json = JSONMap::new();
            json.insert("primary".to_string(), primary);
            update_json_map_opt_key!(json, "revocation", revocation);
            JSONValue::from(json)
        };

        let cred_def_data = {
            let mut json = JSONMap::new();
            json.insert("ver".to_string(), JSONValue::from("1.0"));
            json.insert("id".to_string(), JSONValue::from(id));
            json.insert("schemaId".to_string(), JSONValue::from(schema_id));
            json.insert("type".to_string(), JSONValue::from(signature_type));
            json.insert("tag".to_string(), JSONValue::from(tag));
            json.insert("value".to_string(), JSONValue::from(cred_def_value));
            JSONValue::from(json).to_string()
        };

        let mut request = Ledger::build_cred_def_request(&submitter_did, &cred_def_data)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let payment_method = set_request_fees(&mut request, wallet_handle, Some(&submitter_did), &fees_inputs, &fees_outputs, extra)?;

        let response_json = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request)
            .map_err(|err| handle_indy_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response_json)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "NodeConfig request has been sent to Ledger.",
                                                     Some("data"),
                                                     &[("primary", "Primary Key"),
                                                         ("revocation", "Revocation Key")]))?;

        let receipts = parse_response_with_fees(&response_json, payment_method)?;

        let res = print_response_receipts(receipts);

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_cred_def_command {
    use super::*;

    command!(CommandMetadata::build("get-cred-def", "Get Cred Definition from Ledger.")
                .add_required_param("schema_id", "Sequence number of schema")
                .add_required_param("signature_type", "Signature type (only CL supported now)")
                .add_optional_param("tag", "Allows to distinct between credential definitions for the same issuer and schema. Note that it is mandatory for indy-node version 1.4.x and higher")
                .add_required_param("origin", "Credential definition owner DID")
                .add_example("ledger get-cred-def schema_id=1 signature_type=CL tag=1 origin=VsKV7grR1BUE29mG2Fm2kX")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let submitter_did = get_active_did(&ctx);

        let schema_id = get_str_param("schema_id", params).map_err(error_err!())?;
        let signature_type = get_str_param("signature_type", params).map_err(error_err!())?;
        let tag = get_opt_str_param("tag", params).map_err(error_err!())?.unwrap_or("");
        let origin = get_str_param("origin", params).map_err(error_err!())?;

        let id = build_cred_def_id(&origin, schema_id, signature_type, tag);

        let response = Ledger::build_get_cred_def_request(submitter_did.as_ref().map(String::as_str), &id)
            .and_then(|request| Ledger::submit_request(pool_handle, &request))
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        if let Some(result) = response.result.as_ref() {
            if !result["seqNo"].is_i64() {
                return Err(println_err!("Credential Definition not found"));
            }
        };

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Following Credential Definition has been received.",
                                                     Some("data"),
                                                     &[("primary", "Primary Key"),
                                                         ("revocation", "Revocation Key")]));
        trace!("execute << {:?}", res);
        res
    }
}

pub mod node_command {
    use super::*;

    command!(CommandMetadata::build("node", "Send Node transaction to the Ledger.")
                .add_required_param("target", "Node identifier")
                .add_required_param("alias", "Node alias (can't be changed in case of update)")
                .add_optional_param("node_ip", "Node Ip. Note that it is mandatory for adding node case")
                .add_optional_param("node_port", "Node port. Note that it is mandatory for adding node case")
                .add_optional_param("client_ip", "Client Ip. Note that it is mandatory for adding node case")
                .add_optional_param("client_port","Client port. Note that it is mandatory for adding node case")
                .add_optional_param("blskey",  "Node BLS key")
                .add_optional_param("blskey_pop",  "Node BLS key proof of possession. Note that it is mandatory if blskey specified")
                .add_optional_param("services", "Node type. One of: VALIDATOR, OBSERVER or empty in case of blacklisting node")
                .add_example("ledger node target=A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y node_ip=127.0.0.1 node_port=9710 client_ip=127.0.0.1 client_port=9711 alias=Node5 services=VALIDATOR blskey=2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw blskey_pop=RPLagxaR5xdimFzwmzYnz4ZhWtYQEj8iR5ZU53T2gitPCyCHQneUn2Huc4oeLd2B2HzkGnjAff4hWTJT6C7qHYB1Mv2wU5iHHGFWkhnTX9WsEAbunJCV2qcaXScKj4tTfvdDKfLiVuU2av6hbsMztirRze7LvYBkRHV3tGwyCptsrP")
                .add_example("ledger node target=A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y node_ip=127.0.0.1 node_port=9710 client_ip=127.0.0.1 client_port=9711 alias=Node5 services=VALIDATOR")
                .add_example("ledger node target=A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y alias=Node5 services=VALIDATOR")
                .add_example("ledger node target=A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y alias=Node5 services=")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let target_did = get_str_param("target", params).map_err(error_err!())?;
        let node_ip = get_opt_str_param("node_ip", params).map_err(error_err!())?;
        let node_port = get_opt_number_param::<i32>("node_port", params).map_err(error_err!())?;
        let client_ip = get_opt_str_param("client_ip", params).map_err(error_err!())?;
        let client_port = get_opt_number_param::<i32>("client_port", params).map_err(error_err!())?;
        let alias = get_opt_str_param("alias", params).map_err(error_err!())?;
        let blskey = get_opt_str_param("blskey", params).map_err(error_err!())?;
        let blskey_pop = get_opt_str_param("blskey_pop", params).map_err(error_err!())?;
        let services = get_opt_str_array_param("services", params).map_err(error_err!())?;

        let node_data = {
            let mut json = JSONMap::new();
            update_json_map_opt_key!(json, "node_ip", node_ip);
            update_json_map_opt_key!(json, "node_port", node_port);
            update_json_map_opt_key!(json, "client_ip", client_ip);
            update_json_map_opt_key!(json, "client_port", client_port);
            update_json_map_opt_key!(json, "alias", alias);
            update_json_map_opt_key!(json, "blskey", blskey);
            update_json_map_opt_key!(json, "blskey_pop", blskey_pop);
            update_json_map_opt_key!(json, "services", services);
            JSONValue::from(json).to_string()
        };

        let response = Ledger::build_node_request(&submitter_did, target_did, &node_data)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request))
            .map_err(|err| handle_indy_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "NodeConfig request has been sent to Ledger.",
                                                     Some("data"),
                                                     &[("alias", "Alias"),
                                                         ("node_ip", "Node Ip"),
                                                         ("node_port", "Node Port"),
                                                         ("client_ip", "Client Ip"),
                                                         ("client_port", "Client Port"),
                                                         ("services", "Services"),
                                                         ("blskey", "Blskey"),
                                                         ("blskey_pop", "Blskey Proof of Possession")]));
        trace!("execute << {:?}", res);
        res
    }
}

pub mod pool_config_command {
    use super::*;

    command!(CommandMetadata::build("pool-config", "Send write configuration to pool.")
                .add_required_param("writes", "Accept write transactions.")
                .add_optional_param("force", "Forced configuration applying without reaching pool consensus.")
                .add_example("ledger pool-config writes=true")
                .add_example("ledger pool-config writes=true force=true")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let writes = get_bool_param("writes", params).map_err(error_err!())?;
        let force = get_opt_bool_param("force", params).map_err(error_err!())?.unwrap_or(false);

        let response = Ledger::indy_build_pool_config_request(&submitter_did, writes, force)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request))
            .map_err(|err| handle_indy_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "NodeConfig request has been sent to Ledger.",
                                                     None,
                                                     &[("writes", "Writes"),
                                                         ("force", "Force Apply")]));
        trace!("execute << {:?}", res);
        res
    }
}

pub mod pool_restart_command {
    use super::*;

    command!(CommandMetadata::build("pool-restart", "Send instructions to nodes to restart themselves.")
                .add_required_param("action", "Restart type. Either start or cancel.")
                .add_optional_param("nodes","The list of node names to send the request")
                .add_optional_param("timeout"," Time to wait respond from nodes")
                .add_optional_param("datetime", "Node restart datetime (only for action=start).")
                .add_example(r#"ledger pool-restart action=start datetime=2020-01-25T12:49:05.258870+00:00"#)
                .add_example(r#"ledger pool-restart action=start datetime=2020-01-25T12:49:05.258870+00:00 nodes=Node1,Node2"#)
                .add_example(r#"ledger pool-restart action=start datetime=2020-01-25T12:49:05.258870+00:00 nodes=Node1,Node2 timeout=100"#)
                .add_example(r#"ledger pool-restart action=cancel"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let action = get_str_param("action", params).map_err(error_err!())?;
        let datetime = get_opt_str_param("datetime", params).map_err(error_err!())?;
        let nodes = get_opt_str_array_param("nodes", params).map_err(error_err!())?;
        let timeout = get_opt_number_param::<i32>("timeout", params).map_err(error_err!())?;

        let request = Ledger::indy_build_pool_restart_request(&submitter_did, action, datetime)
            .map_err(|err| handle_indy_error(err, None, Some(&pool_name), Some(&wallet_name)))?;

        let response = if nodes.is_some() || timeout.is_some() {
            sign_and_submit_action(wallet_handle, pool_handle, &submitter_did, &request, nodes, timeout)
                .map_err(|err| handle_indy_error(err, None, None, None))?
        } else {
            Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request)
                .map_err(|err| handle_indy_error(err, None, None, None))?
        };

        let responses = match serde_json::from_str::<HashMap<String, String>>(&response) {
            Ok(responses) => responses,
            Err(_) => {
                let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
                    .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;
                return handle_transaction_response(response).map(|result| println_succ!("{}", result));
            }
        };

        for (node, response) in responses {
            if response.eq("timeout") {
                println_err!("Restart pool node {} timeout.", node);
                continue
            }

            let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
                .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

            println_succ!("Restart pool response for node {}:", node);
            let _res = handle_transaction_response(response).map(|result|
                print_table(&result, &[
                    ("identifier", "From"),
                    ("reqId", "Request Id"),
                    ("action", "Action"),
                    ("datetime", "Datetime")]));
        }
        let res = Ok(());

        trace!("execute << {:?}", res);
        res
    }
}

fn sign_and_submit_action(wallet_handle: i32, pool_handle: i32, submitter_did: &str, request: &str, nodes: Option<Vec<&str>>, timeout: Option<i32>) -> Result<String, IndyError> {
    let nodes = match nodes {
        Some(n) =>
            Some(serde_json::to_string(&n)
                .map_err(|err| IndyError { error_code: ErrorCode::CommonInvalidStructure, message: err.to_string(), indy_backtrace: None })?),
        None => None
    };

    Ledger::sign_request(wallet_handle, submitter_did, request)
        .and_then(|request| Ledger::submit_action(pool_handle, &request, nodes.as_ref().map(String::as_ref), timeout))
}

pub mod pool_upgrade_command {
    use super::*;

    command!(CommandMetadata::build("pool-upgrade", "Send instructions to nodes to update themselves.")
                .add_required_param("name", "Human-readable name for the upgrade.")
                .add_required_param("version","The version of indy-node package we perform upgrade to. \n                  \
                                              Must be greater than existing one (or equal if reinstall flag is True)")
                .add_required_param("action", "Upgrade type. Either start or cancel.")
                .add_required_param("sha256", "Sha256 hash of the package.")
                .add_optional_param("timeout", "Limits upgrade time on each Node.")
                .add_optional_param("schedule", "Node upgrade schedule. Schedule should contain identifiers of all nodes. Upgrade dates should be in future. \n                              \
                                              If force flag is False, then it's required that time difference between each Upgrade must be not less than 5 minutes.\n                              \
                                              Requirements for schedule can be ignored by parameter force=true.\n                              \
                                              Schedule is mandatory for action=start.")
                .add_optional_param("justification", "Justification string for this particular Upgrade.")
                .add_optional_param("reinstall", "Whether it's allowed to re-install the same version. False by default.")
                .add_optional_param("force", "Whether we should apply transaction without waiting for consensus of this transaction. False by default.")
                .add_optional_param("package", "Package to be upgraded.")
                .add_example(r#"ledger pool-upgrade name=upgrade-1 version=2.0 action=start sha256=f284bdc3c1c9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398 schedule={"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv":"2020-01-25T12:49:05.258870+00:00"}"#)
                .add_example(r#"ledger pool-upgrade name=upgrade-1 version=2.0 action=start sha256=f284bdc3c1c9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398 schedule={"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv":"2020-01-25T12:49:05.258870+00:00"} package=some_package"#)
                .add_example(r#"ledger pool-upgrade name=upgrade-1 version=2.0 action=cancel sha256=ac3eb2cc3ac9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let name = get_str_param("name", params).map_err(error_err!())?;
        let version = get_str_param("version", params).map_err(error_err!())?;
        let action = get_str_param("action", params).map_err(error_err!())?;
        let sha256 = get_str_param("sha256", params).map_err(error_err!())?;
        let timeout = get_opt_number_param::<u32>("timeout", params).map_err(error_err!())?;
        let schedule = get_opt_str_param("schedule", params).map_err(error_err!())?;
        let justification = get_opt_str_param("justification", params).map_err(error_err!())?;
        let reinstall = get_opt_bool_param("reinstall", params).map_err(error_err!())?.unwrap_or(false);
        let force = get_opt_bool_param("force", params).map_err(error_err!())?.unwrap_or(false);
        let package = get_opt_str_param("package", params).map_err(error_err!())?;

        let response = Ledger::indy_build_pool_upgrade_request(&submitter_did, name, version, action, sha256,
                                                               timeout, schedule, justification, reinstall, force, package)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request))
            .map_err(|err| handle_indy_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        let mut schedule = None;
        let mut hash = None;
        if let Some(res) = response.result.as_ref() {
            schedule = res["schedule"].as_object()
                .map(|s| format!("{{{}\n}}",
                                 s.iter()
                                     .map(|(key, value)| format!("\n    {:?}:{:?}", key, value.as_str().unwrap_or("")))
                                     .collect::<Vec<String>>()
                                     .join(",")));

            hash = res["sha256"].as_str().map(|h| h.to_string());
        };

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "NodeConfig request has been sent to Ledger.",
                                                     None,
                                                     &[("name", "Name"),
                                                         ("action", "Action"),
                                                         ("version", "Version"),
                                                         ("timeout", "Timeout"),
                                                         ("justification", "Justification"),
                                                         ("reinstall", "Reinstall"),
                                                         ("force", "Force Apply"),
                                                         ("package", "Package Name")]));
        if let Some(h) = hash {
            println_succ!("Hash:");
            println!("{}", h);
        }
        if let Some(s) = schedule {
            println_succ!("Schedule:");
            println!("{}", s);
        }
        trace!("execute << {:?}", res);
        res
    }
}

pub mod custom_command {
    use super::*;

    command!(CommandMetadata::build("custom", "Send custom transaction to the Ledger.")
                .add_main_param("txn", "Transaction json")
                .add_optional_param("sign", "Is signature required")
                .add_example(r#"ledger custom {"reqId":1,"identifier":"V4SGRU86Z58d6TV7PBUe6f","operation":{"type":"105","dest":"V4SGRU86Z58d6TV7PBUe6f"},"protocolVersion":2}"#)
                .add_example(r#"ledger custom {"reqId":2,"identifier":"V4SGRU86Z58d6TV7PBUe6f","operation":{"type":"1","dest":"VsKV7grR1BUE29mG2Fm2kX"},"protocolVersion":2} sign=true"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;

        let txn = get_str_param("txn", params).map_err(error_err!())?;
        let sign = get_opt_bool_param("sign", params).map_err(error_err!())?.unwrap_or(false);

        let (mut submitter, mut wallet) = (String::new(), String::new());

        let response = if sign {
            let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
            let submitter_did = ensure_active_did(&ctx)?;

            submitter = submitter_did.clone();
            wallet = wallet_name.clone();

            Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, txn)
        } else {
            Ledger::submit_request(pool_handle, txn)
        };

        let response_json =
            response.map_err(|err| handle_indy_error(err, Some(&submitter), Some(&pool_name), Some(&wallet)))?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response_json)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        let res = match response {
            Response { op: ResponseType::REPLY, result: Some(_), reason: None } =>
                Ok(println!("Response: \n{}", response_json)),
            Response { op: ResponseType::REQNACK, result: None, reason: Some(reason) } |
            Response { op: ResponseType::REJECT, result: None, reason: Some(reason) } =>
                Err(println_err!("Transaction has been rejected: {}", extract_error_message(&reason))),
            _ => Err(println_err!("Invalid data has been received"))
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_payment_sources_command {
    use super::*;

    command!(CommandMetadata::build("get-payment-sources", "Get sources list for payment address.")
                .add_required_param("payment_address","Target payment address")
                .add_example("ledger get-payment-sources payment_address=pay:null:GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = get_active_did(&ctx);

        let payment_address = get_str_param("payment_address", params).map_err(error_err!())?;

        let (request, payment_method) = Payment::build_get_payment_sources_request(wallet_handle, submitter_did.as_ref().map(String::as_str), payment_address)
            .map_err(|err| handle_payment_error(err, None))?;

        let response = Ledger::submit_request(pool_handle, &request)
            .map_err(|err| handle_indy_error(err, None, Some(&pool_name), Some(&wallet_name)))?;

        let res = match Payment::parse_get_payment_sources_response(&payment_method, &response) {
            Ok(sources_json) => {
                let mut sources: Vec<serde_json::Value> = serde_json::from_str(&sources_json)
                    .map_err(|_| println_err!("Wrong data has been received"))?;

                print_list_table(&sources,
                                 &vec![("source", "Source"),
                                       ("paymentAddress", "Payment Address"),
                                       ("amount", "Amount"),
                                       ("extra", "Extra")],
                                 "There are no source's");
                Ok(())
            }
            Err(err) => Err(println_err!("Invalid data has been received: {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod payment_command {
    use super::*;

    command!(CommandMetadata::build("payment", "Send request for doing payment.")
                .add_required_param("inputs","The list of payment sources")
                .add_required_param("outputs","The list of outputs in the following format: (recipient, amount)")
                .add_required_param("extra","Optional information for payment operation")
                .add_example("ledger payment inputs=pay:null:111_rBuQo2A1sc9jrJg outputs=(pay:null:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100)")
                .add_example("ledger payment inputs=pay:null:111_rBuQo2A1sc9jrJg outputs=(pay:null:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100) extra=some_extra")
                .add_example("ledger payment inputs=pay:null:111_rBuQo2A1sc9jrJg,pay:null:222_aEwACvA1sc9jrJg outputs=(pay:null:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100),(pay:null:ABABefwrhscbaAShva7dkx1d2dZ3zUF8ckg7wmL7ofN4,5)")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = get_active_did(&ctx);
        let extra = get_opt_str_param("extra", params).map_err(error_err!())?;

        let inputs = get_str_array_param("inputs", params).map_err(error_err!())?;
        let outputs = get_str_tuple_array_param("outputs", params).map_err(error_err!())?;

        let inputs = parse_payment_inputs(&inputs).map_err(error_err!())?;
        let outputs = parse_payment_outputs(&outputs).map_err(error_err!())?;

        let (request, payment_method) = Payment::build_payment_req(wallet_handle, submitter_did.as_ref().map(String::as_str), &inputs, &outputs, extra)
            .map_err(|err| handle_payment_error(err, None))?;

        let response = Ledger::submit_request(pool_handle, &request)
            .map_err(|err| handle_indy_error(err, None, Some(&pool_name), Some(&wallet_name)))?;

        let res = match Payment::parse_payment_response(&payment_method, &response) {
            Ok(receipts_json) => {
                let mut receipts: Vec<serde_json::Value> = serde_json::from_str(&receipts_json)
                    .map_err(|_| println_err!("Wrong data has been received"))?;

                print_list_table(&receipts,
                                 &vec![("receipt", "Receipt"),
                                       ("recipient", "Recipient Payment Address"),
                                       ("amount", "Amount"),
                                       ("extra", "Extra")],
                                 "There are no receipts's");
                Ok(())
            }
            Err(err) => Err(handle_payment_error(err, None)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_fees_command {
    use super::*;

    command!(CommandMetadata::build("get-fees", "Get fees amount for transactions.")
                .add_required_param("payment_method","Payment method")
                .add_example("ledger get-fees payment_method=null")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = get_active_did(&ctx);

        let payment_method = get_str_param("payment_method", params).map_err(error_err!())?;

        let request = Payment::build_get_txn_fees_req(wallet_handle, submitter_did.as_ref().map(String::as_str), payment_method)
            .map_err(|err| handle_payment_error(err, Some(payment_method)))?;

        let response = Ledger::submit_request(pool_handle, &request)
            .map_err(|err| handle_indy_error(err, None, Some(&pool_name), Some(&wallet_name)))?;

        let res = match Payment::parse_get_txn_fees_response(&payment_method, &response) {
            Ok(fees_json) => {
                let mut fees: HashMap<String, i32> = serde_json::from_str(&fees_json)
                    .map_err(|_| println_err!("Wrong data has been received"))?;

                let mut fees =
                    fees
                        .iter()
                        .map(|(key, value)|
                            json!({
                            "type": key,
                            "amount": value
                        }))
                        .collect::<Vec<serde_json::Value>>();

                print_list_table(&fees,
                                 &vec![("type", "Transaction"),
                                       ("amount", "Amount")],
                                 "There are no fees");

                Ok(())
            }
            Err(err) => Err(handle_payment_error(err, None)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod mint_prepare_command {
    use super::*;

    command!(CommandMetadata::build("mint-prepare", "Prepare MINT transaction.")
                .add_required_param("outputs","The list of outputs in the following format: (recipient, amount)")
                .add_required_param("extra","Optional information for mint operation")
                .add_example("ledger mint-prepare outputs=(pay:null:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100)")
                .add_example("ledger mint-prepare outputs=(pay:null:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100) extra=some_data")
                .add_example("ledger mint-prepare outputs=(pay:null:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100),(pay:null:ABABaaVwSascbaAShva7dkx1d2dZ3zUF8ckg7wmL7ofN4,5)")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(ctx)?;
        let submitter_did = get_active_did(&ctx);

        let outputs = get_str_tuple_array_param("outputs", params).map_err(error_err!())?;
        let outputs = parse_payment_outputs(&outputs).map_err(error_err!())?;

        let extra = get_opt_str_param("extra", params).map_err(error_err!())?;

        Payment::build_mint_req(wallet_handle, submitter_did.as_ref().map(String::as_str), &outputs, extra)
            .map(|(request, _payment_method)| {
                println_succ!("MINT transaction has been created:");
                println!("     {}", request);
            })
            .map_err(|err| handle_payment_error(err, None))?;

        let res = Ok(());
        trace!("execute << {:?}", res);
        res
    }
}

pub mod set_fees_prepare_command {
    use super::*;

    command!(CommandMetadata::build("set-fees-prepare", " Prepare SET_FEES transaction.")
                .add_required_param("payment_method","Payment method to use")
                .add_required_param("fees","The list of transactions fees")
                .add_example("ledger set-fees-prepare payment_method=null fees=1:100,100:200")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(ctx)?;
        let submitter_did = get_active_did(&ctx);

        let payment_method = get_str_param("payment_method", params).map_err(error_err!())?;
        let fees = get_str_array_param("fees", params).map_err(error_err!())?;

        let fees = parse_payment_fees(&fees).map_err(error_err!())?;

        Payment::build_set_txn_fees_req(wallet_handle, submitter_did.as_ref().map(String::as_str), &payment_method, &fees)
            .map(|request| {
                println_succ!("SET_FEES transaction has been created:");
                println!("     {}", request);
            })
            .map_err(|err| handle_payment_error(err, Some(payment_method)))?;

        let res = Ok(());
        trace!("execute << {:?}", res);
        res
    }
}

pub mod verify_payment_receipt_command {
    use super::*;

    command!(CommandMetadata::build("verify-payment-receipt", "Get payment receipt verification info.")
                .add_main_param("receipt","Receipt to verify")
                .add_example("ledger verify-payment-receipt pay:null:0_PqVjwJC42sxCTJp")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = get_active_did(&ctx);

        let receipt = get_str_param("receipt", params).map_err(error_err!())?;

        let (request, payment_method) = Payment::build_verify_payment_req(wallet_handle, submitter_did.as_ref().map(String::as_str), receipt)
            .map_err(|err| handle_payment_error(err, None))?;

        let response = Ledger::submit_request(pool_handle, &request)
            .map_err(|err| handle_indy_error(err, None, Some(&pool_name), Some(&wallet_name)))?;

        let res = match Payment::parse_verify_payment_response(&payment_method, &response) {
            Ok(info_json) => {
                println_succ!("Following Payment Receipt Verification Info has been received.");
                println!("{}", info_json);
                Ok(())
            }
            Err(err) => Err(handle_payment_error(err, None)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod sign_multi_command {
    use super::*;

    command!(CommandMetadata::build("sign-multi", "Add multi signature by current DID to transaction.")
                .add_required_param("txn","Transaction to sign")
                .add_example(r#"ledger sign-multi txn={"reqId":123456789,"type":"100"}"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (wallet_handle, _) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let txn = get_str_param("txn", params).map_err(error_err!())?;

        let res = match Ledger::multi_sign_request(wallet_handle, &submitter_did, txn) {
            Ok(request) => {
                println_succ!("Transaction has been signed:");
                println_succ!("{}", request);
                Ok(())
            }
            Err(err) => {
                match err.error_code {
                    ErrorCode::WalletItemNotFound => Err(println_err!("Signer DID: \"{}\" not found", submitter_did)),
                    _ => Err(handle_indy_error(err, Some(&submitter_did), None, None)),
                }
            }
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub fn set_request_fees(request: &mut String, wallet_handle: i32, submitter_did: Option<&str>, fees_inputs: &Option<Vec<&str>>, fees_outputs: &Option<Vec<String>>, extra: Option<&str>) -> Result<Option<String>, ()> {
    let mut payment_method: Option<String> = None;
    if let &Some(ref inputs) = fees_inputs {
        let inputs_json = parse_payment_inputs(&inputs)?;

        let outputs_json = if let &Some(ref o) = fees_outputs {
            parse_payment_outputs(&o)?
        } else { "[]".to_string() };

        *request = Payment::add_request_fees(wallet_handle, submitter_did, request, &inputs_json, &outputs_json, extra)
            .map(|(request, _)| request)
            .map_err(|err| handle_payment_error(err, None))?;

        payment_method = parse_method_from_payment_address(inputs[0])
    }
    Ok(payment_method)
}

fn parse_method_from_payment_address(input: &str) -> Option<String> {
    let res: Vec<&str> = input.split(':').collect();
    match res.len() {
        3 => res.get(1).map(|s| s.to_string()),
        _ => None
    }
}

fn parse_payment_inputs(inputs: &Vec<&str>) -> Result<String, ()> {
    serde_json::to_string(&inputs)
        .map_err(|_| println_err!("Wrong data has been received"))
}

fn parse_payment_outputs(outputs: &Vec<String>) -> Result<String, ()> {
    const OUTPUTS_DELIMITER: &'static str = ",";

    if outputs.is_empty() {
        return Err(println_err!("Outputs list is empty"));
    }

    let mut output_objects: Vec<serde_json::Value> = Vec::new();
    for output in outputs {
        let parts: Vec<&str> = output.split(OUTPUTS_DELIMITER).collect::<Vec<&str>>();

        output_objects.push(json!({
                        "recipient": parts.get(0)
                                          .ok_or(())
                                          .map_err(|_| println_err!("Invalid format of Outputs: Payment Address not found"))?,
                        "amount": parts.get(1)
                                    .ok_or(())
                                    .map_err(|_| println_err!("Invalid format of Outputs: Amount not found"))
                                    .and_then(|amount| amount.parse::<u64>()
                                        .map_err(|_| println_err!("Invalid format of Outputs: Amount must be integer and greater then 0")))?
                    }));
    }

    serde_json::to_string(&output_objects)
        .map_err(|_| println_err!("Wrong data has been received"))
}


pub fn parse_response_with_fees(response: &str, payment_method: Option<String>) -> Result<Option<Vec<serde_json::Value>>, ()> {
    let receipts = if let Some(method) = payment_method {
        Some(Payment::parse_response_with_fees(&method, &response)
            .map_err(|err| handle_payment_error(err, Some(&method)))
            .and_then(|fees| serde_json::from_str::<Vec<serde_json::Value>>(&fees)
                .map_err(|err| println_err!("Invalid data has been received: {:?}", err)))?)
    } else { None };

    Ok(receipts)
}

pub fn print_response_receipts(receipts: Option<Vec<serde_json::Value>>) -> Result<(), ()> {
    receipts.map(|receipt| {
        if !receipt.is_empty() {
            println_succ!("Following Receipts has been received.");
            print_list_table(&receipt,
                             &vec![("receipt", "Receipt"),
                                   ("recipient", "Payment Address of recipient"),
                                   ("amount", "Amount"),
                                   ("extra", "Extra")],
                             "");
        }
    });
    Ok(())
}

fn parse_payment_fees(fees: &Vec<&str>) -> Result<String, ()> {
    let mut fees_map: HashMap<String, u64> = HashMap::new();

    for fee in fees {
        let parts = fee.split(":").collect::<Vec<&str>>();

        let type_ = parts.get(0)
            .ok_or(())
            .map_err(|_| println_err!("Invalid format of Fees: Type not found"))?
            .to_string();

        let amount = parts.get(1)
            .ok_or(())
            .map_err(|_| println_err!("Invalid format of Fees: Amount not found"))
            .and_then(|amount| amount.parse::<u64>()
                .map_err(|_| println_err!("Invalid format of Fees: Amount must greater or equal zero")))?;

        fees_map.insert(type_, amount);
    }

    serde_json::to_string(&fees_map)
        .map_err(|_| println_err!("Wrong data has been received"))
}

fn print_transaction_response(mut result: serde_json::Value, title: &str,
                              data_sub_field: Option<&str>,
                              data_headers: &[(&str, &str)]) {
    println_succ!("{}", title);

    let (metadata_headers, metadata, data) = match result["ver"].clone().as_str() {
        None => parse_transaction_response_v0(&mut result),
        Some("1") => parse_transaction_response_v1(&mut result),
        ver @ _ => return println_err!("Unsupported transaction response format: {:?}", ver)
    };

    println_succ!("Metadata:");
    print_table(&metadata, &metadata_headers);

    let data = if data_sub_field.is_some() { &data[data_sub_field.unwrap()] } else { &data };
    let mut data_headers = data_headers.to_vec();
    data_headers.retain(|&(ref key, _)| !data[key].is_null());

    println_succ!("Data:");
    print_table(data, &data_headers);
}

fn parse_transaction_response_v0(result: &mut serde_json::Value) -> ([(&'static str, &'static str); 4], serde_json::Value, serde_json::Value) {
    if let Some(txn_time) = result["txnTime"].as_i64() {
        result["txnTime"] = serde_json::Value::String(timestamp_to_datetime(txn_time))
    }

    let metadata_headers = [
        ("identifier", "Identifier"),
        ("seqNo", "Sequence Number"),
        ("reqId", "Request ID"),
        ("txnTime", "Transaction time")];

    (metadata_headers, result.clone(), result.clone())
}

fn parse_transaction_response_v1(result: &mut serde_json::Value) -> ([(&'static str, &'static str); 4], serde_json::Value, serde_json::Value) {
    if let Some(txn_time) = result["txnMetadata"]["txnTime"].as_i64() {
        result["txnMetadata"]["txnTime"] = serde_json::Value::String(timestamp_to_datetime(txn_time))
    }

    let metadata_headers = [
        ("from", "From"),
        ("seqNo", "Sequence Number"),
        ("reqId", "Request ID"),
        ("txnTime", "Transaction time")];

    let mut metadata_obj = result["txnMetadata"].as_object().unwrap().clone();

    metadata_obj.insert("reqId".to_string(), result["txn"]["metadata"]["reqId"].clone());
    metadata_obj.insert("from".to_string(), result["txn"]["metadata"]["from"].clone());

    let metadata = serde_json::Value::Object(metadata_obj);
    let data = result["txn"]["data"].clone();

    (metadata_headers, metadata, data)
}

pub fn handle_transaction_response(response: Response<serde_json::Value>) -> Result<serde_json::Value, ()> {
    match response {
        Response { op: ResponseType::REPLY, result: Some(result), reason: None } => Ok(result),
        Response { op: ResponseType::REQNACK, result: None, reason: Some(reason) } |
        Response { op: ResponseType::REJECT, result: None, reason: Some(reason) } =>
            Err(println_err!("Transaction has been rejected: {}", extract_error_message(&reason))),
        _ => Err(println_err!("Invalid data has been received"))
    }
}

fn extract_error_message(error: &str) -> String {
    let re = Regex::new(r#"\(["'](.*)["'],\)"#).unwrap();
    match re.captures(error) {
        Some(message) => message[1].to_string(),
        None => error.to_string()
    }
}

fn get_role_title(role: &serde_json::Value) -> serde_json::Value {
    serde_json::Value::String(match role.as_str() {
        Some("0") => "TRUSTEE",
        Some("2") => "STEWARD",
        Some("101") => "TRUST_ANCHOR",
        Some("201") => "NETWORK_MONITOR",
        _ => "-"
    }.to_string())
}

fn timestamp_to_datetime(_time: i64) -> String {
    NaiveDateTime::from_timestamp(_time, 0).to_string()
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
pub enum ResponseType {
    REQNACK,
    REPLY,
    REJECT
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub op: ResponseType,
    pub reason: Option<String>,
    pub result: Option<T>,
}

#[derive(Deserialize, Debug)]
pub struct ReplyResult<T> {
    pub data: T,
    #[serde(rename = "seqNo")]
    pub seq_no: u64,
    pub identifier: String
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use commands::wallet::tests::{create_and_open_wallet, close_and_delete_wallet, open_wallet, close_wallet};
    use commands::pool::tests::{disconnect_and_delete_pool};
    use commands::did::tests::{new_did, use_did, SEED_TRUSTEE, DID_TRUSTEE, DID_MY1, VERKEY_MY1, SEED_MY3, DID_MY3, VERKEY_MY3};
    #[cfg(feature = "nullpay_plugin")]
    use commands::common::tests::{load_null_payment_plugin, NULL_PAYMENT_METHOD};
    #[cfg(feature = "nullpay_plugin")]
    use commands::payment_address::tests::create_payment_address;
    use libindy::ledger::Ledger;
    use libindy::did::Did;

    pub const ATTRIB_RAW_DATA: &'static str = r#"{"endpoint":{"ha":"127.0.0.1:5555"}}"#;
    pub const ATTRIB_HASH_DATA: &'static str = r#"83d907821df1c87db829e96569a11f6fc2e7880acba5e43d07ab786959e13bd3"#;
    pub const ATTRIB_ENC_DATA: &'static str = r#"aa3f41f619aa7e5e6b6d0d"#;

    pub const CRED_DEF_DATA: &'static str = r#"{"n":"1","s":"1","rms":"1","r":{"age":"1","name":"1"},"rctxt":"1","z":"1"}"#;

    #[cfg(feature = "nullpay_plugin")]
    pub const UNKNOWN_PAYMENT_METHOD: &'static str = "UNKNOWN_PAYMENT_METHOD";
    #[cfg(feature = "nullpay_plugin")]
    pub const PAYMENT_ADDRESS: &'static str = "pay:null:BBQr7K6CP1tslXd";
    #[cfg(feature = "nullpay_plugin")]
    pub const INVALID_PAYMENT_ADDRESS: &'static str = "null";
    #[cfg(feature = "nullpay_plugin")]
    pub const INPUT: &'static str = "pay:null:111_rBuQo2A1sc9jrJg";
    #[cfg(feature = "nullpay_plugin")]
    pub const OUTPUT: &'static str = "(pay:null:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,10)";
    #[cfg(feature = "nullpay_plugin")]
    pub const OUTPUT_2: &'static str = "(pay:null:GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa,25)";
    #[cfg(feature = "nullpay_plugin")]
    pub const INVALID_INPUT: &'static str = "pay:null";
    #[cfg(feature = "nullpay_plugin")]
    pub const INVALID_OUTPUT: &'static str = "pay:null:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,100";
    #[cfg(feature = "nullpay_plugin")]
    pub const FEES: &'static str = "1:1,100:1,101:1";
    #[cfg(feature = "nullpay_plugin")]
    pub const EXTRA: &'static str = "extra";
    #[cfg(feature = "nullpay_plugin")]
    pub const AMOUNT: i32 = 100;

    mod nym {
        use super::*;

        #[test]
        pub fn nym_works() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            let (did, verkey) = create_new_did(&ctx);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("verkey", verkey);
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_nym_added(&ctx, &did);
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn nym_works_for_role() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            let (did, verkey) = create_new_did(&ctx);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("verkey", verkey);
                params.insert("role", "TRUSTEE".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_nym_added(&ctx, &did);
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn nym_works_for_set_fees() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            set_fees(&ctx, FEES);
            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);

            let (did, verkey) = create_new_did(&ctx);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("verkey", verkey);
                params.insert("fees_inputs", input);
                params.insert("fees_outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_nym_added(&ctx, &did);
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn nym_works_for_set_fees_with_input_amount_lower_fee() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            set_fees(&ctx, "1:101");

            let (did, verkey) = create_new_did(&ctx);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("verkey", verkey);
                params.insert("fees_inputs", input);
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }


        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn nym_works_for_set_fees_with_input_amount_lower_fee_plus_output() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            set_fees(&ctx, "1:95");

            let (did, verkey) = create_new_did(&ctx);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("verkey", verkey);
                params.insert("fees_inputs", input);
                params.insert("fees_outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn nym_works_for_wrong_role() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);

            let (did, verkey) = create_new_did(&ctx);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("verkey", verkey);
                params.insert("role", "ROLE".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn nym_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool();
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn nym_works_for_no_opened_wallet() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);

            close_and_delete_wallet(&ctx);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            disconnect_and_delete_pool(&ctx);
            tear_down();
        }

        #[test]
        pub fn nym_works_for_no_connected_pool() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);

            disconnect_and_delete_pool(&ctx);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            tear_down();
        }

        #[test]
        pub fn nym_works_for_unknown_submitter() {
            let ctx = setup_with_wallet_and_pool();

            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY3.to_string());
                params.insert("verkey", VERKEY_MY3.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod get_nym {
        use super::*;

        #[test]
        pub fn get_nym_works() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = get_nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_nym_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool();
            {
                let cmd = get_nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_nym_works_for_unknown_did() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = get_nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY3.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod attrib {
        use super::*;

        #[test]
        pub fn attrib_works_for_raw_value() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, &did, Some(ATTRIB_RAW_DATA), None, None);
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn attrib_works_for_hash_value() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("hash", ATTRIB_HASH_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, &did, None, Some(ATTRIB_HASH_DATA), None);
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn attrib_works_for_enc_value() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("enc", ATTRIB_ENC_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, &did, None, None, Some(ATTRIB_ENC_DATA));
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn attrib_works_for_set_fees() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();

            let (did, _) = use_new_identity(&ctx);
            use_did(&ctx, DID_TRUSTEE);
            set_fees(&ctx, FEES);
            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            use_did(&ctx, &did);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                params.insert("fees_inputs", input);
                params.insert("fees_outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, &did, Some(ATTRIB_RAW_DATA), None, None);
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn attrib_works_for_set_fees_input_amount_lower_fee() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();

            let (did, _) = use_new_identity(&ctx);

            use_did(&ctx, DID_TRUSTEE);
            set_fees(&ctx, "ATTRIB:101");
            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            use_did(&ctx, &did);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.to_string());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                params.insert("fees_inputs", input);
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn attrib_works_for_missed_attribute() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn attrib_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool();
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn attrib_works_for_unknown_did() {
            let ctx = setup_with_wallet_and_pool();

            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY3.to_string());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn attrib_works_for_invalid_endpoint_format() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                params.insert("raw", r#"127.0.0.1:5555"#.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod get_attrib {
        use super::*;

        #[test]
        pub fn get_attrib_works_for_raw_value() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, &did, Some(ATTRIB_RAW_DATA), None, None);
            {
                let cmd = get_attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("raw", "endpoint".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_attrib_works_for_hash_value() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("hash", ATTRIB_HASH_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, &did, None, Some(ATTRIB_HASH_DATA), None);
            {
                let cmd = get_attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("hash", ATTRIB_HASH_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_attrib_works_for_enc_value() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("enc", ATTRIB_ENC_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, &did, None, None, Some(ATTRIB_ENC_DATA));
            {
                let cmd = get_attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("enc", ATTRIB_ENC_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_attrib_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, &did, Some(ATTRIB_RAW_DATA), None, None);

            // to reset active did
            close_wallet(&ctx);
            open_wallet(&ctx);

            {
                let cmd = get_attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.clone());
                params.insert("raw", "endpoint".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod schema {
        use super::*;

        #[test]
        pub fn schema_works() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                params.insert("attr_names", "name,age".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_schema_added(&ctx, &did);
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn schema_works_for_set_fees() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            let (did, _) = use_new_identity(&ctx);

            set_fees(&ctx, FEES);
            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                params.insert("attr_names", "name,age".to_string());
                params.insert("fees_inputs", input);
                params.insert("fees_outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_schema_added(&ctx, &did);
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn schema_works_for_set_fees_input_amount_lower_fee() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_new_identity(&ctx);
            set_fees(&ctx, "SCHEMA:101");

            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                params.insert("attr_names", "name,age".to_string());
                params.insert("fees_inputs", input);
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn schema_works_for_missed_required_params() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn schema_works_unknown_submitter() {
            let ctx = setup_with_wallet_and_pool();
            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                params.insert("attr_names", "name,age".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn schema_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool();
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                params.insert("attr_names", "name,age".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod get_validator_info {
        use super::*;

        #[test]
        pub fn get_validator_info_works() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = get_validator_info_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_validator_info_works_for_nodes() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = get_validator_info_command::new();
                let mut params = CommandParams::new();
                params.insert("nodes", "Node1,Node2".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_validator_info_works_for_unknown_node() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = get_validator_info_command::new();
                let mut params = CommandParams::new();
                params.insert("nodes", "Unknown Node".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_validator_info_works_for_timeout() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = get_validator_info_command::new();
                let mut params = CommandParams::new();
                params.insert("nodes", "Node1,Node2".to_string());
                params.insert("timeout", "10".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod get_schema {
        use super::*;

        #[test]
        pub fn get_schema_works() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                params.insert("attr_names", "name,age".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_schema_added(&ctx, &did);
            {
                let cmd = get_schema_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did);
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn schema_works_for_unknown_schema() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = get_schema_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                params.insert("name", "unknown_schema_name".to_string());
                params.insert("version", "1.0".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test] // TODO: CHECK
        pub fn schema_works_for_unknown_submitter() {
            let ctx = setup_with_wallet_and_pool();
            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
            {
                let cmd = get_schema_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY3.to_string());
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn schema_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                params.insert("attr_names", "name,age".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_schema_added(&ctx, &did);

            // to reset active did
            close_wallet(&ctx);
            open_wallet(&ctx);

            {
                let cmd = get_schema_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did);
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod cred_def {
        use super::*;

        #[test]
        pub fn cred_def_works() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            let schema_id = send_schema(&ctx, &did);
            {
                let cmd = cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", schema_id.clone());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("primary", CRED_DEF_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_cred_def_added(&ctx, &did, &schema_id);
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn cred_def_works_for_set_fees() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            let (did, _) = use_new_identity(&ctx);
            let schema_id = send_schema(&ctx, &did);

            set_fees(&ctx, FEES);
            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            {
                let cmd = cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", schema_id.clone());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("primary", CRED_DEF_DATA.to_string());
                params.insert("fees_inputs", input);
                params.insert("fees_outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_cred_def_added(&ctx, &did, &schema_id);
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn cred_def_works_for_set_fees_input_amount_lower_fee() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            let (did, _) = use_new_identity(&ctx);
            let schema_id = send_schema(&ctx, &did);

            set_fees(&ctx, "CRED_DEF:101");
            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            {
                let cmd = cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", schema_id.clone());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("primary", CRED_DEF_DATA.to_string());
                params.insert("fees_inputs", input);
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn cred_def_works_for_missed_required_params() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", "1".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn cred_def_works_for_unknown_submitter() {
            let ctx = setup_with_wallet_and_pool();
            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
            {
                let cmd = cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", "1".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("primary", CRED_DEF_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn cred_def_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool();
            {
                let cmd = cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", "1".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("primary", CRED_DEF_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod get_cred_def {
        use super::*;

        #[test]
        pub fn get_cred_def_works() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            let schema_id = send_schema(&ctx, &did);
            {
                let cmd = cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", schema_id.clone());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("primary", CRED_DEF_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_cred_def_added(&ctx, &did, &schema_id);
            {
                let cmd = get_cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", schema_id);
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("origin", did.clone());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_cred_def_works_for_unknown_cred_def() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = get_cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", "2".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("origin", DID_MY3.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_cred_def_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool();
            let (did, _) = use_new_identity(&ctx);
            let schema_id = send_schema(&ctx, &did);
            {
                let cmd = cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", schema_id.clone());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("primary", CRED_DEF_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_cred_def_added(&ctx, &did, &schema_id);

            // to reset active did
            close_wallet(&ctx);
            open_wallet(&ctx);

            {
                let cmd = get_cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", schema_id);
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("origin", did.clone());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod node {
        use super::*;

        #[test]
        #[ignore] //TODO: FIXME currently unstable pool behaviour after new non-existing node was added
        pub fn node_works() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            let (_did, my_verkey) = create_new_did(&ctx);
            send_nym(&ctx, &_did, &my_verkey, Some("STEWARD"));
            use_did(&ctx, &_did);
            {
                let cmd = node_command::new();
                let mut params = CommandParams::new();
                params.insert("target", "A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y".to_string());
                params.insert("node_ip", "127.0.0.1".to_string());
                params.insert("node_port", "9710".to_string());
                params.insert("client_ip", "127.0.0.2".to_string());
                params.insert("client_port", "9711".to_string());
                params.insert("alias", "Node5".to_string());
                params.insert("blskey", "2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw".to_string());
                params.insert("blskey_pop", "RPLagxaR5xdimFzwmzYnz4ZhWtYQEj8iR5ZU53T2gitPCyCHQneUn2Huc4oeLd2B2HzkGnjAff4hWTJT6C7qHYB1Mv2wU5iHHGFWkhnTX9WsEAbunJCV2qcaXScKj4tTfvdDKfLiVuU2av6hbsMztirRze7LvYBkRHV3tGwyCptsrP".to_string());
                params.insert("services", "VALIDATOR".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod pool_config {
        use super::*;

        #[test]
        pub fn pool_config_works() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = pool_config_command::new();
                let mut params = CommandParams::new();
                params.insert("writes", "false".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            {
                let cmd = pool_config_command::new();
                let mut params = CommandParams::new();
                params.insert("writes", "true".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod pool_restart {
        use super::*;

        #[test]
        pub fn pool_restart_works() {
            let datetime = r#"2020-01-25T12:49:05.258870+00:00"#;

            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = pool_restart_command::new();
                let mut params = CommandParams::new();
                params.insert("action", "start".to_string());
                params.insert("datetime", datetime.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn pool_restart_works_for_nodes() {
            let datetime = r#"2020-01-25T12:49:05.258870+00:00"#;

            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = pool_restart_command::new();
                let mut params = CommandParams::new();
                params.insert("action", "start".to_string());
                params.insert("datetime", datetime.to_string());
                params.insert("nodes", "Node1,Node2".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn pool_restart_works_for_timeout() {
            let datetime = r#"2020-01-25T12:49:05.258870+00:00"#;

            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = pool_restart_command::new();
                let mut params = CommandParams::new();
                params.insert("action", "start".to_string());
                params.insert("datetime", datetime.to_string());
                params.insert("nodes", "Node1,Node2".to_string());
                params.insert("timeout", "10".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod pool_upgrade {
        use super::*;

        #[test]
        #[ignore]
        pub fn pool_upgrade_works() {
            let schedule = r#"{"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv":"2020-01-25T12:49:05.258870+00:00",
                                    "8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb":"2020-01-25T13:49:05.258870+00:00",
                                    "DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya":"2020-01-25T14:49:05.258870+00:00",
                                    "4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA":"2020-01-25T15:49:05.258870+00:00"}"#;

            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = pool_upgrade_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "upgrade-indy-cli".to_string());
                params.insert("version", "2.0.0".to_string());
                params.insert("action", "start".to_string());
                params.insert("sha256", "f284bdc3c1c9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398".to_string());
                params.insert("schedule", schedule.to_string());
                params.insert("force", "true".to_string()); // because node_works test added fifth Node
                cmd.execute(&ctx, &params).unwrap();
            }
            // There is no way to read upgrade transaction to be sure about completely write before sending next one.
            // So just sleep agains other places where control read request is available
            ::std::thread::sleep(::std::time::Duration::from_secs(1));
            {
                let cmd = pool_upgrade_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "upgrade-indy-cli".to_string());
                params.insert("version", "2.0.0".to_string());
                params.insert("action", "cancel".to_string());
                params.insert("sha256", "ac3eb2cc3ac9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod custom {
        use super::*;

        pub const TXN: &'static str = r#"{
                                            "reqId":1513241300414292814,
                                            "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                                            "operation":{
                                                "type":"105",
                                                "dest":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL"
                                            },
                                            "protocolVersion":2
                                          }"#;

        pub const TXN_FOR_SIGN: &'static str = r#"{
                                                    "reqId":1513241300414292814,
                                                    "identifier":"V4SGRU86Z58d6TV7PBUe6f",
                                                    "operation":{
                                                        "type":"1",
                                                        "dest":"E1XWGvsrVp5ZDif2uDdTAM",
                                                        "verkey":"86F43kmApX7Da5Rcba1vCbYmc7bbauEksGxPKy8PkZyb"
                                                    },
                                                    "protocolVersion":2
                                                  }"#;

        #[test]
        pub fn custom_works() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", TXN.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn custom_works_for_sign() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("sign", "true".to_string());
                params.insert("txn", TXN_FOR_SIGN.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn custom_works_for_missed_txn_field() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = custom_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn custom_works_for_invalid_transaction_format() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", format!(r#"
                                                    "reqId":1513241300414292814,
                                                    "identifier":"{}",
                                                    "protocolVersion":2
                                                  "#, DID_TRUSTEE));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn custom_works_for_no_opened_pool() {
            let ctx = setup();

            create_and_open_wallet(&ctx);

            use_trustee(&ctx);
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", TXN.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            tear_down();
        }


        #[test]
        pub fn custom_works_for_sign_without_active_did() {
            let ctx = setup_with_wallet_and_pool();
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("sign", "true".to_string());
                params.insert("txn", TXN.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn custom_works_for_unknown_submitter_did() {
            let ctx = setup_with_wallet_and_pool();

            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("sign", "true".to_string());
                params.insert("txn", TXN_FOR_SIGN.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    #[cfg(feature = "nullpay_plugin")]
    mod get_payment_sources {
        use super::*;

        #[test]
        pub fn get_payment_sources_works() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            let payment_address = create_address_and_mint_sources(&ctx);
            {
                let cmd = get_payment_sources_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", payment_address);
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_payment_sources_works_for_no_sources() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = get_payment_sources_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", PAYMENT_ADDRESS.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_payment_sources_works_for_unknown_payment_method() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = get_payment_sources_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", format!("pay:{}:test", UNKNOWN_PAYMENT_METHOD));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_payment_sources_works_for_invalid_payment_address() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = get_payment_sources_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", INVALID_PAYMENT_ADDRESS.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_payment_sources_works_for_no_active_wallet() {
            let ctx = setup();

            ::commands::pool::tests::create_and_connect_pool(&ctx);
            load_null_payment_plugin(&ctx);
            {
                let cmd = get_payment_sources_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", INVALID_PAYMENT_ADDRESS.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            disconnect_and_delete_pool(&ctx);
            tear_down();
        }

        #[test]
        pub fn get_payment_sources_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            {
                let cmd = get_payment_sources_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", PAYMENT_ADDRESS.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_payment_sources_works_for_extra() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, AMOUNT));
                params.insert("extra", EXTRA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            {
                let cmd = get_payment_sources_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", PAYMENT_ADDRESS.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    #[cfg(feature = "nullpay_plugin")]
    mod payment {
        use super::*;

        #[test]
        pub fn payment_works() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", input);
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, 10));
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_extra() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", input);
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, 10));
                params.insert("extra", EXTRA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_multiple_inputs() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);

            let payment_address_from_1 = create_address_and_mint_sources(&ctx);
            let input_1 = get_source_input(&ctx, &payment_address_from_1);

            let payment_address_from_2 = create_address_and_mint_sources(&ctx);
            let input_2 = get_source_input(&ctx, &payment_address_from_2);

            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", format!("{},{}", input_1, input_2));
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, 150));
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_one_input_and_multiple_outputs() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);

            let payment_address_from_1 = create_address_and_mint_sources(&ctx);
            let input_1 = get_source_input(&ctx, &payment_address_from_1);

            let payment_address_to = create_payment_address(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", format!("{}", input_1));
                params.insert("outputs", format!("({},{}),({},{})", PAYMENT_ADDRESS, 10, payment_address_to, 20));
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_multiple_inputs_and_outputs() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);

            let payment_address_from_1 = create_address_and_mint_sources(&ctx);
            let input_1 = get_source_input(&ctx, &payment_address_from_1);

            let payment_address_from_2 = create_address_and_mint_sources(&ctx);
            let input_2 = get_source_input(&ctx, &payment_address_from_2);

            let payment_address_to = create_payment_address(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", format!("{},{}", input_1, input_2));
                params.insert("outputs", format!("({},{}),({},{})", PAYMENT_ADDRESS, 10, payment_address_to, 20));
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_not_enough_amount() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);

            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", input);
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, 1000));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_unknown_input() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", INPUT.to_string());
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, 10));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_unknown_payment_method() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", format!("pay:{}:111_rBuQo2A1sc9jrJg", UNKNOWN_PAYMENT_METHOD));
                params.insert("outputs", format!("(pay:{}:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,100)", UNKNOWN_PAYMENT_METHOD));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_incompatible_payment_methods() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", "pay:null_method_1:111_rBuQo2A1sc9jrJg".to_string());
                params.insert("outputs", "(pay:null_method_2:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,100))".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_empty_inputs() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", r#""#.to_string());
                params.insert("outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_empty_outputs() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", INPUT.to_string());
                params.insert("outputs", "".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_invalid_inputs() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", INVALID_INPUT.to_string());
                params.insert("outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_invalid_outputs() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", INPUT.to_string());
                params.insert("outputs", r#"(pay:null,CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW)"#.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", r#"pay:null"#.to_string());
                params.insert("outputs", INVALID_OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_several_equal_inputs() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", format!("{},{}", INPUT, INPUT));
                params.insert("outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_negative_inputs_amount() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", "pay:null:-11_wD7gzzUlOnYRkb4".to_string());
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, 10));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn payment_works_for_negative_outputs_amount() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);

            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", input);
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, -10));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    #[cfg(feature = "nullpay_plugin")]
    mod get_fees {
        use super::*;

        #[test]
        pub fn get_fees_works() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            set_fees(&ctx, FEES);
            {
                let cmd = get_fees_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_fees_works_for_no_fees() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = get_fees_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_fees_works_for_unknown_payment_method() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = get_fees_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", UNKNOWN_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn get_fees_works_for_no_active_wallet() {
            let ctx = setup();

            ::commands::pool::tests::create_and_connect_pool(&ctx);
            load_null_payment_plugin(&ctx);
            {
                let cmd = get_fees_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            disconnect_and_delete_pool(&ctx);
            tear_down();
        }

        #[test]
        pub fn get_fees_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            {
                let cmd = get_fees_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    #[cfg(feature = "nullpay_plugin")]
    mod mint_prepare {
        use super::*;

        #[test]
        pub fn mint_prepare_works() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn mint_prepare_works_for_big_amount() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", "(pay:null:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,10000000000)".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn mint_prepare_works_for_extra() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", OUTPUT.to_string());
                params.insert("extra", EXTRA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn mint_prepare_works_for_multiple_outputs() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", format!("{},{}", OUTPUT, OUTPUT_2));
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn mint_prepare_works_for_empty_outputs() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", "".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn mint_prepare_works_for_unknown_payment_method() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", format!("(pay:{}:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,100)", UNKNOWN_PAYMENT_METHOD));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn mint_prepare_works_for_invalid_outputs_format() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", INVALID_OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn mint_prepare_works_for_invalid_payment_address() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", "(pay:null,100)".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn mint_prepare_works_for_incompatible_payment_methods() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", "(pay:null_1:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,100),(pay:null_2:GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa,11)".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn mint_prepare_works_for_negative_amount() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", "(pay:null:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,-10)".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn mint_prepare_works_for_multiple_outputs_negative_amount_for_second() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", "(pay:null:address1,10),(pay:null:address2,-10)".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    #[cfg(feature = "nullpay_plugin")]
    mod set_fees_prepare {
        use super::*;

        #[test]
        pub fn set_fees_prepare_works() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = set_fees_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                params.insert("fees", FEES.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn set_fees_prepare_works_for_unknown_payment_method() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = set_fees_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", UNKNOWN_PAYMENT_METHOD.to_string());
                params.insert("fees", FEES.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn set_fees_prepare_works_for_invalid_fees_format() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = set_fees_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                params.insert("fees", "1,100".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn set_fees_prepare_works_for_empty_fees() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = set_fees_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                params.insert("fees", "".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn set_fees_prepare_works_for_no_active_wallet() {
            let ctx = setup();
            pool::tests::create_and_connect_pool(&ctx);
            common::tests::load_null_payment_plugin(&ctx);
            {
                let cmd = set_fees_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                params.insert("fees", FEES.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            disconnect_and_delete_pool(&ctx);
            tear_down();
        }

        #[test]
        pub fn set_fees_prepare_works_for_negative_amount() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = set_fees_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                params.insert("fees", "1:-1,101:-1".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    #[cfg(feature = "nullpay_plugin")]
    mod verify_payment_receipts {
        use super::*;

        #[test]
        pub fn verify_payment_receipts_works() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);

            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);
            {
                let cmd = verify_payment_receipt_command::new();
                let mut params = CommandParams::new();
                params.insert("receipt", input);
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn verify_payment_receipts_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);

            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);

            // to reset active did
            close_wallet(&ctx);
            open_wallet(&ctx);

            {
                let cmd = verify_payment_receipt_command::new();
                let mut params = CommandParams::new();
                params.insert("receipt", input);
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn verify_payment_receipts_works_for_not_found() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = verify_payment_receipt_command::new();
                let mut params = CommandParams::new();
                params.insert("receipt", "pay:null:0_PqVjwJC42sxCTJp".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn verify_payment_receipts_works_for_invalid_receipt() {
            let ctx = setup_with_wallet_and_pool_and_payment_plugin();
            use_trustee(&ctx);
            {
                let cmd = verify_payment_receipt_command::new();
                let mut params = CommandParams::new();
                params.insert("receipt", "invalid_receipt".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod sign_multi {
        use super::*;

        #[test]
        pub fn sign_multi_works() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = sign_multi_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", r#"{"reqId":1496822211362017764}"#.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn sign_multi_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool();
            {
                let cmd = sign_multi_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", r#"{"reqId":1496822211362017764}"#.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn sign_multi_works_for_invalid_message_format() {
            let ctx = setup_with_wallet_and_pool();
            use_trustee(&ctx);
            {
                let cmd = sign_multi_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", r#"1496822211362017764"#.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    fn create_new_did(ctx: &CommandContext) -> (String, String) {
        let (wallet_handle, _) = get_opened_wallet(ctx).unwrap();
        Did::new(wallet_handle, "{}").unwrap()
    }

    fn use_trustee(ctx: &CommandContext) {
        new_did(&ctx, SEED_TRUSTEE);
        use_did(&ctx, DID_TRUSTEE);
    }

    fn use_new_identity(ctx: &CommandContext) -> (String, String) {
        use_trustee(ctx);
        let (did, verkey) = create_new_did(ctx);
        send_nym(ctx, &did, &verkey, Some("TRUST_ANCHOR"));
        use_did(&ctx, &did);
        (did, verkey)
    }

    pub fn send_schema(ctx: &CommandContext, did: &str) -> String {
        let (pool_handle, _) = get_connected_pool(ctx).unwrap();
        let (wallet_handle, _) = get_opened_wallet(ctx).unwrap();
        let schema_data = r#"{"id":"id", "name":"cli_gvt","version":"1.0","attrNames":["name"],"ver":"1.0"}"#;
        let schema_request = Ledger::build_schema_request(&did, schema_data).unwrap();
        let schema_response = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &schema_request).unwrap();
        let schema: serde_json::Value = serde_json::from_str(&schema_response).unwrap();
        let seq_no = schema["result"]["txnMetadata"]["seqNo"].as_i64().unwrap();
        seq_no.to_string()
    }

    pub fn send_nym(ctx: &CommandContext, did: &str, verkey: &str, role: Option<&str>) {
        let cmd = nym_command::new();
        let mut params = CommandParams::new();
        params.insert("did", did.to_string());
        params.insert("verkey", verkey.to_string());
        if let Some(role) = role {
            params.insert("role", role.to_string());
        }
        cmd.execute(&ctx, &params).unwrap();
    }

    #[cfg(feature = "nullpay_plugin")]
    pub fn create_address_and_mint_sources(ctx: &CommandContext) -> String {
        let (wallet_handle, _) = get_opened_wallet(ctx).unwrap();
        let submitter_did = ensure_active_did(&ctx).unwrap();

        let payment_address = create_payment_address(&ctx);

        Payment::build_mint_req(wallet_handle,
                                Some(&submitter_did),
                                &parse_payment_outputs(&vec![format!("{},{}", payment_address, AMOUNT)]).unwrap(),
                                None).unwrap();
        payment_address
    }

    #[cfg(feature = "nullpay_plugin")]
    pub fn get_source_input(ctx: &CommandContext, payment_address: &str) -> String {
        let (pool_handle, _) = get_connected_pool(ctx).unwrap();
        let (wallet_handle, _) = get_opened_wallet(ctx).unwrap();
        let submitter_did = ensure_active_did(&ctx).unwrap();

        let (get_sources_txn_json, _) = Payment::build_get_payment_sources_request(wallet_handle, Some(&submitter_did), payment_address).unwrap();
        let response = Ledger::submit_request(pool_handle, &get_sources_txn_json).unwrap();

        let sources_json = Payment::parse_get_payment_sources_response(NULL_PAYMENT_METHOD, &response).unwrap();

        let sources = serde_json::from_str::<serde_json::Value>(&sources_json).unwrap();
        let source: &serde_json::Value = &sources.as_array().unwrap()[0];
        source["source"].as_str().unwrap().to_string()
    }

    #[cfg(feature = "nullpay_plugin")]
    pub fn set_fees(ctx: &CommandContext, fees: &str) {
        {
            let cmd = set_fees_prepare_command::new();
            let mut params = CommandParams::new();
            params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
            params.insert("fees", fees.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }

    fn _ensure_nym_added(ctx: &CommandContext, did: &str) {
        let request = Ledger::build_get_nym_request(None, did).unwrap();
        submit_retry(ctx, &request, |response| {
            serde_json::from_str::<Response<ReplyResult<String>>>(&response)
                .and_then(|response| serde_json::from_str::<serde_json::Value>(&response.result.unwrap().data))
        }).unwrap();
    }

    fn _ensure_attrib_added(ctx: &CommandContext, did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) {
        let attr = if raw.is_some() { Some("endpoint") } else { None };
        let request = Ledger::build_get_attrib_request(None, did, attr, hash, enc).unwrap();
        submit_retry(ctx, &request, |response| {
            serde_json::from_str::<Response<ReplyResult<String>>>(&response)
                .map_err(|_| ())
                .and_then(|response| {
                    let expected_value = if raw.is_some() { raw.unwrap() } else if hash.is_some() { hash.unwrap() } else { enc.unwrap() };
                    if response.result.is_some() && expected_value == response.result.unwrap().data { Ok(()) } else { Err(()) }
                })
        }).unwrap();
    }

    fn _ensure_schema_added(ctx: &CommandContext, did: &str) {
        let id = build_schema_id(did, "gvt", "1.0");
        let request = Ledger::build_get_schema_request(None, &id).unwrap();
        submit_retry(ctx, &request, |response| {
            let schema: serde_json::Value = serde_json::from_str(&response).unwrap();
            schema["result"]["seqNo"].as_i64().ok_or(())
        }).unwrap();
    }

    fn _ensure_cred_def_added(ctx: &CommandContext, did: &str, schema_id: &str) {
        let id = build_cred_def_id(did, schema_id, "CL", "TAG");
        let request = Ledger::build_get_cred_def_request(None, &id).unwrap();
        submit_retry(ctx, &request, |response| {
            let cred_def: serde_json::Value = serde_json::from_str(&response).unwrap();
            cred_def["result"]["seqNo"].as_i64().ok_or(())
        }).unwrap();
    }
}