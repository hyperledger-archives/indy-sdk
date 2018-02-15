extern crate regex;
extern crate chrono;

use command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;

use libindy::ErrorCode;
use libindy::ledger::Ledger;

use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use std::collections::HashSet;
use utils::table::print_table;

use self::regex::Regex;
use self::chrono::prelude::*;

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
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX verkey=GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX role=TRUSTEE")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX role=")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let verkey = get_opt_str_param("verkey", params).map_err(error_err!())?;
        let role = get_opt_empty_str_param("role", params).map_err(error_err!())?;

        let response = Ledger::build_nym_request(&submitter_did, target_did, verkey, None, role)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let response = match response {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name))
        }?;

        let mut response: Response<serde_json::Value> = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        if let Some(result) = response.result.as_mut() {
            result["role"] = get_role_title(&result["role"]);
        }

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Nym request has been sent to Ledger.",
                                                     &[("reqId", "Request ID"),
                                                         ("txnTime", "Transaction time")],
                                                     None,
                                                     &mut vec![("dest", "Did"),
                                                               ("verkey", "Verkey"),
                                                               ("role", "Role")]));
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

        let res = Ledger::build_get_nym_request(&submitter_did, target_did)
            .and_then(|request| Ledger::submit_request(pool_handle, &request));

        let response = match res {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, None, None, None),
        }?;

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
                                                     &[("seqNo", "Sequence Number"),
                                                         ("reqId", "Request ID"),
                                                         ("txnTime", "Transaction time")],
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
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX raw={"endpoint":{"ha":"127.0.0.1:5555"}}"#)
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX hash=83d907821df1c87db829e96569a11f6fc2e7880acba5e43d07ab786959e13bd3"#)
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX enc=aa3f41f619aa7e5e6b6d0d"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let hash = get_opt_str_param("hash", params).map_err(error_err!())?;
        let raw = get_opt_str_param("raw", params).map_err(error_err!())?;
        let enc = get_opt_str_param("enc", params).map_err(error_err!())?;

        let response = Ledger::build_attrib_request(&submitter_did, target_did, hash, raw, enc)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let response = match response {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name))
        }?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        let attribute =
            if raw.is_some() {
                ("raw", "Raw value")
            } else if hash.is_some() {
                ("hash", "Hashed value")
            } else { ("enc", "Encrypted value") };

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Attrib request has been sent to Ledger.",
                                                     &[("dest", "Dest"),
                                                         ("seqNo", "Sequence Number"),
                                                         ("reqId", "Request ID"),
                                                         ("txnTime", "Transaction time")],
                                                     None,
                                                     &[attribute]));

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

        let res = Ledger::build_get_attrib_request(&submitter_did, target_did, raw, hash, enc)
            .and_then(|request| Ledger::submit_request(pool_handle, &request));

        let response = match res {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, None, None, None),
        }?;

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
                                                     &[("dest", "Did"),
                                                         ("seqNo", "Sequence Number"),
                                                         ("reqId", "Request ID"),
                                                         ("txnTime", "Transaction time")],
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
                .add_example("ledger schema name=gvt version=1.0 attr_names=name,age")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;

        let name = get_str_param("name", params).map_err(error_err!())?;
        let version = get_str_param("version", params).map_err(error_err!())?;
        let attr_names = get_str_array_param("attr_names", params).map_err(error_err!())?;

        let schema_data = {
            let mut json = JSONMap::new();
            json.insert("name".to_string(), JSONValue::from(name));
            json.insert("version".to_string(), JSONValue::from(version));
            json.insert("attr_names".to_string(), JSONValue::from(attr_names));
            JSONValue::from(json).to_string()
        };

        let response = Ledger::build_schema_request(&submitter_did, &schema_data)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let response = match response {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name))
        }?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "NodeConfig request has been sent to Ledger.",
                                                     &[("identifier", "Identifier"),
                                                         ("seqNo", "Sequence Number"),
                                                         ("reqId", "Request ID"),
                                                         ("txnTime", "Transaction time")],
                                                     Some("data"),
                                                     &[("name", "Name"),
                                                         ("version", "Version"),
                                                         ("attr_names", "Attributes")]));
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

        let schema_data = {
            let mut json = JSONMap::new();
            json.insert("name".to_string(), JSONValue::from(name));
            json.insert("version".to_string(), JSONValue::from(version));
            JSONValue::from(json).to_string()
        };

        let res = Ledger::build_get_schema_request(&submitter_did, target_did, &schema_data)
            .and_then(|request| Ledger::submit_request(pool_handle, &request));

        let response = match res {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, None, None, None),
        }?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        if let Some(result) = response.result.as_ref() {
            //TODO strange condition
            if !result["seqNo"].is_i64() {
                return Err(println_err!("Schema not found"));
            }
        };

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Following Schema has been received.",
                                                     &[("dest", "Did"),
                                                         ("seqNo", "Sequence Number"),
                                                         ("reqId", "Request ID"),
                                                         ("txnTime", "Transaction time")],
                                                     Some("data"),
                                                     &[("name", "Name"),
                                                         ("version", "Version"),
                                                         ("attr_names", "Attributes")]));
        trace!("execute << {:?}", res);
        res
    }
}

pub mod claim_def_command {
    use super::*;

    command!(CommandMetadata::build("claim-def", "Send Claim Def transaction to the Ledger.")
                .add_required_param("schema_no", "Sequence number of schema")
                .add_required_param("signature_type", "Signature type (only CL supported now)")
                .add_required_param("primary", "Primary key in json format")
                .add_optional_param("revocation", "Revocation key in json format")
                .add_example(r#"ledger claim-def schema_no=1 signature_type=CL primary={"n":"1","s":"2","rms":"3","r":{"age":"4","name":"5"},"rctxt":"6","z":"7"}"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;

        let xref = get_int_param::<i32>("schema_no", params).map_err(error_err!())?;
        let signature_type = get_str_param("signature_type", params).map_err(error_err!())?;
        let primary = get_object_param("primary", params).map_err(error_err!())?;
        let revocation = get_opt_str_param("revocation", params).map_err(error_err!())?;

        let claim_def_data = {
            let mut json = JSONMap::new();
            json.insert("primary".to_string(), primary);
            update_json_map_opt_key!(json, "revocation", revocation);
            JSONValue::from(json).to_string()
        };

        let response = Ledger::build_claim_def_txn(&submitter_did, xref, signature_type, &claim_def_data)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let response = match response {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name))
        }?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "NodeConfig request has been sent to Ledger.",
                                                     &[("identifier", "Identifier"),
                                                         ("seqNo", "Sequence Number"),
                                                         ("reqId", "Request ID"),
                                                         ("txnTime", "Transaction time")],
                                                     Some("data"),
                                                     &[("primary", "Primary Key"),
                                                         ("revocation", "Revocation Key")]));
        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_claim_def_command {
    use super::*;

    command!(CommandMetadata::build("get-claim-def", "Get Claim Definition from Ledger.")
                .add_required_param("schema_no", "Sequence number of schema")
                .add_required_param("signature_type", "Signature type (only CL supported now)")
                .add_required_param("origin", "Claim definition owner DID")
                .add_example("ledger get-claim-def schema_no=1 signature_type=CL origin=VsKV7grR1BUE29mG2Fm2kX")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;

        let xref = get_int_param::<i32>("schema_no", params).map_err(error_err!())?;
        let signature_type = get_str_param("signature_type", params).map_err(error_err!())?;
        let origin = get_str_param("origin", params).map_err(error_err!())?;

        let res = Ledger::build_get_claim_def_txn(&submitter_did, xref, signature_type, origin)
            .and_then(|request| Ledger::submit_request(pool_handle, &request));

        let response = match res {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, None, None, None),
        }?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        if let Some(result) = response.result.as_ref() {
            //TODO strange condition
            if !result["seqNo"].is_i64() {
                return Err(println_err!("Schema not found"));
            }
        };

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "Following Claim Definition has been received.",
                                                     &[("identifier", "Identifier"),
                                                         ("seqNo", "Sequence Number"),
                                                         ("reqId", "Request ID"),
                                                         ("txnTime", "Transaction time")],
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

        let submitter_did = ensure_active_did(&ctx)?;
        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;

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
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let response = match response {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name))
        }?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "NodeConfig request has been sent to Ledger.",
                                                     &[("identifier", "Identifier"),
                                                         ("seqNo", "Sequence Number"),
                                                         ("reqId", "Request ID"),
                                                         ("txnTime", "Transaction time")],
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

        let submitter_did = ensure_active_did(&ctx)?;
        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;

        let writes = get_bool_param("writes", params).map_err(error_err!())?;
        let force = get_opt_bool_param("force", params).map_err(error_err!())?.unwrap_or(false);

        let response = Ledger::indy_build_pool_config_request(&submitter_did, writes, force)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let response = match response {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name))
        }?;

        let response = serde_json::from_str::<Response<serde_json::Value>>(&response)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

        let res = handle_transaction_response(response)
            .map(|result| print_transaction_response(result,
                                                     "NodeConfig request has been sent to Ledger.",
                                                     &[("identifier", "Identifier"),
                                                         ("seqNo", "Sequence Number"),
                                                         ("reqId", "Request ID"),
                                                         ("txnTime", "Transaction time")],
                                                     None,
                                                     &[("writes", "Writes"),
                                                         ("force", "Force Apply")]));
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

        let submitter_did = ensure_active_did(&ctx)?;
        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;

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
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let response = match response {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, Some(&submitter_did), Some(&pool_name), Some(&wallet_name))
        }?;

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
                                                     &[("identifier", "Identifier"),
                                                         ("seqNo", "Sequence Number"),
                                                         ("reqId", "Request ID"),
                                                         ("txnTime", "Transaction time")],
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
                .add_example(r#"ledger custom {"reqId":1,"identifier":"V4SGRU86Z58d6TV7PBUe6f","operation":{"type":"105","dest":"V4SGRU86Z58d6TV7PBUe6f"},"protocolVersion":1}"#)
                .add_example(r#"ledger custom {"reqId":2,"identifier":"V4SGRU86Z58d6TV7PBUe6f","operation":{"type":"1","dest":"VsKV7grR1BUE29mG2Fm2kX"},"protocolVersion":1} sign=true"#)
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

        let response_json = match response {
            Ok(response) => Ok(response),
            Err(err) => handle_transaction_error(err, Some(&submitter), Some(&pool_name), Some(&wallet))
        }?;

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

fn print_transaction_response(mut result: serde_json::Value, title: &str,
                              metadata_headers: &[(&str, &str)],
                              data_field: Option<&str>,
                              data_headers: &[(&str, &str)]) {
    if let Some(txn_time) = result["txnTime"].as_i64() {
        result["txnTime"] = serde_json::Value::String(timestamp_to_datetime(txn_time))
    }

    println_succ!("{}", title);
    println_succ!("Metadata:");
    print_table(&result, metadata_headers);
    println_succ!("Data:");

    let data = if data_field.is_some() { &result[data_field.unwrap()] } else { &result };
    let mut data_headers = data_headers.to_vec();
    data_headers.retain(|&(ref key, _)| !data[key].is_null());

    print_table(data, &data_headers);
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

pub fn handle_transaction_error(err: ErrorCode, submitter_did: Option<&str>, pool_name: Option<&str>, wallet_name: Option<&str>) -> Result<String, ()> {
    match err {
        ErrorCode::CommonInvalidStructure => Err(println_err!("Invalid format of command params. Please check format of posted JSONs, Keys, DIDs and etc...")),
        ErrorCode::WalletNotFoundError => Err(println_err!("Submitter DID: \"{}\" not found", submitter_did.unwrap_or(""))),
        ErrorCode::WalletIncompatiblePoolError => Err(println_err!("Wallet \"{}\" is incompatible with pool \"{}\".", wallet_name.unwrap_or(""), pool_name.unwrap_or(""))),
        ErrorCode::PoolLedgerTimeout => Err(println_err!("Transaction response has not been received")),
        err => Err(println_err!("Indy SDK error occurred {:?}", err))
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

#[derive(Deserialize, Debug)]
pub struct NymData {
    pub identifier: Option<String>,
    pub dest: String,
    pub role: Option<String>,
    pub verkey: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct AttribData {
    pub endpoint: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SchemaData {
    pub attr_names: HashSet<String>,
    pub name: String,
    pub version: String
}

#[derive(Deserialize, Debug)]
pub struct ClaimDefData {
    pub primary: serde_json::Value,
    pub revocation: Option<serde_json::Value>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use commands::wallet::tests::{create_and_open_wallet, close_and_delete_wallet};
    use commands::pool::tests::{create_and_connect_pool, disconnect_and_delete_pool};
    use commands::did::tests::{new_did, use_did, SEED_TRUSTEE, DID_TRUSTEE, SEED_MY1, DID_MY1, VERKEY_MY1, SEED_MY3, DID_MY3, VERKEY_MY3};
    use libindy::ledger::Ledger;
    use libindy::did::Did;

    pub const ATTRIB_RAW_DATA: &'static str = r#"{"endpoint":{"ha":"127.0.0.1:5555"}}"#;
    pub const ATTRIB_HASH_DATA: &'static str = r#"83d907821df1c87db829e96569a11f6fc2e7880acba5e43d07ab786959e13bd3"#;
    pub const ATTRIB_ENC_DATA: &'static str = r#"aa3f41f619aa7e5e6b6d0d"#;

    mod nym {
        use super::*;

        #[test]
        pub fn nym_works() {
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
        }

        #[test]
        pub fn nym_works_for_role() {
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
        }

        #[test]
        pub fn nym_works_for_wrong_role() {
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
        }

        #[test]
        pub fn nym_works_for_no_active_did() {
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
        }

        #[test]
        pub fn nym_works_for_no_opened_wallet() {
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
        }

        #[test]
        pub fn nym_works_for_no_connected_pool() {
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
        }

        #[test]
        pub fn nym_works_for_unknown_submitter() {
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
        }
    }

    mod get_nym {
        use super::*;

        #[test]
        pub fn get_nym_works() {
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
        }

        #[test]
        pub fn get_nym_works_for_unknown_did() {
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
        }

        #[test]
        pub fn get_nym_works_for_unknown_submitter() {
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
        }
    }

    mod attrib {
        use super::*;

        #[test]
        pub fn attrib_works_for_raw_value() {
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
        }

        #[test]
        pub fn attrib_works_for_hash_value() {
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
        }

        #[test]
        pub fn attrib_works_for_enc_value() {
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
        }

        #[test]
        pub fn attrib_works_for_missed_attribute() {
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
        }

        #[test]
        pub fn attrib_works_for_no_active_did() {
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
        }

        #[test]
        pub fn attrib_works_for_unknown_did() {
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
        }

        #[test]
        pub fn attrib_works_for_invalid_endpoint_format() {
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
        }
    }

    mod get_attrib {
        use super::*;

        #[test]
        pub fn get_attrib_works_for_raw_value() {
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
        }

        #[test]
        pub fn get_attrib_works_for_hash_value() {
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
        }

        #[test]
        pub fn get_attrib_works_for_enc_value() {
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
        }

        #[test]
        pub fn get_attrib_works_for_no_active_did() {
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
        }
    }

    mod schema {
        use super::*;

        #[test]
        pub fn schema_works() {
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
        }

        #[test]
        pub fn schema_works_for_missed_required_params() {
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
        }

        #[test]
        pub fn schema_works_unknown_submitter() {
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
        }

        #[test]
        pub fn schema_works_for_no_active_did() {
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
        }
    }

    mod get_schema {
        use super::*;

        #[test]
        pub fn get_schema_works() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = schema_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                params.insert("attr_names", "name,age".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            //TODO avoid assumption aboout previous one test schema::schema successfully passed
            _ensure_schema_added(&ctx, DID_TRUSTEE);
            {
                let cmd = get_schema_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
        }

        #[test]
        pub fn schema_works_for_unknown_schema() {
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
        }

        #[test]
        pub fn schema_works_for_unknown_submitter() {
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
        }

        #[test]
        pub fn schema_works_for_no_active_did() {
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
        }
    }

    mod claim_def {
        use super::*;

        #[test]
        pub fn claim_def_works() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            let did = crate_send_and_use_new_nym(&ctx);
            {
                let cmd = claim_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_no", "1".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("primary", r#"{"n":"1","s":"1","rms":"1","r":{"age":"1","name":"1"},"rctxt":"1","z":"1"}"#.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_claim_def_added(&ctx, &did);
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
        }

        #[test]
        pub fn claim_def_works_for_missed_required_params() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = claim_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_no", "1".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
        }

        #[test]
        pub fn claim_def_works_for_unknown_submitter() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
            {
                let cmd = claim_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_no", "1".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("primary", r#"{"n":"1","s":"1","rms":"1","r":{"age":"1","name":"1"},"rctxt":"1","z":"1"}"#.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
        }

        #[test]
        pub fn claim_def_works_for_no_active_did() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            {
                let cmd = claim_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_no", "1".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("primary", r#"{"n":"1","s":"1","rms":"1","r":{"age":"1","name":"1"},"rctxt":"1","z":"1"}"#.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
        }
    }

    mod get_claim_def {
        use super::*;

        #[test]
        pub fn get_claim_def_works() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = claim_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_no", "1".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("primary", r#"{"n":"1","s":"1","rms":"1","r":{"age":"1","name":"1"},"rctxt":"1","z":"1"}"#.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_claim_def_added(&ctx, DID_TRUSTEE);
            {
                let cmd = get_claim_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_no", "1".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("origin", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
        }

        #[test]
        pub fn get_claim_def_works_for_unknown_claim_def() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = get_claim_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_no", "2".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("origin", DID_MY3.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
        }

        #[test]
        pub fn get_claim_def_works_for_no_active_did() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            {
                let cmd = get_claim_def_command::new();
                let mut params = CommandParams::new();
                params.insert("schema_no", "1".to_string());
                params.insert("signature_type", "CL".to_string());
                params.insert("origin", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
        }
    }

    mod node {
        use super::*;

        #[test]
        pub fn node_works() {
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
        }
    }

    mod pool_config {
        use super::*;

        #[test]
        pub fn pool_config_works() {
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
        }
    }

    mod pool_upgrade {
        use super::*;

        #[test]
        pub fn pool_upgrade_works() {
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
                                            "protocolVersion":1
                                          }"#;

        pub const TXN_FOR_SIGN: &'static str = r#"{
                                                    "reqId":1513241300414292814,
                                                    "identifier":"V4SGRU86Z58d6TV7PBUe6f",
                                                    "operation":{
                                                        "type":"1",
                                                        "dest":"E1XWGvsrVp5ZDif2uDdTAM",
                                                        "verkey":"86F43kmApX7Da5Rcba1vCbYmc7bbauEksGxPKy8PkZyb"
                                                    },
                                                    "protocolVersion":1
                                                  }"#;

        #[test]
        pub fn custom_works() {
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
        }

        #[test]
        pub fn custom_works_for_sign() {
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
        }

        #[test]
        pub fn custom_works_for_missed_txn_field() {
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
        }

        #[test]
        pub fn custom_works_for_invalid_transaction_format() {
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
                                                    "protocolVersion":1
                                                  "#, DID_TRUSTEE));
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
        }

        #[test]
        pub fn custom_works_for_no_opened_pool() {
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
        }


        #[test]
        pub fn custom_works_for_sign_without_active_did() {
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
        }

        #[test]
        pub fn custom_works_for_unknown_submitter_did() {
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

    pub fn crate_send_and_use_new_nym(ctx: &CommandContext) -> String {
        let (wallet_handle, _) = get_opened_wallet(ctx).unwrap();
        new_did(&ctx, SEED_TRUSTEE);
        use_did(&ctx, DID_TRUSTEE);
        let (did, verkey) = Did::new(wallet_handle, "{}").unwrap();
        send_nym(ctx, &did, &verkey, None);
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

    fn _ensure_nym_added(ctx: &CommandContext, did: &str) {
        let request = Ledger::build_get_nym_request(DID_TRUSTEE, did).unwrap();
        _submit_retry(ctx, &request, |response| {
            serde_json::from_str::<Response<ReplyResult<String>>>(&response)
                .and_then(|response| serde_json::from_str::<NymData>(&response.result.unwrap().data))
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
        let data = r#"{"name":"gvt", "version":"1.0"}"#;
        let request = Ledger::build_get_schema_request(DID_TRUSTEE, did, data).unwrap();
        _submit_retry(ctx, &request, |response| {
            serde_json::from_str::<Response<ReplyResult<SchemaData>>>(&response)
        }).unwrap();
    }

    fn _ensure_claim_def_added(ctx: &CommandContext, did: &str) {
        let request = Ledger::build_get_claim_def_txn(DID_TRUSTEE, 1, "CL", did).unwrap();
        _submit_retry(ctx, &request, |response| {
            serde_json::from_str::<Response<ReplyResult<ClaimDefData>>>(response)
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