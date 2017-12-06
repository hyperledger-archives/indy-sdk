extern crate serde_json;

use IndyContext;
use command_executor::{Command, CommandMetadata, Group as GroupTrait, GroupMetadata};
use commands::*;

use libindy::ErrorCode;
use libindy::ledger::Ledger;

use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use std::collections::{HashMap, HashSet};
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

#[derive(Debug)]
pub struct SendNymCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct GetNymCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct SendAttribCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct GetAttribCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct SendSchemaCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct GetSchemaCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct SendClaimDefCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct GetClaimDefCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct SendNodeCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct SenCustomCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

impl SendNymCommand {
    pub fn new(ctx: Rc<IndyContext>) -> SendNymCommand {
        SendNymCommand {
            ctx,
            metadata: CommandMetadata::build("send-nym", "Add NYM to Ledger.")
                .add_param("did", false, "DID of new identity")
                .add_param("verkey", true, "Verification key of new identity")
                .add_param("alias", true, "Alias of new identity")
                .add_param("role", true, "Role of new identity. One of: STEWARD, TRUSTEE, TRUST_ANCHOR, TGB")
                .finalize()
        }
    }
}

impl Command for SendNymCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SendNymCommand::execute >> self {:?} params {:?}", self, params);

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let verkey = get_opt_str_param("verkey", params).map_err(error_err!())?;
        let alias = get_opt_str_param("alias", params).map_err(error_err!())?;
        let role = get_opt_str_param("role", params).map_err(error_err!())?;

        let submitter_did = get_active_did(&self.ctx)?;
        let pool_handle = get_connected_pool_handle(&self.ctx)?;
        let wallet_handle = get_opened_wallet_handle(&self.ctx)?;

        let request = match Ledger::build_nym_request(&submitter_did, target_did, verkey, alias, role) {
            Ok(request) => Ok(request),
            Err(_) => return Err(println_err!("Wrong command params")),
        }?;

        let res = match Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request) {
            Ok(_) => Ok(println_succ!("NYM with did: \"{}\" has been added to Ledger", target_did)),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Submitter DID: \"{}\" not found", submitter_did)),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid NYM transaction \"{}\"", request)),
            Err(ErrorCode::WalletIncompatiblePoolError) => Err(println_err!("Pool handle \"{}\" invalid for wallet handle \"{}\"", pool_handle, wallet_handle)),
            Err(err) => Err(println_err!("Send NYM request failed with unexpected Indy SDK error {:?}", err)),
        };

        trace!("SendNymCommand::execute << {:?}", res);
        Ok(())
    }
}

impl GetNymCommand {
    pub fn new(ctx: Rc<IndyContext>) -> GetNymCommand {
        GetNymCommand {
            ctx,
            metadata: CommandMetadata::build("get-nym", "Get NYM from Ledger.")
                .add_param("did", false, "DID of identity presented in Ledger")
                .finalize()
        }
    }
}

impl Command for GetNymCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("GetNymCommand::execute >> self {:?} params {:?}", self, params);

        let target_did = get_str_param("did", params).map_err(error_err!())?;

        let submitter_did = get_active_did(&self.ctx)?;
        let pool_handle = get_connected_pool_handle(&self.ctx)?;

        let request = match Ledger::build_get_nym_request(&submitter_did, target_did) {
            Ok(request) => Ok(request),
            Err(_) => return Err(println_err!("Wrong command params")),
        }?;

        let response = match Ledger::submit_request(pool_handle, &request) {
            Ok(response) => Ok(response),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid Get NYM transaction \"{}\"", request)),
            Err(err) => Err(println_err!("Get NYM failed with unexpected Indy SDK error {:?}", err)),
        }?;

        let res = match serde_json::from_str::<Reply<String>>(&response) {
            Ok(nym) => Ok(println_succ!("Following NYM has been received: \"{}\"", nym.result.data)),
            Err(_) => Err(println_err!("NYM not found"))
        };

        trace!("SendNymCommand::execute << {:?}", res);
        Ok(())
    }
}

impl SendAttribCommand {
    pub fn new(ctx: Rc<IndyContext>) -> SendAttribCommand {
        SendAttribCommand {
            ctx,
            metadata: CommandMetadata::build("send-attrib", "Add Attribute to exists NYM.")
                .add_param("did", false, "DID of identity presented in Ledger")
                .add_param("hash", true, "Hash of attribute data")
                .add_param("raw", true, "JSON representation of attribute data")
                .add_param("enc", true, "Encrypted attribute data")
                .finalize()
        }
    }
}

impl Command for SendAttribCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SendAttribCommand::execute >> self {:?} params {:?}", self, params);

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let hash = get_opt_str_param("hash", params).map_err(error_err!())?;
        let raw = get_opt_str_param("raw", params).map_err(error_err!())?;
        let enc = get_opt_str_param("enc", params).map_err(error_err!())?;

        let submitter_did = get_active_did(&self.ctx)?;
        let pool_handle = get_connected_pool_handle(&self.ctx)?;
        let wallet_handle = get_opened_wallet_handle(&self.ctx)?;

        let request = match Ledger::build_attrib_request(&submitter_did, target_did, hash, raw, enc) {
            Ok(request) => Ok(request),
            Err(_) => return Err(println_err!("Wrong command params")),
        }?;

        let res = match Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request) {
            Ok(_) => Ok(println_succ!("Attribute for did: \"{}\" has been added to Ledger", target_did)),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Submitter DID: \"{}\" not found", submitter_did)),
            Err(ErrorCode::WalletIncompatiblePoolError) => Err(println_err!("Pool handle \"{}\" invalid for wallet handle \"{}\"", pool_handle, wallet_handle)),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid ATTRIB transaction \"{}\"", request)),
            Err(err) => Err(println_err!("Send ATTRIB request failed with unexpected Indy SDK error {:?}", err)),
        };

        trace!("SendAttribCommand::execute << {:?}", res);
        Ok(())
    }
}

impl GetAttribCommand {
    pub fn new(ctx: Rc<IndyContext>) -> GetAttribCommand {
        GetAttribCommand {
            ctx,
            metadata: CommandMetadata::build("get-attrib", "Get ATTRIB from Ledger.")
                .add_param("did", false, "DID of identity presented in Ledger")
                .add_param("attr", false, "Name of attribute")
                .finalize()
        }
    }
}

impl Command for GetAttribCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("GetAttribCommand::execute >> self {:?} params {:?}", self, params);

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let attr = get_str_param("attr", params).map_err(error_err!())?;

        let submitter_did = get_active_did(&self.ctx)?;
        let pool_handle = get_connected_pool_handle(&self.ctx)?;

        let request = match Ledger::build_get_attrib_request(&submitter_did, target_did, attr) {
            Ok(request) => Ok(request),
            Err(_) => return Err(println_err!("Wrong command params")),
        }?;

        let response = match Ledger::submit_request(pool_handle, &request) {
            Ok(response) => Ok(response),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid Get ATTRIB transaction \"{}\"", request)),
            Err(err) => Err(println_err!("Get ATTRIB failed with unexpected Indy SDK error {:?}", err)),
        }?;

        let res = match serde_json::from_str::<Reply<String>>(&response) {
            Ok(attrib) => Ok(println_succ!("Following ATTRIB has been received: \"{}\"", attrib.result.data)),
            Err(_) => Err(println_err!("Attribute not found"))
        };

        trace!("GetAttribCommand::execute << {:?}", res);
        Ok(())
    }
}

impl SendSchemaCommand {
    pub fn new(ctx: Rc<IndyContext>) -> SendSchemaCommand {
        SendSchemaCommand {
            ctx,
            metadata: CommandMetadata::build("send-schema", "Add Schema to Ledger.")
                .add_param("name", false, "Schema name")
                .add_param("version", false, "Schema version")
                .add_param("attr_names", false, "Schema attributes split by comma")
                .finalize()
        }
    }
}

impl Command for SendSchemaCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SendSchemaCommand::execute >> self {:?} params {:?}", self, params);

        let name = get_str_param("name", params).map_err(error_err!())?;
        let version = get_str_param("version", params).map_err(error_err!())?;
        let attr_names = get_str_array_param("attr_names", params).map_err(error_err!())?;

        let submitter_did = get_active_did(&self.ctx)?;
        let pool_handle = get_connected_pool_handle(&self.ctx)?;
        let wallet_handle = get_opened_wallet_handle(&self.ctx)?;

        let schema_data = {
            let mut json = JSONMap::new();
            json.insert("name".to_string(), JSONValue::from(name));
            json.insert("version".to_string(), JSONValue::from(version));
            json.insert("attr_names".to_string(), JSONValue::from(attr_names));
            JSONValue::from(json).to_string()
        };

        let request = match Ledger::build_schema_request(&submitter_did, &schema_data) {
            Ok(request) => Ok(request),
            Err(_) => return Err(println_err!("Wrong command params")),
        }?;

        let res = match Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request) {
            Ok(_) => Ok(println_succ!("Schema {{name: \"{}\" version: \"{}\"}}  has been added to Ledger", name, version)),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Submitter DID: \"{}\" not found", submitter_did)),
            Err(ErrorCode::WalletIncompatiblePoolError) => Err(println_err!("Pool handle \"{}\" invalid for wallet handle \"{}\"", pool_handle, wallet_handle)),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid Schema transaction \"{}\"", request)),
            Err(err) => Err(println_err!("Send Schema request failed with unexpected Indy SDK error {:?}", err))
        };

        trace!("SendSchemaCommand::execute << {:?}", res);
        Ok(())
    }
}

impl GetSchemaCommand {
    pub fn new(ctx: Rc<IndyContext>) -> GetSchemaCommand {
        GetSchemaCommand {
            ctx,
            metadata: CommandMetadata::build("get-schema", "Get Schema from Ledger.")
                .add_param("did", false, "DID of identity presented in Ledger")
                .add_param("name", false, "Schema name")
                .add_param("version", false, "Schema version")
                .finalize()
        }
    }
}

impl Command for GetSchemaCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("GetSchemaCommand::execute >> self {:?} params {:?}", self, params);

        let target_did = get_str_param("did", params).map_err(error_err!())?;
        let name = get_str_param("name", params).map_err(error_err!())?;
        let version = get_str_param("version", params).map_err(error_err!())?;

        let submitter_did = get_active_did(&self.ctx)?;
        let pool_handle = get_connected_pool_handle(&self.ctx)?;

        let schema_data = {
            let mut json = JSONMap::new();
            json.insert("name".to_string(), JSONValue::from(name));
            json.insert("version".to_string(), JSONValue::from(version));
            JSONValue::from(json).to_string()
        };

        let request = match Ledger::build_get_schema_request(&submitter_did, target_did, &schema_data) {
            Ok(request) => Ok(request),
            Err(_) => return Err(println_err!("Wrong command params")),
        }?;

        let response = match Ledger::submit_request(pool_handle, &request) {
            Ok(response) => Ok(response),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid Get Schema transaction \"{}\"", request)),
            Err(err) => Err(println_err!("Get Schema failed with unexpected Indy SDK error {:?}", err)),
        }?;

        let res = match serde_json::from_str::<Reply<SchemaData>>(&response) {
            Ok(schema) => Ok(println_succ!("Following Schema has been received: \"{:?}\"", schema.result.data)),
            Err(_) => Err(println_err!("Schema not found"))
        };

        trace!("GetSchemaCommand::execute << {:?}", res);
        Ok(())
    }
}

impl SendClaimDefCommand {
    pub fn new(ctx: Rc<IndyContext>) -> SendClaimDefCommand {
        SendClaimDefCommand {
            ctx,
            metadata: CommandMetadata::build("send-claim-def", "Add claim definition to Ledger.")
                .add_param("schema_no", false, "Sequence number of schema")
                .add_param("signature_type", false, "Signature type (only CL supported now)")
                .add_param("primary", false, "Primary key in json format")
                .add_param("revocation", true, "Revocation key in json format")
                .finalize()
        }
    }
}

impl Command for SendClaimDefCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SendClaimDefCommand::execute >> self {:?} params {:?}", self, params);

        let xref = get_i32_param("schema_no", params).map_err(error_err!())?;
        let signature_type = get_str_param("signature_type", params).map_err(error_err!())?;
        let primary = get_str_param("primary", params).map_err(error_err!())?;
        let revocation = get_opt_str_param("revocation", params).map_err(error_err!())?;

        let submitter_did = get_active_did(&self.ctx)?;
        let pool_handle = get_connected_pool_handle(&self.ctx)?;
        let wallet_handle = get_opened_wallet_handle(&self.ctx)?;

        let claim_def_data = {
            let mut json = JSONMap::new();
            json.insert("primary".to_string(), JSONValue::from(primary));

            if let Some(revocation) = revocation {
                json.insert("revocation".to_string(), JSONValue::from(revocation));
            }

            JSONValue::from(json).to_string()
        };

        let request = match Ledger::build_claim_def_txn(&submitter_did, xref, signature_type, &claim_def_data) {
            Ok(request) => Ok(request),
            Err(_) => return Err(println_err!("Wrong command params")),
        }?;

        let res = match Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request) {
            Ok(_) => Ok(println_succ!("Claim def {{\"origin\":\"{}\", \"schema_seq_no\":{}}} has been added to Ledger", submitter_did, xref)),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Submitter DID: \"{}\" not found", submitter_did)),
            Err(ErrorCode::WalletIncompatiblePoolError) => Err(println_err!("Pool handle \"{}\" invalid for wallet handle \"{}\"", pool_handle, wallet_handle)),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid ClaimDef transaction \"{}\"", request)),
            Err(err) => Err(println_err!("Send ClaimDef request failed with unexpected Indy SDK error {:?}", err))
        };

        trace!("SendClaimDefCommand::execute << {:?}", res);
        Ok(())
    }
}

impl GetClaimDefCommand {
    pub fn new(ctx: Rc<IndyContext>) -> GetClaimDefCommand {
        GetClaimDefCommand {
            ctx,
            metadata: CommandMetadata::build("send-claim-def", "Add claim definition to Ledger.")
                .add_param("schema_no", false, "Sequence number of schema")
                .add_param("signature_type", false, "Signature type (only CL supported now)")
                .add_param("origin", false, "Claim definition owner DID")
                .finalize()
        }
    }
}

impl Command for GetClaimDefCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("GetClaimDefCommand::execute >> self {:?} params {:?}", self, params);

        let xref = get_i32_param("schema_no", params).map_err(error_err!())?;
        let signature_type = get_str_param("signature_type", params).map_err(error_err!())?;
        let origin = get_str_param("origin", params).map_err(error_err!())?;

        let submitter_did = get_active_did(&self.ctx)?;
        let pool_handle = get_connected_pool_handle(&self.ctx)?;

        let request = match Ledger::build_get_claim_def_txn(&submitter_did, xref, signature_type, origin) {
            Ok(request) => Ok(request),
            Err(_) => return Err(println_err!("Wrong command params")),
        }?;

        let response = match Ledger::submit_request(pool_handle, &request) {
            Ok(response) => Ok(response),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid Get ClaimDef transaction \"{}\"", request)),
            Err(err) => Err(println_err!("Get ClaimDef failed with unexpected Indy SDK error {:?}", err)),
        }?;

        let res = match serde_json::from_str::<Reply<SchemaData>>(&response) {
            Ok(claim_def) => Ok(println_succ!("Following ClaimDef has been received: \"{:?}\"", claim_def.result.data)),
            Err(_led) => Err(println_err!("Claim definition not found"))
        };

        trace!("GetClaimDefCommand::execute << {:?}", res);
        Ok(())
    }
}

impl SendNodeCommand {
    pub fn new(ctx: Rc<IndyContext>) -> SendNodeCommand {
        SendNodeCommand {
            ctx,
            metadata: CommandMetadata::build("send-node", "Add Node to Ledger.")
                .add_param("target", false, "DID of new identity")
                .add_param("node_ip", false, "Node Ip")
                .add_param("node_port", false, "Node port")
                .add_param("client_ip", false, "Client Ip")
                .add_param("client_port", false, "Client port")
                .add_param("alias", false, "Node alias")
                .add_param("blskey", false, "Node BLS key")
                .add_param("services", true, "Node type [VALIDATOR, OBSERVER]")
                .finalize()
        }
    }
}

impl Command for SendNodeCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SendNodeCommand::execute >> self {:?} params {:?}", self, params);

        let target_did = get_str_param("target", params).map_err(error_err!())?;
        let node_ip = get_str_param("node_ip", params).map_err(error_err!())?;
        let node_port = get_i32_param("node_port", params).map_err(error_err!())?;
        let client_ip = get_str_param("client_ip", params).map_err(error_err!())?;
        let client_port = get_i32_param("client_port", params).map_err(error_err!())?;
        let alias = get_str_param("alias", params).map_err(error_err!())?;
        let blskey = get_str_param("blskey", params).map_err(error_err!())?;
        let services = get_str_array_param("services", params).map_err(error_err!())?;

        let submitter_did = get_active_did(&self.ctx)?;
        let pool_handle = get_connected_pool_handle(&self.ctx)?;
        let wallet_handle = get_opened_wallet_handle(&self.ctx)?;

        let node_data = {
            let mut json = JSONMap::new();
            json.insert("node_ip".to_string(), JSONValue::from(node_ip));
            json.insert("node_port".to_string(), JSONValue::from(node_port));
            json.insert("client_ip".to_string(), JSONValue::from(client_ip));
            json.insert("client_port".to_string(), JSONValue::from(client_port));
            json.insert("alias".to_string(), JSONValue::from(alias));
            json.insert("blskey".to_string(), JSONValue::from(blskey));
            json.insert("services".to_string(), JSONValue::from(services));
            JSONValue::from(json).to_string()
        };

        let request = match Ledger::build_node_request(&submitter_did, target_did, &node_data) {
            Ok(request) => Ok(request),
            Err(_) => return Err(println_err!("Wrong command params")),
        }?;

        let res = match Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, &request) {
            Ok(response) => Ok(response),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Submitter DID: \"{}\" not found", submitter_did)),
            Err(ErrorCode::WalletIncompatiblePoolError) => Err(println_err!("Pool handle \"{}\" invalid for wallet handle \"{}\"", pool_handle, wallet_handle)),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid NODE transaction \"{}\"", request)),
            Err(err) => Err(println_err!("Send NODE request failed with unexpected Indy SDK error {:?}", err))
        };

        trace!("SendNodeCommand::execute << {:?}", res);
        Ok(())
    }
}

impl SenCustomCommand {
    pub fn new(ctx: Rc<IndyContext>) -> SenCustomCommand {
        SenCustomCommand {
            ctx,
            metadata: CommandMetadata::build("send-custom", "Add NYM to Ledger.")
                .add_main_param("txn", "Transaction json")
                .add_param("sign", true, "Is signature required")
                .finalize()
        }
    }
}

impl Command for SenCustomCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("SenCustomCommand::execute >> self {:?} params {:?}", self, params);

        let txn = get_str_param("txn", params).map_err(error_err!())?;
        let sign = get_opt_bool_param("sign", params).map_err(error_err!())?.unwrap_or(false);

        let pool_handle = get_connected_pool_handle(&self.ctx)?;

        let res = if sign {
            let submitter_did = get_active_did(&self.ctx)?;
            let wallet_handle = get_opened_wallet_handle(&self.ctx)?;

            Ledger::sign_and_submit_request(pool_handle, wallet_handle, &submitter_did, txn)
        } else {
            Ledger::submit_request(pool_handle, txn)
        };

        let res = match res {
            Ok(_) => Ok(println_succ!("Transaction has been sent to Ledger")),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid transaction \"{}\"", txn)),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("There is no active did")),
            Err(err) => Err(println_err!("Send transaction failed with unexpected Indy SDK error {:?}", err)),
        };

        trace!("SenCustomCommand::execute << {:?}", res);
        Ok(())
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
    pub identifier: String,
    pub dest: String,
    pub role: Option<String>,
    pub alias: Option<String>,
    pub verkey: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct AttribData {
    pub endpoint: Option<Endpoint>,
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

#[derive(Deserialize, Debug)]
pub struct ClaimDefData {
    pub primary: serde_json::Value,
    pub revocation: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {}