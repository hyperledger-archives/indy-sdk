use errors::sovrin::SovrinError;
use commands::{Command, CommandExecutor};
use commands::sovrin::SovrinCommand;
use common::{SovrinRole, SovrinIdentity};

use std::error;
use std::sync::Arc;

pub struct SovrinAPI {
    command_executor: Arc<CommandExecutor>
}

impl SovrinAPI {
    /// Constructs a new `SovrinAPI`.
    ///
    /// #Params
    /// command_executor: Reference to `CommandExecutor` instance.
    ///
    pub fn new(command_executor: Arc<CommandExecutor>) -> SovrinAPI {
        SovrinAPI {
            command_executor: command_executor
        }
    }

    /// Sends NYM transaction to Identity Ledger.
    ///
    /// Creates a new NYM records for specific user, trust anchor, steward or trustee.
    /// Note that only trustees and stewards can create new sponsors and trustee can be created only by other trusties.
    ///
    /// #Params
    /// identity: Transaction identity.
    /// dest: Id of a target NYM record or an alias.
    /// verkey: Optional(defaults to dest). Verification key.
    /// xref: Optional (Required if dest is an alias). Id of a NYM record.
    /// data: Optional. Alias.
    /// role: Optional (defaults to None). Role of a user NYM record being created for.
    ///     One of USER, TRUST_ANCHOR, STEWARD, TRUSTEE.
    ///     Also a TRUSTEE can change any Nym's role to None, this stopping it from making any writes.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// No result.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `SovrinError` docs for common errors description.
    pub fn send_nym_tx(&self, identity: &SovrinIdentity, dest: &str, verkey: Option<&str>, xref: Option<&str>,
                       data: Option<&str>, role: Option<SovrinRole>,
                       cb: Box<Fn(Result<(), SovrinError>) + Send>) {
        self.command_executor.send(
            Command::Sovrin(
                SovrinCommand::SendNymTx(
                    dest.to_string(),
                    verkey.map(String::from),
                    xref.map(String::from),
                    data.map(String::from),
                    role,
                    cb)));
    }

    /// Sends ATTRIB transaction to Identity Ledger.
    ///
    /// Adds attribute to NYM record.
    ///
    /// #Params
    /// identity: Transaction identity.
    /// dest: Optional (defaults to origin). Id of a target NYM record.
    /// hash: Hash of attribute data.
    /// raw: Raw attribute data represented as json, where key is attribute name and value is it's value.
    /// enc: Optional. Encrypted attribute data.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// No result.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `SovrinError` docs for common errors description.
    pub fn send_attrib_tx(&self, identity: &SovrinIdentity, dest: Option<&str>, hash: &str, raw: &str, enc: &str,
                          cb: Box<Fn(Result<(), SovrinError>) + Send>) {
        unimplemented!();
    }

    /// Sends GET_ATTR transaction to Identity Ledger.
    ///
    /// Get attribute value.
    ///
    /// #Params
    /// identity: Transaction identity.
    /// dest: Id of a target NYM record.
    /// data: Attribute name.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Attribute value.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `SovrinError` docs for common errors description.
    pub fn send_get_att_tx(&self, identity: &SovrinIdentity, dest: &str, data: &str,
                           cb: Box<Fn(Result<String, SovrinError>) + Send>) {
        unimplemented!();
    }

    /// Sends GET_NYM transaction to Identity Ledger.
    ///
    /// Get information about existing NYM record, such as a role
    /// and id of a sponsor, who created it.
    ///
    /// #Params
    /// identity: Transaction identity.
    /// dest: Id of a target NYM record.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// NIM data represent as json.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `SovrinError` docs for common errors description.
    pub fn send_get_nym_tx(&self, identity: &SovrinIdentity, dest: &str,
                           cb: Box<Fn(Result<String, SovrinError>) + Send>) {
        unimplemented!();
    }

    /// Sends SCHEMA transaction to Identity Ledger.
    ///
    /// Write the schema of a claim on sovrin.
    ///
    /// #Params
    /// identity: Transaction identity.
    /// data: Schema represent as json: name, version, type, attr_names (ip, port, keys) and etc...
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// No data
    ///
    /// #Errors
    /// No method specific errors.
    /// See `SovrinError` docs for common errors description.
    pub fn send_schema_tx(&self, identity: &SovrinIdentity, data: &str,
                          cb: Box<Fn(Result<(), SovrinError>) + Send>) {
        unimplemented!();
    }

    /// Sends GET_SCHEMA transaction to Identity Ledger.
    ///
    /// Write the schema of a claim on sovrin.
    ///
    /// #Params
    /// identity: Transaction identity.
    /// data: Schema query represent as json: name and version
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Schema data represent as json.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `SovrinError` docs for common errors description.
    pub fn send_get_schema_tx(&self, identity: &SovrinIdentity, data: &str,
                              cb: Box<Fn(Result<String, SovrinError>) + Send>) {
        unimplemented!();
    }

    /// Sends ISSUER_KEY transaction to Identity Ledger.
    ///
    /// Set public key, that Issuer creates and publishes for a particular credential definition.
    ///
    /// #Params
    /// identity: Transaction identity.
    /// xref: Seq. number of schema.
    /// data: components of a key as json: N, R, S, Z.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// No data
    ///
    /// #Errors
    /// No method specific errors.
    /// See `SovrinError` docs for common errors description.
    pub fn send_issuer_key_tx(&self, identity: &SovrinIdentity, xref: &str, data: &str,
                              cb: Box<Fn(Result<(), SovrinError>) + Send>) {
        unimplemented!();
    }

    /// Sends GET_ISSUER_KEY transaction to Identity Ledger.
    ///
    /// Get issuer key.
    ///
    /// #Params
    /// identity: Transaction identity.
    /// xref: Seq. number of schema.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Issuer key represent as json.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `SovrinError` docs for common errors description.
    pub fn send_get_issuer_key_tx(&self, identity: &SovrinIdentity, xref: &str,
                                  cb: Box<Fn(Result<String, SovrinError>) + Send>) {
        unimplemented!();
    }

    /// Sends NODE transaction to Pool Ledger.
    ///
    /// Add new node to a cluster, or update existing node of pool.
    ///
    /// #Params
    /// identity: Transaction identity.
    /// dest: id of a target NYM record.
    /// data: Node data as json: node_ip, node_port, client_ip, client_port,alias.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// No data.
    ///
    /// #Errors
    /// No method specific errors.
/// See `SovrinError` docs for common errors description.
    pub fn send_node_tx(&self, identity: &SovrinIdentity, dest: &str, data: &str,
                        cb: Box<Fn(Result<(), SovrinError>) + Send>) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sovrin_api_can_be_created() {
        let sovrin_api = SovrinAPI::new(Arc::new(CommandExecutor::new()));
        assert! (true, "No crashes on SovrinAPI::new");
    }

    #[test]
    fn sovrin_api_can_be_dropped() {
        fn drop_test() {
            let sovrin_api = SovrinAPI::new(Arc::new(CommandExecutor::new()));
        }

        drop_test();
        assert! (true, "No crashes on SovrinAPI::drop");
    }
}