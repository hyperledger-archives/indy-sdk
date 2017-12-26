use command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;

use libindy::ErrorCode;
use libindy::ledger::Ledger;

use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use std::collections::{HashSet, HashMap};
use std::fmt;

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("ledger", "Ledger management commands"));
}

pub mod nym_command {
    use super::*;

    command!(CommandMetadata::build("nym", "Add NYM to Ledger.")
                .add_param("did", false, "DID of new identity")
                .add_param("verkey", true, "Verification key of new identity")
                .add_param("role", true, "Role of new identity. One of: STEWARD, TRUSTEE, TRUST_ANCHOR, TGB")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX verkey=GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX verkey=GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa")
                .add_example("ledger nym did=VsKV7grR1BUE29mG2Fm2kX role=TRUSTEE")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let verkey = get_opt_str_param("verkey", params).map_err(error_err!())?;
        let role = get_opt_str_param("role", params).map_err(error_err!())?;

        let res = Ledger::build_nym_request(&submitter_did, target_did, verkey, None, role)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let res = match res {
            Ok(_) => Ok(println_succ!("NYM {{\"did\":\"{}\", \"verkey\":\"{:?}\", \"role\":\"{:?}\"}} has been added to Ledger",
                                      target_did, verkey, role)),
            Err(err) => handle_send_command_error(err, &submitter_did, pool_handle, wallet_handle)
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_nym_command {
    use super::*;

    command!(CommandMetadata::build("get-nym", "Get NYM from Ledger.")
                .add_param("did", false, "DID of identity presented in Ledger")
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
            Err(err) => handle_get_command_error(err),
        }?;

        let nym = serde_json::from_str::<Reply<String>>(&response)
            .and_then(|response| serde_json::from_str::<NymData>(&response.result.data));

        let res = match nym {
            Ok(nym) => Ok(println_succ!("Following NYM has been received: {}", nym)),
            Err(_) => Err(println_err!("NYM not found"))
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod attrib_command {
    use super::*;

    command!(CommandMetadata::build("attrib", "Add Attribute to exists NYM.")
                .add_param("did", false, "DID of identity presented in Ledger")
                .add_param("hash", true, "Hash of attribute data")
                .add_param("raw", true, "JSON representation of attribute data")
                .add_param("enc", true, "Encrypted attribute data")
                .add_example(r#"ledger attrib did=VsKV7grR1BUE29mG2Fm2kX raw={"endpoint":{"ha":"127.0.0.1:5555"}}"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let hash = get_opt_str_param("hash", params).map_err(error_err!())?;
        let raw = get_opt_str_param("raw", params).map_err(error_err!())?;
        let enc = get_opt_str_param("enc", params).map_err(error_err!())?;

        let res = Ledger::build_attrib_request(&submitter_did, target_did, hash, raw, enc)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let attribute = raw.unwrap_or(hash.unwrap_or(enc.unwrap_or("")));

        let res = match res {
            Ok(_) => Ok(println_succ!("Attribute \"{}\" has been added to Ledger", attribute)),
            Err(err) => handle_send_command_error(err, &submitter_did, pool_handle, wallet_handle)
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_attrib_command {
    use super::*;

    command!(CommandMetadata::build("get-attrib", "Get ATTRIB from Ledger.")
                .add_param("did", false, "DID of identity presented in Ledger")
                .add_param("attr", false, "Name of attribute")
                .add_example("ledger get-attrib did=VsKV7grR1BUE29mG2Fm2kX attr=endpoint")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let attr = get_str_param("attr", params).map_err(error_err!())?;

        let res = Ledger::build_get_attrib_request(&submitter_did, target_did, attr)
            .and_then(|request| Ledger::submit_request(pool_handle, &request));

        let response = match res {
            Ok(response) => Ok(response),
            Err(err) => handle_get_command_error(err),
        }?;

        let attrib = serde_json::from_str::<Reply<String>>(&response)
            .and_then(|response| serde_json::from_str::<AttribData>(&response.result.data));

        let res = match attrib {
            Ok(nym) => Ok(println_succ!("Following Attribute has been received: {}", nym)),
            Err(_) => Err(println_err!("Attribute not found"))
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod schema_command {
    use super::*;

    command!(CommandMetadata::build("schema", "Add Schema to Ledger.")
                .add_param("name", false, "Schema name")
                .add_param("version", false, "Schema version")
                .add_param("attr_names", false, "Schema attributes split by comma")
                .add_example("ledger schema name=gvt version=1.0 attr_names=name,age")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

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

        let res = Ledger::build_schema_request(&submitter_did, &schema_data)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let res = match res {
            Ok(_) => Ok(println_succ!("Schema {{name: \"{}\" version: \"{}\"}}  has been added to Ledger", name, version)),
            Err(err) => handle_send_command_error(err, &submitter_did, pool_handle, wallet_handle)
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_schema_command {
    use super::*;

    command!(CommandMetadata::build("get-schema", "Get Schema from Ledger.")
                .add_param("did", false, "DID of identity presented in Ledger")
                .add_param("name", false, "Schema name")
                .add_param("version", false, "Schema version")
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
            Err(err) => handle_get_command_error(err),
        }?;

        let res = match serde_json::from_str::<Reply<SchemaData>>(&response) {
            Ok(schema) => Ok(println_succ!("Following Schema has been received: {}", schema)),
            Err(_) => Err(println_err!("Schema not found"))
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod claim_def_command {
    use super::*;

    command!(CommandMetadata::build("claim-def", "Add claim definition to Ledger.")
                .add_param("schema_no", false, "Sequence number of schema")
                .add_param("signature_type", false, "Signature type (only CL supported now)")
                .add_param("primary", false, "Primary key in json format")
                .add_param("revocation", true, "Revocation key in json format")
                .add_example(r#"ledger claim-def schema_no=1 signature_type=CL primary={"n":"1","s":"2","rms":"3","r":{"age":"4","name":"5"},"rctxt":"6","z":"7"}"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

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

        let res = Ledger::build_claim_def_txn(&submitter_did, xref, signature_type, &claim_def_data)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let res = match res {
            Ok(_) => Ok(println_succ!("Claim definition {{\"identifier\":\"{}\", \"schema_seq_no\":{}, \"signature_type\":{}}} has been added to Ledger",
                                      submitter_did, xref, signature_type)),
            Err(err) => handle_send_command_error(err, &submitter_did, pool_handle, wallet_handle)
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_claim_def_command {
    use super::*;

    command!(CommandMetadata::build("get-claim-def", "Add claim definition to Ledger.")
                .add_param("schema_no", false, "Sequence number of schema")
                .add_param("signature_type", false, "Signature type (only CL supported now)")
                .add_param("origin", false, "Claim definition owner DID")
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
            Err(err) => handle_get_command_error(err),
        }?;

        let res = match serde_json::from_str::<Reply<ClaimDefData>>(&response) {
            Ok(claim_def) => Ok(println_succ!("Following Claim Definition has been received: {}", claim_def.result.data)),
            Err(_led) => Err(println_err!("Claim definition not found"))
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod node_command {
    use super::*;

    command!(CommandMetadata::build("node", "Add Node to Ledger.")
                .add_param("target", false, "DID of new identity")
                .add_param("alias", false, "Node alias")
                .add_param("node_ip", true, "Node Ip")
                .add_param("node_port", true, "Node port")
                .add_param("client_ip", true, "Client Ip")
                .add_param("client_port", true, "Client port")
                .add_param("blskey", true, "Node BLS key")
                .add_param("services", true, "Node type [VALIDATOR, OBSERVER]")
                .add_example("ledger node target=A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y node_ip=127.0.0.1 node_port=9710 client_ip=127.0.0.1 client_port=9711 alias=Node5 services=VALIDATOR blskey=2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw")
                .add_example("ledger node target=A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y node_ip=127.0.0.1 node_port=9710 client_ip=127.0.0.1 client_port=9711 alias=Node5 services=VALIDATOR")
                .add_example("ledger node target=A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y alias=Node5 services=VALIDATOR")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

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

        let res = Ledger::build_node_request(&submitter_did, target_did, &node_data)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let res = match res {
            Ok(_) => Ok(println_succ!("Node \"{}\" has been added to Ledger", node_data)),
            Err(err) => handle_send_command_error(err, &submitter_did, pool_handle, wallet_handle)
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod pool_config_command {
    use super::*;

    command!(CommandMetadata::build("pool-config", "Sends write configuration to pool.")
                .add_param("writes", false, "Accept write transactions.")
                .add_param("force", true, "Forced configuration applying without reaching pool consensus.")
                .add_example("ledger pool-config writes=true")
                .add_example("ledger pool-config writes=true force=true")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let writes = get_bool_param("writes", params).map_err(error_err!())?;
        let force = get_opt_bool_param("force", params).map_err(error_err!())?.unwrap_or(false);

        let res = Ledger::indy_build_pool_config_request(&submitter_did, writes, force)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let res = match res {
            Ok(_) => Ok(println_succ!("Pool configuration {{\"writes\":\"{}\"}} has been set.", writes)),
            Err(err) => handle_send_command_error(err, &submitter_did, pool_handle, wallet_handle)
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod pool_upgrade_command {
    use super::*;

    command!(CommandMetadata::build("pool-upgrade", "Sends instructions to nodes to update themselves.")
                .add_param("name", false, "Unique upgrade name.")
                .add_param("version", false, "Target upgrade version.")
                .add_param("action", false, "Upgrade type. Either start or cancel.")
                .add_param("sha256", false, "Unique hex identifier.")
                .add_param("timeout", true, "Timeout.")
                .add_param("schedule", true, "Node upgrade schedule.")
                .add_param("justification", true, "Comment.")
                .add_param("reinstall", true, "Same version reinstallation.")
                .add_param("force", true, "Forced upgrade applying without reaching pool consensus")
                .add_example(r#"ledger pool-upgrade name=upgrade-1 version=2.0 action=start sha256=f284bdc3c1c9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398 schedule={"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv":"2020-01-25T12:49:05.258870+00:00"}"#)
                .add_example(r#"ledger pool-upgrade name=upgrade-1 version=2.0 action=cancel sha256=ac3eb2cc3ac9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let name = get_str_param("name", params).map_err(error_err!())?;
        let version = get_str_param("version", params).map_err(error_err!())?;
        let action = get_str_param("action", params).map_err(error_err!())?;
        let sha256 = get_str_param("sha256", params).map_err(error_err!())?;
        let timeout = get_opt_number_param::<u32>("timeout", params).map_err(error_err!())?;
        let schedule = get_opt_str_param("schedule", params).map_err(error_err!())?;
        let justification = get_opt_str_param("justification", params).map_err(error_err!())?;
        let reinstall = get_opt_bool_param("reinstall", params).map_err(error_err!())?.unwrap_or(false);
        let force = get_opt_bool_param("force", params).map_err(error_err!())?.unwrap_or(false);

        let res = Ledger::indy_build_pool_upgrade_request(&submitter_did, name, version, action, sha256,
                                                          timeout, schedule, justification, reinstall, force)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let res = match res {
            Ok(_) => Ok(println_succ!("Pool upgrade instruction {{\"name\":\"{}\", \"version\":\"{}\", \"action\":\"{}\",\
             \"sha256\":\"{}\", \"timeout\":{:?}, \"schedule\":{:?}, \"justification\":{:?}, \"reinstall\":{}, \"force\":{}}} has been sent.",
              name, version, action, sha256, timeout, schedule, justification, reinstall, force)),
            Err(err) => handle_send_command_error(err, &submitter_did, pool_handle, wallet_handle)
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod custom_command {
    use super::*;

    command!(CommandMetadata::build("custom", "Send custom transaction to Ledger.")
                .add_main_param("txn", "Transaction json")
                .add_param("sign", true, "Is signature required")
                .add_example(r#"ledger custom {"reqId":1,"identifier":"V4SGRU86Z58d6TV7PBUe6f","operation":{"type":"105","dest":"V4SGRU86Z58d6TV7PBUe6f"},"protocolVersion":1}"#)
                .add_example(r#"ledger custom {"reqId":2,"identifier":"V4SGRU86Z58d6TV7PBUe6f","operation":{"type":"1","dest":"VsKV7grR1BUE29mG2Fm2kX"},"protocolVersion":1} sign=true"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let pool_handle = ensure_connected_pool_handle(&ctx)?;

        let txn = get_str_param("txn", params).map_err(error_err!())?;
        let sign = get_opt_bool_param("sign", params).map_err(error_err!())?.unwrap_or(false);

        let res = if sign {
            let submitter_did = ensure_active_did(&ctx)?;
            let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

            Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, txn)
        } else {
            Ledger::submit_request(pool_handle, txn)
        };

        let res = match res {
            Ok(response) => Ok(println_succ!("Response: {}", response)),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid transaction \"{}\"", txn)),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("There is no active did")),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

fn handle_send_command_error(err: ErrorCode, submitter_did: &str, pool_handle: i32, wallet_handle: i32) -> Result<(), ()> {
    match err {
        ErrorCode::CommonInvalidStructure => Err(println_err!("Wrong command params")),
        ErrorCode::WalletNotFoundError => Err(println_err!("Submitter DID: \"{}\" not found", submitter_did)),
        ErrorCode::LedgerInvalidTransaction => Err(println_err!("Invalid transaction")),
        ErrorCode::WalletIncompatiblePoolError => Err(println_err!("Pool handle \"{}\" invalid for wallet handle \"{}\"", pool_handle, wallet_handle)),
        err => Err(println_err!("Indy SDK error occurred {:?}", err))
    }
}

fn handle_get_command_error(err: ErrorCode) -> Result<String, ()> {
    match err {
        ErrorCode::CommonInvalidStructure => Err(println_err!("Wrong command params")),
        ErrorCode::LedgerInvalidTransaction => Err(println_err!("Invalid transaction")),
        err => Err(println_err!("Indy SDK error occurred {:?}", err)),
    }
}

#[derive(Deserialize, Debug)]
pub struct Reply<T> {
    pub result: ReplyResult<T>,
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

impl fmt::Display for NymData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let role = match self.role.as_ref().map(String::as_str) {
            Some("0") => "TRUSTEE",
            Some("2") => "STEWARD",
            Some("100") => "TGB",
            Some("101") => "TRUST_ANCHOR",
            _ => "null"
        };

        write!(f, "\nsubmitter:{} | did:{} | verkey:{} | role:{} ",
               self.identifier.clone().unwrap_or("null".to_string()), self.dest,
               self.verkey.clone().unwrap_or("null".to_string()), role)
    }
}

#[derive(Deserialize, Debug)]
pub struct AttribData {
    pub endpoint: Option<Endpoint>,
}

impl fmt::Display for AttribData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref endpoint) = self.endpoint {
            write!(f, "\n{:?}", endpoint)?;
        }
        write!(f, "")
    }
}

#[derive(Deserialize, Debug)]
pub struct Endpoint {
    pub ha: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SchemaData {
    pub attr_names: HashSet<String>,
    pub name: String,
    pub version: String
}

impl fmt::Display for Reply<SchemaData> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nname:{} | version:{} | attr_names:{:?} | origin:{:?} | seq_no:{:?}",
               self.result.data.name, self.result.data.version, self.result.data.attr_names, self.result.identifier, self.result.seq_no)
    }
}

#[derive(Deserialize, Debug)]
pub struct ClaimDefData {
    pub primary: serde_json::Value,
    pub revocation: Option<serde_json::Value>,
}

impl fmt::Display for ClaimDefData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Primary key: {{"n":"{}","s":"{}","rms":"{}","rctxt":"{}","z":"{}","r":{:?}}}"#,
               self.primary["n"].as_str().unwrap_or(""),
               self.primary["s"].as_str().unwrap_or(""),
               self.primary["rms"].as_str().unwrap_or(""),
               self.primary["rctxt"].as_str().unwrap_or(""),
               self.primary["z"].as_str().unwrap_or(""),
               self.primary["r"].as_object().unwrap().iter()
                   .map(|(key, value)| (key.clone(), value.as_str().unwrap_or("").to_string())).collect::<HashMap<String, String>>())?;

        if let Some(ref revocation) = self.revocation {
            if revocation.as_object().unwrap().len() > 0 {
                write!(f, "\n")?;
                write!(f, r#"Revocation key: {{"g":"{}","g_dash":"{}","h":"{}","h0":"{}","h1":"{}","h2":"{}","htilde":"{}","h_cap":"{}","u":"{}","pk":"{}","y":"{}"}}"#,
                       revocation["g"].as_str().unwrap_or(""),
                       revocation["g_dash"].as_str().unwrap_or(""),
                       revocation["h"].as_str().unwrap_or(""),
                       revocation["h0"].as_str().unwrap_or(""),
                       revocation["h1"].as_str().unwrap_or(""),
                       revocation["h2"].as_str().unwrap_or(""),
                       revocation["htilde"].as_str().unwrap_or(""),
                       revocation["h_cap"].as_str().unwrap_or(""),
                       revocation["u"].as_str().unwrap_or(""),
                       revocation["pk"].as_str().unwrap_or(""),
                       revocation["y"].as_str().unwrap_or(""))?;
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use commands::wallet::tests::{create_and_open_wallet, close_and_delete_wallet};
    use commands::pool::tests::{create_and_connect_pool, disconnect_and_delete_pool};
    use commands::did::tests::{new_did, use_did, SEED_TRUSTEE, DID_TRUSTEE, SEED_MY1, DID_MY1, VERKEY_MY1, SEED_MY3, DID_MY3};
    use libindy::ledger::Ledger;

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
            _ensure_nym_added(&ctx);
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
            _ensure_nym_added(&ctx);
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
                params.insert("did", DID_MY1.to_string());
                params.insert("verkey", VERKEY_MY1.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            _ensure_nym_added(&ctx);
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

            new_did(&ctx, SEED_MY3);
            use_did(&ctx, DID_MY3);
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
    }

    mod attrib {
        use super::*;

        #[test]
        pub fn attrib_works() {
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
                params.insert("raw", r#"{"endpoint":{"ha":"127.0.0.1:5555"}}"#.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_attrib_added(&ctx);
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
                params.insert("raw", r#"{"endpoint":{"ha":"127.0.0.1:5555"}}"#.to_string());
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
                params.insert("raw", r#"{"endpoint":{"ha":"127.0.0.1:5555"}}"#.to_string());
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
        pub fn get_attrib_works() {
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
                params.insert("raw", r#"{"endpoint":{"ha":"127.0.0.1:5555"}}"#.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            {
                let cmd = get_attrib_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("attr", "endpoint".to_string());
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
            _ensure_schema_added(&ctx);
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
        pub fn schema_works() {
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
            {
                let cmd = get_schema_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                params.insert("name", "gvt".to_string());
                params.insert("version", "1.0".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            _ensure_schema_added(&ctx);
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
            _ensure_claim_def_added(&ctx);
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
                                                        "dest":"VsKV7grR1BUE29mG2Fm2kX",
                                                        "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
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
        pub fn custom_works_for_invalid_transaction() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);
            {
                let cmd = custom_command::new();
                let mut params = CommandParams::new();
                params.insert("txn", format!(r#"{{
                                                    "reqId":1513241300414292814,
                                                    "identifier":"{}",
                                                    "protocolVersion":1
                                                  }}"#, DID_TRUSTEE));
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

    fn _ensure_nym_added(ctx: &CommandContext) {
        let request = Ledger::build_get_nym_request(DID_TRUSTEE, DID_MY1).unwrap();
        let pool_handle = ensure_connected_pool_handle(&ctx).unwrap();
        let response = Ledger::submit_request(pool_handle, &request).unwrap();
        serde_json::from_str::<Reply<String>>(&response)
            .and_then(|response| serde_json::from_str::<NymData>(&response.result.data)).unwrap();
    }

    fn _ensure_attrib_added(ctx: &CommandContext) {
        let request = Ledger::build_get_attrib_request(DID_MY1, DID_MY1, "endpoint").unwrap();
        let pool_handle = ensure_connected_pool_handle(&ctx).unwrap();
        let response = Ledger::submit_request(pool_handle, &request).unwrap();
        serde_json::from_str::<Reply<String>>(&response)
            .and_then(|response| serde_json::from_str::<AttribData>(&response.result.data)).unwrap();
    }

    fn _ensure_schema_added(ctx: &CommandContext) {
        let data = r#"{"name":"gvt", "version":"1.0"}"#;
        let request = Ledger::build_get_schema_request(DID_TRUSTEE, DID_TRUSTEE, data).unwrap();
        let pool_handle = ensure_connected_pool_handle(&ctx).unwrap();
        let response = Ledger::submit_request(pool_handle, &request).unwrap();
        serde_json::from_str::<Reply<SchemaData>>(&response).unwrap();
    }

    fn _ensure_claim_def_added(ctx: &CommandContext) {
        let request = Ledger::build_get_claim_def_txn(DID_TRUSTEE, 1, "CL", DID_TRUSTEE).unwrap();
        let pool_handle = ensure_connected_pool_handle(&ctx).unwrap();
        let response = Ledger::submit_request(pool_handle, &request).unwrap();
        serde_json::from_str::<Reply<ClaimDefData>>(&response).unwrap();
    }
}