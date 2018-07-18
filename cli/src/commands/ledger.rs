extern crate regex;
extern crate chrono;

use command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;
use commands::payment_address::handle_payment_error;

use libindy::ErrorCode;
use libindy::ledger::Ledger;
use libindy::payment::Payment;

use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use std::collections::HashMap;
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
                .add_optional_param("role", "Role of identity. One of: STEWARD, TRUSTEE, TRUST_ANCHOR, TGB or empty in case of blacklisting NYM")
                .add_optional_param("fees_inputs","The list of UTXO inputs")
                .add_optional_param("fees_outputs","The list of UTXO outputs")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX verkey=GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX role=TRUSTEE")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX role=")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX fees_inputs=txo:sov:111_rBuQo2A1sc9jrJg fees_outputs=(pay:sov:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100)")
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

        let mut request = Ledger::build_nym_request(&submitter_did, target_did, verkey, None, role)
            .map_err(|err| handle_build_request_error(err))?;

        let payment_method = set_request_fees(&mut request, wallet_handle, &submitter_did, &fees_inputs, &fees_outputs)?;

        let response = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request)
            .map_err(|err| handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

        let utxo = parse_response_with_fees(&response, payment_method)?;

        let mut response: Response<serde_json::Value> = serde_json::from_str::<Response<serde_json::Value>>(&response)
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

        let res = print_response_utxo(utxo);

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

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;

        let target_did = get_str_param("did", params).map_err(error_err!())?;

        let response = Ledger::build_get_nym_request(&submitter_did, target_did)
            .and_then(|request| Ledger::submit_request(pool_handle, &request))
            .map_err(|err| handle_transaction_error(err, None, None, None))?;

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
                .add_optional_param("fees_inputs","The list of UTXO inputs")
                .add_optional_param("fees_outputs","The list of UTXO outputs")
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX raw={"endpoint":{"ha":"127.0.0.1:5555"}}"#)
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX hash=83d907821df1c87db829e96569a11f6fc2e7880acba5e43d07ab786959e13bd3"#)
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX enc=aa3f41f619aa7e5e6b6d0d"#)
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX enc=aa3f41f619aa7e5e6b6d0d fees_inputs=txo:sov:111_rBuQo2A1sc9jrJg fees_outputs=(pay:sov:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100)"#)
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

        let mut request = Ledger::build_attrib_request(&submitter_did, target_did, hash, raw, enc)
            .map_err(|err| handle_build_request_error(err))?;

        let payment_method = set_request_fees(&mut request, wallet_handle, &submitter_did, &fees_inputs, &fees_outputs)?;

        let response = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request)
            .map_err(|err| handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

        let utxo = parse_response_with_fees(&response, payment_method)?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
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

        let res = print_response_utxo(utxo);

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
                .add_example("ledger get-attrib did=VsKV7grR1BUE29mG2Fm2kX attr=endpoint")
                .add_example("ledger get-attrib did=VsKV7grR1BUE29mG2Fm2kX hash=83d907821df1c87db829e96569a11f6fc2e7880acba5e43d07ab786959e13bd3")
                .add_example("ledger get-attrib did=VsKV7grR1BUE29mG2Fm2kX enc=aa3f41f619aa7e5e6b6d0d")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let raw = get_opt_str_param("raw", params).map_err(error_err!())?;
        let hash = get_opt_str_param("hash", params).map_err(error_err!())?;
        let enc = get_opt_str_param("enc", params).map_err(error_err!())?;

        let response = Ledger::build_get_attrib_request(&submitter_did, target_did, raw, hash, enc)
            .and_then(|request| Ledger::submit_request(pool_handle, &request))
            .map_err(|err| handle_transaction_error(err, None, None, None))?;

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
                .add_required_param("attr_names", "Schema attributes split by comma")
                .add_optional_param("fees_inputs","The list of UTXO inputs")
                .add_optional_param("fees_outputs","The list of UTXO outputs")
                .add_example("ledger schema name=gvt version=1.0 attr_names=name,age")
                .add_example("ledger schema name=gvt version=1.0 attr_names=name,age fees_inputs=txo:sov:111_rBuQo2A1sc9jrJg fees_outputs=(pay:sov:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100)")
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
            .map_err(|err| handle_build_request_error(err))?;

        let payment_method = set_request_fees(&mut request, wallet_handle, &submitter_did, &fees_inputs, &fees_outputs)?;

        let response = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request)
            .map_err(|err| handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

        let utxo = parse_response_with_fees(&response, payment_method)?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Schema request has been sent to Ledger.",
                                                     Some("data"),
                                                     &[("name", "Name"),
                                                         ("version", "Version"),
                                                         ("attr_names", "Attributes")]))?;

        let res = print_response_utxo(utxo);

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_validator_info_command {
    use super::*;

    command!(CommandMetadata::build("get-validator-info", "Get validator info from all nodes.")
                .add_example(r#"ledger get-validator-info"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let response = Ledger::build_get_validator_info_request(&submitter_did)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request))
            .map_err(|err| handle_transaction_error(err, None, None, None))?;

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
            println_succ!("Get validator info response for node {}:", node);
            let _res = handle_transaction_response(response).map(|result| println!("{}", result));
        }
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

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let name = get_str_param("name", params).map_err(error_err!())?;
        let version = get_str_param("version", params).map_err(error_err!())?;

        let id = build_schema_id(target_did, name, version);

        let response = Ledger::build_get_schema_request(&submitter_did, &id)
            .and_then(|request| Ledger::submit_request(pool_handle, &request))
            .map_err(|err| handle_transaction_error(err, None, None, None))?;

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
                .add_optional_param("fees_inputs","The list of UTXO inputs")
                .add_optional_param("fees_outputs","The list of UTXO outputs")
                .add_example(r#"ledger cred-def schema_id=1 signature_type=CL primary={"n":"1","s":"2","rms":"3","r":{"age":"4","name":"5"},"rctxt":"6","z":"7"}"#)
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
            .map_err(|err| handle_build_request_error(err))?;

        let payment_method = set_request_fees(&mut request, wallet_handle, &submitter_did, &fees_inputs, &fees_outputs)?;

        let response = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request)
            .map_err(|err| handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

        let utxo = parse_response_with_fees(&response, payment_method)?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "NodeConfig request has been sent to Ledger.",
                                                     Some("data"),
                                                     &[("primary", "Primary Key"),
                                                         ("revocation", "Revocation Key")]))?;

        let res = print_response_utxo(utxo);

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
                .add_example("ledger get-cred-def schema_id=1 signature_type=CL origin=VsKV7grR1BUE29mG2Fm2kX")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;

        let schema_id = get_str_param("schema_id", params).map_err(error_err!())?;
        let signature_type = get_str_param("signature_type", params).map_err(error_err!())?;
        let tag = get_opt_str_param("tag", params).map_err(error_err!())?.unwrap_or("");
        let origin = get_str_param("origin", params).map_err(error_err!())?;

        let id = build_cred_def_id(&origin, schema_id, signature_type, tag);

        let response = Ledger::build_get_cred_def_request(&submitter_did, &id)
            .and_then(|request| Ledger::submit_request(pool_handle, &request))
            .map_err(|err| handle_transaction_error(err, None, None, None))?;

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
                .add_optional_param("services", "Node type. One of: VALIDATOR, OBSERVER or empty in case of blacklisting node")
                .add_example("ledger node target=A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y node_ip=127.0.0.1 node_port=9710 client_ip=127.0.0.1 client_port=9711 alias=Node5 services=VALIDATOR blskey=2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw")
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
        let services = get_opt_str_array_param("services", params).map_err(error_err!())?;

        let node_data = {
            let mut json = JSONMap::new();
            update_json_map_opt_key!(json, "node_ip", node_ip);
            update_json_map_opt_key!(json, "node_port", node_port);
            update_json_map_opt_key!(json, "client_ip", client_ip);
            update_json_map_opt_key!(json, "client_port", client_port);
            update_json_map_opt_key!(json, "alias", alias);
            update_json_map_opt_key!(json, "blskey", blskey);
            update_json_map_opt_key!(json, "services", services);
            JSONValue::from(json).to_string()
        };

        let response = Ledger::build_node_request(&submitter_did, target_did, &node_data)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request))
            .map_err(|err| handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

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
                                                         ("blskey", "Blskey")]));
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
            .map_err(|err| handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

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
                .add_optional_param("datetime", "Node restart datetime (only for action=start).")
                .add_example(r#"ledger pool-restart action=start datetime=2020-01-25T12:49:05.258870+00:00"#)
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

        let response = Ledger::indy_build_pool_restart_request(&submitter_did, action, datetime)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request))
            .map_err(|err| handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

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
                .add_example(r#"ledger pool-upgrade name=upgrade-1 version=2.0 action=start sha256=f284bdc3c1c9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398 schedule={"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv":"2020-01-25T12:49:05.258870+00:00"}"#)
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

        let response = Ledger::indy_build_pool_upgrade_request(&submitter_did, name, version, action, sha256,
                                                               timeout, schedule, justification, reinstall, force)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request))
            .map_err(|err| handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name)))?;

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
                                                         ("force", "Force Apply")]));
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
            response.map_err(|err| handle_transaction_error(err, Some(&submitter), Some(&pool_name), Some(&wallet)))?;

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

pub mod get_utxo_command {
    use super::*;

    command!(CommandMetadata::build("get-utxo", "Get UTXO list for payment address.")
                .add_required_param("payment_address","Target payment address")
                .add_example("ledger get-utxo payment_address=pay:sov:GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let payment_address = get_str_param("payment_address", params).map_err(error_err!())?;

        let (request, payment_method) = Payment::build_get_utxo_request(wallet_handle, &submitter_did, payment_address)
            .map_err(|err| handle_payment_error(err, None))?;

        let response = Ledger::submit_request(pool_handle, &request)
            .map_err(|err| handle_transaction_error(err, None, Some(&pool_name), Some(&wallet_name)))?;

        let res = match Payment::parse_get_utxo_response(&payment_method, &response) {
            Ok(utxo_json) => {
                let mut utxo: Vec<serde_json::Value> = serde_json::from_str(&utxo_json)
                    .map_err(|_| println_err!("Wrong data has been received"))?;

                print_list_table(&utxo,
                                 &vec![("txo", "Txo"),
                                       ("paymentAddress", "Payment Address"),
                                       ("amount", "Amount"),
                                       ("extra", "Extra")],
                                 "There are no utxo's");
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

    command!(CommandMetadata::build("payment", "Send request for doing tokens payment.")
                .add_required_param("inputs","The list of UTXO inputs")
                .add_required_param("outputs","The list of outputs")
                .add_example("ledger payment inputs=txo:sov:111_rBuQo2A1sc9jrJg outputs=(pay:sov:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100,extradata)")
                .add_example("ledger payment inputs=txo:sov:111_rBuQo2A1sc9jrJg,txo:sov:222_aEwACvA1sc9jrJg outputs=(pay:sov:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100,extradata),(pay:sov:ABABefwrhscbaAShva7dkx1d2dZ3zUF8ckg7wmL7ofN4,5)")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let inputs = get_str_array_param("inputs", params).map_err(error_err!())?;
        let outputs = get_str_tuple_array_param("outputs", params).map_err(error_err!())?;

        let inputs = parse_payment_inputs(&inputs).map_err(error_err!())?;
        let outputs = parse_payment_outputs(&outputs).map_err(error_err!())?;

        let (request, payment_method) = Payment::build_payment_req(wallet_handle, &submitter_did, &inputs, &outputs)
            .map_err(|err| handle_payment_error(err, None))?;

        let response = Ledger::submit_request(pool_handle, &request)
            .map_err(|err| handle_transaction_error(err, None, Some(&pool_name), Some(&wallet_name)))?;

        let res = match Payment::parse_payment_response(&payment_method, &response) {
            Ok(utxo_json) => {
                let mut utxo: Vec<serde_json::Value> = serde_json::from_str(&utxo_json)
                    .map_err(|_| println_err!("Wrong data has been received"))?;

                print_list_table(&utxo,
                                 &vec![("txo", "Txo"),
                                       ("paymentAddress", "Payment Address"),
                                       ("amount", "Amount"),
                                       ("extra", "Extra")],
                                 "There are no utxo's");
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
                .add_example("ledger get-fees payment_method=sov")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let payment_method = get_str_param("payment_method", params).map_err(error_err!())?;

        let request = Payment::build_get_txn_fees_req(wallet_handle, &submitter_did, payment_method)
            .map_err(|err| handle_payment_error(err, Some(payment_method)))?;

        let response = Ledger::submit_request(pool_handle, &request)
            .map_err(|err| handle_transaction_error(err, None, Some(&pool_name), Some(&wallet_name)))?;

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
            Err(err) => Err(println_err!("Invalid data has been received: {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod mint_prepare_command {
    use super::*;

    command!(CommandMetadata::build("mint-prepare", "Prepare MINT transaction.")
                .add_required_param("outputs","The list of UTXO outputs")
                .add_example("ledger mint-prepare outputs=(pay:sov:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100,extradata)")
                .add_example("ledger mint-prepare outputs=(pay:sov:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100,extradata),(pay:sov:ABABaaVwSascbaAShva7dkx1d2dZ3zUF8ckg7wmL7ofN4,5)")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let outputs = get_str_tuple_array_param("outputs", params).map_err(error_err!())?;
        let outputs = parse_payment_outputs(&outputs).map_err(error_err!())?;

        Payment::build_mint_req(wallet_handle, &submitter_did, &outputs)
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
                .add_example("ledger set-fees-prepare payment_method=sov fees=NYM:100,ATTRIB:200")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let payment_method = get_str_param("payment_method", params).map_err(error_err!())?;
        let fees = get_str_array_param("fees", params).map_err(error_err!())?;

        let fees = parse_payment_fees(&fees).map_err(error_err!())?;

        Payment::build_set_txn_fees_req(wallet_handle, &submitter_did, &payment_method, &fees)
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

pub mod sign_multi_command {
    use super::*;

    command!(CommandMetadata::build("sign-multi", "Add multi signature by current DID to transaction.")
                .add_required_param("txn","Transaction to sign")
                .add_example(r#"ledger sign-multi txn={"reqId":123456789,"type":"100"}"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;
        let submitter_did = ensure_active_did(&ctx)?;

        let txn = get_str_param("txn", params).map_err(error_err!())?;

        let res = match Ledger::multi_sign_request(wallet_handle, &submitter_did, txn) {
            Ok(request) => {
                println_succ!("Transaction has been signed:");
                println_succ!("{}", request);
                Ok(())
            }
            Err(ErrorCode::CommonInvalidStructure) => Err(println_err!("Invalid Transaction JSON")),
            Err(ErrorCode::WalletInvalidHandle) => Err(println_err!("Wallet: \"{}\" not found", wallet_name)),
            Err(ErrorCode::WalletItemNotFound) => Err(println_err!("Signer DID: \"{}\" not found", submitter_did)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err))
        };

        trace!("execute << {:?}", res);
        res
    }
}

fn set_request_fees(request: &mut String, wallet_handle: i32, submitter_did: &str, fees_inputs: &Option<Vec<&str>>, fees_outputs: &Option<Vec<String>>) -> Result<Option<String>, ()> {
    let mut payment_method: Option<String> = None;
    if let &Some(ref inputs) = fees_inputs {
        let inputs_json = parse_payment_inputs(&inputs)?;

        let outputs_json = if let &Some(ref o) = fees_outputs {
            parse_payment_outputs(&o)?
        } else { "[]".to_string() };

        *request = Payment::add_request_fees(wallet_handle, submitter_did, request, &inputs_json, &outputs_json)
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
                        "paymentAddress": parts.get(0)
                                            .ok_or(())
                                            .map_err(|_| println_err!("Invalid format of Outputs: Payment Address not found"))?,
                        "amount": parts.get(1)
                                    .ok_or(())
                                    .map_err(|_| println_err!("Invalid format of Outputs: Amount not found"))
                                    .and_then(|amount| amount.parse::<u32>()
                                        .map_err(|_| println_err!("Invalid format of Outputs: Amount must be integer and greather then 0")))?,
                        "extra": if parts.len() > 1 {Some(parts[2..].join(":"))} else { None }
                    }));
    }

    serde_json::to_string(&output_objects)
        .map_err(|_| println_err!("Wrong data has been received"))
}


fn parse_response_with_fees(response: &str, payment_method: Option<String>) -> Result<Option<Vec<serde_json::Value>>, ()> {
    let utxo = if let Some(method) = payment_method {
        Some(Payment::parse_response_with_fees(&method, &response)
            .map_err(|err| handle_payment_error(err, Some(&method)))
            .and_then(|fees| serde_json::from_str::<Vec<serde_json::Value>>(&fees)
                .map_err(|err| println_err!("Invalid data has been received: {:?}", err)))?)
    } else { None };

    Ok(utxo)
}

fn print_response_utxo(utxo: Option<Vec<serde_json::Value>>) -> Result<(), ()> {
    utxo.map(|utxo| {
        if !utxo.is_empty() {
            println_succ!("Following Utxo has been received.");
            print_list_table(&utxo,
                             &vec![("txo", "Txo"),
                                   ("paymentAddress", "Payment Address"),
                                   ("amount", "Amount"),
                                   ("extra", "Extra")],
                             "");
        }
    });
    Ok(())
}

fn parse_payment_fees(fees: &Vec<&str>) -> Result<String, ()> {
    let mut fees_map: HashMap<String, i32> = HashMap::new();

    for fee in fees {
        let parts = fee.split(":").collect::<Vec<&str>>();

        let type_ = parts.get(0)
            .ok_or(())
            .map_err(|_| println_err!("Invalid format of Fees: Type not found"))?
            .to_string();

        let amount = parts.get(1)
            .ok_or(())
            .map_err(|_| println_err!("Invalid format of Fees: Amount not found"))
            .and_then(|amount| amount.parse::<i32>()
                .map_err(|_| println_err!("Invalid format of Fees: Amount must be integer")))?;

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

pub fn handle_build_request_error(err: ErrorCode) {
    match err {
        ErrorCode::CommonInvalidStructure => println_err!("Invalid format of command params. Please check format of posted JSONs, Keys, DIDs and etc..."),
        err => println_err!("Indy SDK error occurred {:?}", err)
    }
}

pub fn handle_transaction_error(err: ErrorCode, submitter_did: Option<&str>, pool_name: Option<&str>, wallet_name: Option<&str>) {
    match err {
        ErrorCode::CommonInvalidStructure => println_err!("Invalid format of command params. Please check format of posted JSONs, Keys, DIDs and etc..."),
        ErrorCode::PoolLedgerInvalidPoolHandle => println_err!("Pool: \"{}\" not found", pool_name.unwrap_or("")),
        ErrorCode::WalletInvalidHandle => println_err!("Wallet: \"{}\" not found", wallet_name.unwrap_or("")),
        ErrorCode::WalletItemNotFound => println_err!("Submitter DID: \"{}\" not found", submitter_did.unwrap_or("")),
        ErrorCode::WalletIncompatiblePoolError =>
            println_err!("Wallet \"{}\" is incompatible with pool \"{}\".", wallet_name.unwrap_or(""), pool_name.unwrap_or("")),
        ErrorCode::PoolLedgerTimeout => println_err!("Transaction response has not been received"),
        err => println_err!("Indy SDK error occurred {:?}", err)
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
        Some("100") => "TGB",
        Some("101") => "TRUST_ANCHOR",
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
    use utils::test::TestUtils;
    use commands::wallet::tests::{create_and_open_wallet, close_and_delete_wallet};
    use commands::pool::tests::{create_and_connect_pool, disconnect_and_delete_pool};
    use commands::did::tests::{new_did, use_did, SEED_TRUSTEE, DID_TRUSTEE, SEED_MY1, DID_MY1, VERKEY_MY1, SEED_MY3, DID_MY3, VERKEY_MY3};
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
    pub const INPUT: &'static str = "txo:null:111_rBuQo2A1sc9jrJg";
    #[cfg(feature = "nullpay_plugin")]
    pub const OUTPUT: &'static str = "(pay:null:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,10)";
    #[cfg(feature = "nullpay_plugin")]
    pub const OUTPUT_2: &'static str = "(pay:null:GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa,25,some extra)";
    #[cfg(feature = "nullpay_plugin")]
    pub const INVALID_INPUT: &'static str = "txo:null";
    #[cfg(feature = "nullpay_plugin")]
    pub const INVALID_OUTPUT: &'static str = "pay:null:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,100";
    #[cfg(feature = "nullpay_plugin")]
    pub const FEES: &'static str = "NYM:1,ATTRIB:1,SCHEMA:1";
    #[cfg(feature = "nullpay_plugin")]
    pub const TOKES_COUNT: i32 = 100;

    mod nym {
        use super::*;

        #[test]
        pub fn nym_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_nym_added(&ctx, DID_MY1);
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn nym_works_for_role() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                params.insert("role", "TRUSTEE".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_nym_added(&ctx, DID_MY1);
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn nym_works_for_set_fees() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            load_null_payment_plugin(&ctx);
            set_fees(&ctx, FEES);
            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                params.insert("fees_inputs", input);
                params.insert("fees_outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_nym_added(&ctx, DID_MY1);
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn nym_works_for_set_fees_with_input_amount_lower_fee() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            load_null_payment_plugin(&ctx);
            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
            set_fees(&ctx, "NYM:101");
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                params.insert("fees_inputs", input);
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }


        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn nym_works_for_set_fees_with_input_amount_lower_fee_plus_output() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            load_null_payment_plugin(&ctx);
            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
            set_fees(&ctx, "NYM:95");
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                params.insert("fees_inputs", input);
                params.insert("fees_outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn nym_works_for_wrong_role() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                params.insert("role", "ROLE".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn nym_works_for_no_active_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn nym_works_for_no_opened_wallet() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            close_and_delete_wallet(&ctx);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn nym_works_for_no_connected_pool() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            disconnect_and_delete_pool(&ctx);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn nym_works_for_unknown_submitter() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
            {
                let cmd = nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY3.to_string());
                params.insert("verkey", VERKEY_MY3.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod get_nym {
        use super::*;

        #[test]
        pub fn get_nym_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            send_nym_my1(&ctx);
            _ensure_nym_added(&ctx, DID_MY1);
            {
                let cmd = get_nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_nym_works_for_unknown_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            send_nym_my1(&ctx);
            {
                let cmd = get_nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY3.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_nym_works_for_unknown_submitter() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            send_nym_my1(&ctx);
            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
            {
                let cmd = get_nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY3.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod attrib {
        use super::*;

        #[test]
        pub fn attrib_works_for_raw_value() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_TRUSTEE);
            send_nym_my1(&ctx);
            use_did(&ctx, DID_MY1);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, DID_MY1, Some(ATTRIB_RAW_DATA), None, None);
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn attrib_works_for_hash_value() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_TRUSTEE);
            send_nym_my1(&ctx);
            use_did(&ctx, DID_MY1);

            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("hash", ATTRIB_HASH_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, DID_MY1, None, Some(ATTRIB_HASH_DATA), None);
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn attrib_works_for_enc_value() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_TRUSTEE);
            send_nym_my1(&ctx);
            use_did(&ctx, DID_MY1);

            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("enc", ATTRIB_ENC_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, DID_MY1, None, None, Some(ATTRIB_ENC_DATA));
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn attrib_works_for_set_fees() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            let did = crate_send_and_use_new_nym(&ctx);

            load_null_payment_plugin(&ctx);
            set_fees(&ctx, FEES);
            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.to_string());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                params.insert("fees_inputs", input);
                params.insert("fees_outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, &did, Some(ATTRIB_RAW_DATA), None, None);
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn attrib_works_for_set_fees_input_amount_lower_fee() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            let did = crate_send_and_use_new_nym(&ctx);

            load_null_payment_plugin(&ctx);
            set_fees(&ctx, "ATTRIB:101");
            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", did.to_string());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                params.insert("fees_inputs", input);
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn attrib_works_for_missed_attribute() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_TRUSTEE);
            send_nym_my1(&ctx);
            use_did(&ctx, DID_MY1);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn attrib_works_for_no_active_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn attrib_works_for_unknown_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY3.to_string());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn attrib_works_for_invalid_endpoint_format() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_TRUSTEE);
            send_nym_my1(&ctx);
            use_did(&ctx, DID_MY1);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("raw", r#"127.0.0.1:5555"#.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod get_attrib {
        use super::*;

        #[test]
        pub fn get_attrib_works_for_raw_value() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_TRUSTEE);
            send_nym_my1(&ctx);
            use_did(&ctx, DID_MY1);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("raw", ATTRIB_RAW_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, DID_MY1, Some(ATTRIB_RAW_DATA), None, None);
            {
                let cmd = get_attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("raw", "endpoint".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_attrib_works_for_hash_value() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_TRUSTEE);
            send_nym_my1(&ctx);
            use_did(&ctx, DID_MY1);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("hash", ATTRIB_HASH_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, DID_MY1, None, Some(ATTRIB_HASH_DATA), None);
            {
                let cmd = get_attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("hash", ATTRIB_HASH_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_attrib_works_for_enc_value() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_TRUSTEE);
            send_nym_my1(&ctx);
            use_did(&ctx, DID_MY1);
            {
                let cmd = attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("enc", ATTRIB_ENC_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx, DID_MY1, None, None, Some(ATTRIB_ENC_DATA));
            {
                let cmd = get_attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("enc", ATTRIB_ENC_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_attrib_works_for_no_active_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            {
                let cmd = get_attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("attr", "endpoint".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod schema {
        use super::*;

        #[test]
        pub fn schema_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            let did = crate_send_and_use_new_nym(&ctx);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                params.insert("attr_names", "name,age".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_schema_added(&ctx, &did);
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn schema_works_for_set_fees() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            let did = crate_send_and_use_new_nym(&ctx);
            load_null_payment_plugin(&ctx);
            set_fees(&ctx, FEES);

            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
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
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn schema_works_for_set_fees_input_amount_lower_fee() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            crate_send_and_use_new_nym(&ctx);
            load_null_payment_plugin(&ctx);
            set_fees(&ctx, "SCHEMA:101");

            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                params.insert("attr_names", "name,age".to_string());
                params.insert("fees_inputs", input);
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn schema_works_for_missed_required_params() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn schema_works_unknown_submitter() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

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
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn schema_works_for_no_active_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                params.insert("attr_names", "name,age".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod get_validator_info {
        use super::*;

        #[test]
        pub fn get_validator_info_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = get_validator_info_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod get_schema {
        use super::*;

        #[test]
        pub fn get_schema_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            let did = crate_send_and_use_new_nym(&ctx);
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
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn schema_works_for_unknown_schema() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = get_schema_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                params.insert("name", "unknown_schema_name".to_string());
                params.insert("version", "1.0".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn schema_works_for_unknown_submitter() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

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
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn schema_works_for_no_active_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            {
                let cmd = get_schema_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod cred_def {
        use super::*;

        #[test]
        pub fn cred_def_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            let did = crate_send_and_use_new_nym(&ctx);
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
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn cred_def_works_for_set_fees() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            let did = crate_send_and_use_new_nym(&ctx);
            let schema_id = send_schema(&ctx, &did);

            load_null_payment_plugin(&ctx);
            set_fees(&ctx, FEES);
            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
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
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn cred_def_works_for_set_fees_input_amount_lower_fee() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            let did = crate_send_and_use_new_nym(&ctx);
            let schema_id = send_schema(&ctx, &did);

            load_null_payment_plugin(&ctx);
            set_fees(&ctx, "CRED_DEF:101");
            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
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
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn cred_def_works_for_missed_required_params() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", "1".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn cred_def_works_for_unknown_submitter() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

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
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn cred_def_works_for_no_active_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            {
                let cmd = cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", "1".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("primary", CRED_DEF_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod get_cred_def {
        use super::*;

        #[test]
        pub fn get_cred_def_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            let did = crate_send_and_use_new_nym(&ctx);
            let schema_id = send_schema(&ctx, &did);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", schema_id.clone());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("primary", CRED_DEF_DATA.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_cred_def_added(&ctx, DID_TRUSTEE, &schema_id);
            {
                let cmd = get_cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", schema_id);
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("origin", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_cred_def_works_for_unknown_cred_def() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = get_cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", "2".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("origin", DID_MY3.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_cred_def_works_for_no_active_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            {
                let cmd = get_cred_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_id", "1".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("tag", "TAG".to_string());
                params.insert("origin", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod node {
        use super::*;

        #[test]
        pub fn node_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            let my_seed = "00000000000000000000000MySTEWARD";
            let my_did = "GykzQ65PxaH3RUDypuwWTB";
            let my_verkey = "9i7fMkxTSdTaHkTmLqZ3exRkTfsQ5LLoxzDG1kjE8HLD";

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            new_did(&ctx, my_seed);
            use_did(&ctx, DID_TRUSTEE);
            send_nym(&ctx, my_did, my_verkey, Some("STEWARD"));
            use_did(&ctx, my_did);
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
                params.insert("services", "VALIDATOR".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod pool_config {
        use super::*;

        #[test]
        pub fn pool_config_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
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
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod pool_restart {
        use super::*;

        #[test]
        pub fn pool_restart_works() {
            TestUtils::cleanup_storage();
            let datetime = r#"2020-01-25T12:49:05.258870+00:00"#;

            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = pool_restart_command::new();
                let mut params = CommandParams::new();
                params.insert("action", "start".to_string());
                params.insert("datetime", datetime.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod pool_upgrade {
        use super::*;

        #[test]
        #[ignore]
        pub fn pool_upgrade_works() {
            TestUtils::cleanup_storage();
            let schedule = r#"{"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv":"2020-01-25T12:49:05.258870+00:00",
                                    "8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb":"2020-01-25T13:49:05.258870+00:00",
                                    "DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya":"2020-01-25T14:49:05.258870+00:00",
                                    "4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA":"2020-01-25T15:49:05.258870+00:00"}"#;

            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
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
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
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
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", TXN.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn custom_works_for_sign() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("sign", "true".to_string());
                params.insert("txn", TXN_FOR_SIGN.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn custom_works_for_missed_txn_field() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = custom_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn custom_works_for_invalid_transaction_format() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
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
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn custom_works_for_no_opened_pool() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", TXN.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }


        #[test]
        pub fn custom_works_for_sign_without_active_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("sign", "true".to_string());
                params.insert("txn", TXN.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn custom_works_for_unknown_submitter_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("sign", "true".to_string());
                params.insert("txn", TXN_FOR_SIGN.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    #[cfg(feature = "nullpay_plugin")]
    mod get_utxo {
        use super::*;

        #[test]
        pub fn get_utxo_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            let payment_address = create_address_and_mint_tokens(&ctx);
            {
                let cmd = get_utxo_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", payment_address);
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_utxo_works_for_no_utxos() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = get_utxo_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", PAYMENT_ADDRESS.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_utxo_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = get_utxo_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", format!("pay:{}:test", UNKNOWN_PAYMENT_METHOD));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_utxo_works_for_invalid_payment_address() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = get_utxo_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", INVALID_PAYMENT_ADDRESS.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_utxo_works_for_no_active_wallet() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            load_null_payment_plugin(&ctx);
            {
                let cmd = get_utxo_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", INVALID_PAYMENT_ADDRESS.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_utxo_works_for_no_active_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            {
                let cmd = get_utxo_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_address", PAYMENT_ADDRESS.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    #[cfg(feature = "nullpay_plugin")]
    mod payment {
        use super::*;

        #[test]
        pub fn payment_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", input);
                params.insert("outputs", format!("({},{},{})", PAYMENT_ADDRESS, 10, "some extra data"));
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_multiple_inputs() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            let payment_address_from_1 = create_address_and_mint_tokens(&ctx);
            let input_1 = get_utxo_input(&ctx, &payment_address_from_1);

            let payment_address_from_2 = create_address_and_mint_tokens(&ctx);
            let input_2 = get_utxo_input(&ctx, &payment_address_from_2);

            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", format!("{},{}", input_1, input_2));
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, 150));
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_one_input_and_multiple_outputs() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            let payment_address_from_1 = create_address_and_mint_tokens(&ctx);
            let input_1 = get_utxo_input(&ctx, &payment_address_from_1);

            let payment_address_to = create_payment_address(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", format!("{}", input_1));
                params.insert("outputs", format!("({},{}),({},{})", PAYMENT_ADDRESS, 10, payment_address_to, 20));
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_multiple_inputs_and_outputs() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            let payment_address_from_1 = create_address_and_mint_tokens(&ctx);
            let input_1 = get_utxo_input(&ctx, &payment_address_from_1);

            let payment_address_from_2 = create_address_and_mint_tokens(&ctx);
            let input_2 = get_utxo_input(&ctx, &payment_address_from_2);

            let payment_address_to = create_payment_address(&ctx);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", format!("{},{}", input_1, input_2));
                params.insert("outputs", format!("({},{}),({},{})", PAYMENT_ADDRESS, 10, payment_address_to, 20));
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_not_enough_amount() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", input);
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, 1000));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_unknown_input() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", INPUT.to_string());
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, 10));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", format!("txo:{}:111_rBuQo2A1sc9jrJg", UNKNOWN_PAYMENT_METHOD));
                params.insert("outputs", format!("(pay:{}:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,100)", UNKNOWN_PAYMENT_METHOD));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_incompatible_payment_methods() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", "txo:null_method_1:111_rBuQo2A1sc9jrJg".to_string());
                params.insert("outputs", "(pay:null_method_2:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,100))".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_empty_inputs() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", r#""#.to_string());
                params.insert("outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_empty_outputs() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", INPUT.to_string());
                params.insert("outputs", "".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_invalid_inputs() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", INVALID_INPUT.to_string());
                params.insert("outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_invalid_outputs() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
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
                params.insert("inputs", r#"txo:null"#.to_string());
                params.insert("outputs", INVALID_OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_several_equal_inputs() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", format!("{},{}", INPUT, INPUT));
                params.insert("outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_negative_inputs_amount() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", "pay:null:-11_wD7gzzUlOnYRkb4".to_string());
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, 10));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn payment_works_for_negative_outputs_amount() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            let payment_address_from = create_address_and_mint_tokens(&ctx);
            let input = get_utxo_input(&ctx, &payment_address_from);
            {
                let cmd = payment_command::new();
                let mut params = CommandParams::new();
                params.insert("inputs", input);
                params.insert("outputs", format!("({},{})", PAYMENT_ADDRESS, -10));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    #[cfg(feature = "nullpay_plugin")]
    mod get_fees {
        use super::*;

        #[test]
        pub fn get_fees_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            set_fees(&ctx, FEES);
            {
                let cmd = get_fees_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_fees_works_for_no_fees() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = get_fees_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_fees_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = get_fees_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", UNKNOWN_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_fees_works_for_no_active_wallet() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            load_null_payment_plugin(&ctx);
            {
                let cmd = get_fees_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn get_fees_works_for_no_active_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);

            {
                let cmd = get_fees_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    #[cfg(feature = "nullpay_plugin")]
    mod mint_prepare {
        use super::*;

        #[test]
        pub fn mint_prepare_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn mint_prepare_works_for_multiple_outputs() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", format!("{},{}", OUTPUT, OUTPUT_2));
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn mint_prepare_works_for_empty_outputs() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", "".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn mint_prepare_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", format!("(pay:{}:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,100)", UNKNOWN_PAYMENT_METHOD));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn mint_prepare_works_for_invalid_outputs_format() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", INVALID_OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn mint_prepare_works_for_invalid_payment_address() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", "(pay:null,100)".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn mint_prepare_works_for_incompatible_payment_methods() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = mint_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("outputs", "(pay:null_1:CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW,100),(pay:null_2:GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa,11)".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    #[cfg(feature = "nullpay_plugin")]
    mod set_fees_prepare {
        use super::*;

        #[test]
        pub fn set_fees_prepare_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = set_fees_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                params.insert("fees", FEES.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn set_fees_prepare_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = set_fees_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", UNKNOWN_PAYMENT_METHOD.to_string());
                params.insert("fees", FEES.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn set_fees_prepare_works_for_invalid_fees_format() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = set_fees_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                params.insert("fees", "NYM,ATTRIB".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn set_fees_prepare_works_for_empty_fees() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            create_and_open_wallet(&ctx);
            load_null_payment_plugin(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = set_fees_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                params.insert("fees", "".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn set_fees_prepare_works_for_no_active_wallet() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            load_null_payment_plugin(&ctx);
            {
                let cmd = set_fees_prepare_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                params.insert("fees", FEES.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            disconnect_and_delete_pool(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod sign_multi {
        use super::*;

        #[test]
        pub fn sign_multi_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = sign_multi_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", r#"{"reqId":1496822211362017764}"#.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn sign_multi_works_for_no_active_did() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            {
                let cmd = sign_multi_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", r#"{"reqId":1496822211362017764}"#.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn sign_multi_works_for_invalid_message_format() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = sign_multi_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", r#"1496822211362017764"#.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    use std::sync::{Once, ONCE_INIT};

    pub fn send_nym_my1(ctx: &CommandContext) {
        lazy_static! {
            static ref SEND_NYM: Once = ONCE_INIT;
        }

        SEND_NYM.call_once(|| {
            let cmd = nym_command::new();
            let mut params = CommandParams::new();
            params.insert("did", DID_MY1.to_string());
            params.insert("verkey", VERKEY_MY1.to_string());
            cmd.execute(&ctx, &params).unwrap();
        });
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

    pub fn crate_send_and_use_new_nym(ctx: &CommandContext) -> String {
        let (wallet_handle, _) = get_opened_wallet(ctx).unwrap();
        new_did(&ctx, SEED_TRUSTEE);
        use_did(&ctx, DID_TRUSTEE);
        let (did, verkey) = Did::new(wallet_handle, "{}").unwrap();
        send_nym(ctx, &did, &verkey, Some("TRUST_ANCHOR"));
        use_did(&ctx, &did);
        did
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
    pub fn create_address_and_mint_tokens(ctx: &CommandContext) -> String {
        let (wallet_handle, _) = get_opened_wallet(ctx).unwrap();
        let submitter_did = ensure_active_did(&ctx).unwrap();

        let payment_address = create_payment_address(&ctx);

        Payment::build_mint_req(wallet_handle,
                                &submitter_did,
                                &parse_payment_outputs(&vec![format!("{},{}", payment_address, TOKES_COUNT)]).unwrap()).unwrap();
        payment_address
    }

    #[cfg(feature = "nullpay_plugin")]
    pub fn get_utxo_input(ctx: &CommandContext, payment_address: &str) -> String {
        let (pool_handle, _) = get_connected_pool(ctx).unwrap();
        let (wallet_handle, _) = get_opened_wallet(ctx).unwrap();
        let submitter_did = ensure_active_did(&ctx).unwrap();

        let (get_utxo_txn_json, _) = Payment::build_get_utxo_request(wallet_handle, &submitter_did, payment_address).unwrap();
        let response = Ledger::submit_request(pool_handle, &get_utxo_txn_json).unwrap();

        let utxo_json = Payment::parse_get_utxo_response(NULL_PAYMENT_METHOD, &response).unwrap();

        let utxos = serde_json::from_str::<serde_json::Value>(&utxo_json).unwrap();
        let utxo: &serde_json::Value = &utxos.as_array().unwrap()[0];
        utxo["txo"].as_str().unwrap().to_string()
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
        let request = Ledger::build_get_nym_request(DID_TRUSTEE, did).unwrap();
        _submit_retry(ctx, &request, |response| {
            serde_json::from_str::<Response<ReplyResult<String>>>(&response)
                .and_then(|response| serde_json::from_str::<serde_json::Value>(&response.result.unwrap().data))
        }).unwrap();
    }

    fn _ensure_attrib_added(ctx: &CommandContext, did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) {
        let attr = if raw.is_some() { Some("endpoint") } else { None };
        let request = Ledger::build_get_attrib_request(DID_MY1, did, attr, hash, enc).unwrap();
        _submit_retry(ctx, &request, |response| {
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
        let request = Ledger::build_get_schema_request(DID_TRUSTEE, &id).unwrap();
        _submit_retry(ctx, &request, |response| {
            let schema: serde_json::Value = serde_json::from_str(&response).unwrap();
            schema["result"]["seqNo"].as_i64().ok_or(())
        }).unwrap();
    }

    fn _ensure_cred_def_added(ctx: &CommandContext, did: &str, schema_id: &str) {
        let id = build_cred_def_id(did, schema_id, "CL", "TAG");
        let request = Ledger::build_get_cred_def_request(DID_TRUSTEE, &id).unwrap();
        _submit_retry(ctx, &request, |response| {
            let cred_def: serde_json::Value = serde_json::from_str(&response).unwrap();
            cred_def["result"]["seqNo"].as_i64().ok_or(())
        }).unwrap();
    }

    fn _submit_retry<F, T, E>(ctx: &CommandContext, request: &str, parser: F) -> Result<(), ()>
        where F: Fn(&str) -> Result<T, E> {
        const SUBMIT_RETRY_CNT: usize = 3;
        const SUBMIT_TIMEOUT_SEC: u64 = 2;

        let pool_handle = ensure_connected_pool_handle(ctx).unwrap();

        for _ in 0..SUBMIT_RETRY_CNT {
            let response = Ledger::submit_request(pool_handle, request).unwrap();
            if parser(&response).is_ok() {
                return Ok(());
            }
            ::std::thread::sleep(::std::time::Duration::from_secs(SUBMIT_TIMEOUT_SEC));
        }

        return Err(());
    }
}