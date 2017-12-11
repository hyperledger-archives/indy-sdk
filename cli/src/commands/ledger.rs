extern crate serde_json;

use IndyContext;
use command_executor::{Command, CommandMetadata, Group as GroupTrait, GroupMetadata};
use commands::*;

use libindy::ErrorCode;
use libindy::ledger::Ledger;

use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

pub struct Group {
    metadata: GroupMetadata
}

impl Group {
    pub fn new() -> Group {
        Group {
            metadata: GroupMetadata::new("ledger", "Ledger management commands")
        }
    }
}

impl GroupTrait for Group {
    fn metadata(&self) -> &GroupMetadata {
        &self.metadata
    }
}

pub mod NymCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("nym", "Add NYM to Ledger.")
                .add_param("did", false, "DID of new identity")
                .add_param("verkey", true, "Verification key of new identity")
                .add_param("alias", true, "Alias of new identity")
                .add_param("role", true, "Role of new identity. One of: STEWARD, TRUSTEE, TRUST_ANCHOR, TGB")
                .finalize()
    );

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SendNymCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = get_active_did(&ctx)?;
        let pool_handle = get_connected_pool_handle(&ctx)?;
        let wallet_handle = get_opened_wallet_handle(&ctx)?;

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let verkey = get_opt_str_param("verkey", params).map_err(error_err!())?;
        let alias = get_opt_str_param("alias", params).map_err(error_err!())?;
        let role = get_opt_str_param("role", params).map_err(error_err!())?;

        let res = Ledger::build_nym_request(&submitter_did, target_did, verkey, alias, role)
            .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request));

        let res = match res {
            Ok(_) => Ok(println_succ!("NYM {{\"did\":\"{}\", \"verkey\":\"{:?}\", \"alias\":\"{:?}\", \"role\":\"{:?}\"}} has been added to Ledger",
                                      target_did, verkey, alias, role)),
            Err(err) => handle_send_command_error(err, &submitter_did, pool_handle, wallet_handle)
        };

        trace!("SendNymCommand::execute << {:?}", res);
        Ok(())
    }
}

pub mod GetNymCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("get-nym", "Get NYM from Ledger.")
                .add_param("did", false, "DID of identity presented in Ledger")
                .finalize()
    );

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("GetNymCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = get_active_did(&ctx)?;
        let pool_handle = get_connected_pool_handle(&ctx)?;

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

        trace!("SendNymCommand::execute << {:?}", res);
        Ok(())
    }
}

pub mod AttribCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("attrib", "Add Attribute to exists NYM.")
                .add_param("did", false, "DID of identity presented in Ledger")
                .add_param("hash", true, "Hash of attribute data")
                .add_param("raw", true, "JSON representation of attribute data")
                .add_param("enc", true, "Encrypted attribute data")
                .finalize()
    );

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SendAttribCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = get_active_did(&ctx)?;
        let pool_handle = get_connected_pool_handle(&ctx)?;
        let wallet_handle = get_opened_wallet_handle(&ctx)?;

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

        trace!("SendAttribCommand::execute << {:?}", res);
        Ok(())
    }
}

pub mod GetAttribCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("get-attrib", "Get ATTRIB from Ledger.")
                .add_param("did", false, "DID of identity presented in Ledger")
                .add_param("attr", false, "Name of attribute")
                .finalize()
    );

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("GetAttribCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = get_active_did(&ctx)?;
        let pool_handle = get_connected_pool_handle(&ctx)?;

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

        trace!("GetAttribCommand::execute << {:?}", res);
        Ok(())
    }
}

pub mod SchemaCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("schema", "Add Schema to Ledger.")
                .add_param("name", false, "Schema name")
                .add_param("version", false, "Schema version")
                .add_param("attr_names", false, "Schema attributes split by comma")
                .finalize()
    );

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SendSchemaCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = get_active_did(&ctx)?;
        let pool_handle = get_connected_pool_handle(&ctx)?;
        let wallet_handle = get_opened_wallet_handle(&ctx)?;

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

        trace!("SendSchemaCommand::execute << {:?}", res);
        Ok(())
    }
}

pub mod GetSchemaCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("get-schema", "Get Schema from Ledger.")
                .add_param("did", false, "DID of identity presented in Ledger")
                .add_param("name", false, "Schema name")
                .add_param("version", false, "Schema version")
                .finalize()
    );

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("GetSchemaCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = get_active_did(&ctx)?;
        let pool_handle = get_connected_pool_handle(&ctx)?;

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
            Ok(schema) => Ok(println_succ!("Following Schema has been received: \"{}\"", schema.result.data)),
            Err(_) => Err(println_err!("Schema not found"))
        };

        trace!("GetSchemaCommand::execute << {:?}", res);
        Ok(())
    }
}

pub mod ClaimDefCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("claim-def", "Add claim definition to Ledger.")
                .add_param("schema_no", false, "Sequence number of schema")
                .add_param("signature_type", false, "Signature type (only CL supported now)")
                .add_param("primary", false, "Primary key in json format")
                .add_param("revocation", true, "Revocation key in json format")
                .finalize()
    );

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SendClaimDefCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = get_active_did(&ctx)?;
        let pool_handle = get_connected_pool_handle(&ctx)?;
        let wallet_handle = get_opened_wallet_handle(&ctx)?;

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
            Ok(_) => Ok(println_succ!("Claim def {{\"origin\":\"{}\", \"schema_seq_no\":{}, \"signature_type\":{}}} has been added to Ledger",
                                      submitter_did, xref, signature_type)),
            Err(err) => handle_send_command_error(err, &submitter_did, pool_handle, wallet_handle)
        };

        trace!("SendClaimDefCommand::execute << {:?}", res);
        Ok(())
    }
}

pub mod GetClaimDefCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("get-claim-def", "Add claim definition to Ledger.")
                .add_param("schema_no", false, "Sequence number of schema")
                .add_param("signature_type", false, "Signature type (only CL supported now)")
                .add_param("origin", false, "Claim definition owner DID")
                .finalize()
    );

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("GetClaimDefCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = get_active_did(&ctx)?;
        let pool_handle = get_connected_pool_handle(&ctx)?;

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
            Ok(claim_def) => Ok(println_succ!("Following ClaimDef has been received: \"{:?}\"", claim_def.result.data)),
            Err(_led) => Err(println_err!("Claim definition not found"))
        };

        trace!("GetClaimDefCommand::execute << {:?}", res);
        Ok(())
    }
}

pub mod NodeCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("node", "Add Node to Ledger.")
                .add_param("target", false, "DID of new identity")
                .add_param("node_ip", false, "Node Ip")
                .add_param("node_port", false, "Node port")
                .add_param("client_ip", false, "Client Ip")
                .add_param("client_port", false, "Client port")
                .add_param("alias", false, "Node alias")
                .add_param("blskey", false, "Node BLS key")
                .add_param("services", true, "Node type [VALIDATOR, OBSERVER]")
                .finalize()
    );

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SendNodeCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let submitter_did = get_active_did(&ctx)?;
        let pool_handle = get_connected_pool_handle(&ctx)?;
        let wallet_handle = get_opened_wallet_handle(&ctx)?;

        let target_did = get_str_param("target", params).map_err(error_err!())?;
        let node_ip = get_opt_str_param("node_ip", params).map_err(error_err!())?;
        let node_port = get_opt_int_param::<i32>("node_port", params).map_err(error_err!())?;
        let client_ip = get_opt_str_param("client_ip", params).map_err(error_err!())?;
        let client_port = get_opt_int_param::<i32>("client_port", params).map_err(error_err!())?;
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

        trace!("SendNodeCommand::execute << {:?}", res);
        Ok(())
    }
}

pub mod CustomCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("custom", "Send custom transaction to Ledger.")
                .add_main_param("txn", "Transaction json")
                .add_param("sign", true, "Is signature required")
                .finalize()
    );

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SenCustomCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let pool_handle = get_connected_pool_handle(&ctx)?;

        let txn = get_str_param("txn", params).map_err(error_err!())?;
        let sign = get_opt_bool_param("sign", params).map_err(error_err!())?.unwrap_or(false);

        let res = if sign {
            let submitter_did = get_active_did(&ctx)?;
            let wallet_handle = get_opened_wallet_handle(&ctx)?;

            Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, txn)
        } else {
            Ledger::submit_request(pool_handle, txn)
        };

        let res = match res {
            Ok(_) => Ok(println_succ!("Transaction has been sent to Ledger")),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid transaction \"{}\"", txn)),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("There is no active did")),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("SenCustomCommand::execute << {:?}", res);
        Ok(())
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
    pub data: T
}

#[derive(Deserialize, Debug)]
pub struct NymData {
    pub identifier: Option<String>,
    pub dest: String,
    pub role: Option<String>,
    pub alias: Option<String>,
    pub verkey: Option<String>
}

impl fmt::Display for NymData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nsubmitter:{} | did:{} | role:{} | alias:{} | verkey:{}",
               self.identifier.clone().unwrap_or("null".to_string()), self.dest,
               self.role.clone().unwrap_or("null".to_string()),
               self.alias.clone().unwrap_or("null".to_string()),
               self.verkey.clone().unwrap_or("null".to_string()))
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
    pub origin: Option<String>,
    pub version: String
}

impl fmt::Display for SchemaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nname:{} | version:{} | attr_names:{:?} | origin:{}",
               self.name, self.version, self.attr_names, self.origin.clone().unwrap_or("null".to_string()))
    }
}

#[derive(Deserialize, Debug)]
pub struct ClaimDefData {
    pub primary: serde_json::Value,
    pub revocation: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {}