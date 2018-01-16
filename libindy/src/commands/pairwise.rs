extern crate serde_json;
extern crate indy_crypto;

use errors::common::CommonError;
use errors::indy::IndyError;
use errors::wallet::WalletError;
use services::wallet::WalletService;

use std::error::Error;
use std::rc::Rc;
use std::str;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

pub enum PairwiseCommand {
    PairwiseExists(
        i32, // wallet handle
        String, // their_did
        Box<Fn(Result<bool, IndyError>) + Send>),
    CreatePairwise(
        i32, // wallet handle
        String, // their_did
        String, // my_did
        Option<String>, // metadata
        Box<Fn(Result<(), IndyError>) + Send>),
    ListPairwise(
        i32, // wallet handle
        Box<Fn(Result<String, IndyError>) + Send>),
    GetPairwise(
        i32, // wallet handle
        String, // their_did
        Box<Fn(Result<String, IndyError>) + Send>),
    SetPairwiseMetadata(
        i32, // wallet handle
        String, // their_did
        Option<String>, // metadata
        Box<Fn(Result<(), IndyError>) + Send>)
}

pub struct PairwiseCommandExecutor {
    wallet_service: Rc<WalletService>
}

impl PairwiseCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>) -> PairwiseCommandExecutor {
        PairwiseCommandExecutor {
            wallet_service: wallet_service
        }
    }

    pub fn execute(&self, command: PairwiseCommand) {
        match command {
            PairwiseCommand::PairwiseExists(wallet_handle, their_did, cb) => {
                info!(target: "pairwise_command_executor", "PairwiseExists command received");
                self.pairwise_exists(wallet_handle, &their_did, cb);
            }
            PairwiseCommand::CreatePairwise(wallet_handle, their_did, my_did, metadata, cb) => {
                info!(target: "pairwise_command_executor", "CreatePairwise command received");
                self.create_pairwise(wallet_handle, &their_did, &my_did, metadata.as_ref().map(String::as_str), cb);
            }
            PairwiseCommand::ListPairwise(wallet_handle, cb) => {
                info!(target: "pairwise_command_executor", "ListPairwise command received");
                self.list_pairwise(wallet_handle, cb);
            }
            PairwiseCommand::GetPairwise(wallet_handle, their_did, cb) => {
                info!(target: "pairwise_command_executor", "GetPairwise command received");
                self.get_pairwise(wallet_handle, &their_did, cb);
            }
            PairwiseCommand::SetPairwiseMetadata(wallet_handle, their_did, metadata, cb) => {
                info!(target: "pairwise_command_executor", "SetPairwiseMetadata command received");
                self.set_pairwise_metadata(wallet_handle, &their_did, metadata.as_ref().map(String::as_str), cb);
            }
        };
    }

    fn pairwise_exists(&self,
                       wallet_handle: i32,
                       their_did: &str,
                       cb: Box<Fn(Result<bool, IndyError>) + Send>) {
        cb(match self.wallet_service.get(wallet_handle, &format!("pairwise::{}", their_did)) {
            Ok(_) => Ok(true),
            Err(WalletError::NotFound(_)) => Ok(false),
            Err(err) => Err(IndyError::WalletError(err)),
        });
    }

    fn create_pairwise(&self,
                       wallet_handle: i32,
                       their_did: &str,
                       my_did: &str,
                       metadata: Option<&str>,
                       cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self._create_pairwise(wallet_handle, their_did, my_did, metadata));
    }

    fn _create_pairwise(&self,
                        wallet_handle: i32,
                        their_did: &str,
                        my_did: &str,
                        metadata: Option<&str>) -> Result<(), IndyError> {
        self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did))?;
        self.wallet_service.get(wallet_handle, &format!("their_did::{}", their_did))?;

        let pairwise_json = Pairwise::new(my_did.to_string(), their_did.to_string(),
                                          metadata.map(str::to_string)).to_json()
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Pairwise: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("pairwise::{}", their_did), &pairwise_json)?;

        Ok(())
    }

    fn list_pairwise(&self,
                     wallet_handle: i32,
                     cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._list_pairwise(wallet_handle));
    }

    fn _list_pairwise(&self,
                      wallet_handle: i32) -> Result<String, IndyError> {
        let list_pairwise: Vec<String> =
            self.wallet_service.list(wallet_handle, &format!("pairwise::"))?
                .iter()
                .map(|&(_, ref pair)| pair.clone())
                .collect::<Vec<String>>();

        let list_pairwise_json = serde_json::to_string(&list_pairwise)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't serialize {}", err)))?;

        Ok(list_pairwise_json)
    }

    fn get_pairwise(&self,
                    wallet_handle: i32,
                    their_did: &str,
                    cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._get_pairwise(wallet_handle, their_did));
    }

    fn _get_pairwise(&self,
                     wallet_handle: i32,
                     their_did: &str) -> Result<String, IndyError> {
        let pairwise = self.wallet_service.get(wallet_handle, &format!("pairwise::{}", their_did))?;

        let pairwise_info: PairwiseInfo = PairwiseInfo::from_json(&pairwise)
            .map_err(|e|
                CommonError::InvalidState(format!("Can't deserialize PairwiseInfo: {:?}", e)))?;

        let pairwise_info_json = pairwise_info.to_json()
            .map_err(|e|
                CommonError::InvalidState(format!("Can't serialize PairwiseInfo: {:?}", e)))?;

        Ok(pairwise_info_json)
    }

    fn set_pairwise_metadata(&self,
                             wallet_handle: i32,
                             their_did: &str,
                             metadata: Option<&str>,
                             cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self._set_pairwise_metadata(wallet_handle, their_did, metadata))
    }

    fn _set_pairwise_metadata(&self,
                              wallet_handle: i32,
                              their_did: &str,
                              metadata: Option<&str>) -> Result<(), IndyError> {
        let pairwise_json = self.wallet_service.get(wallet_handle, &format!("pairwise::{}", their_did))?;

        let mut pairwise: Pairwise = Pairwise::from_json(&pairwise_json)
            .map_err(|err|
                CommonError::InvalidState(format!("Can't deserialize Pairwise: {:?}", err)))?;

        pairwise.metadata = metadata.as_ref().map(|s| s.to_string());

        let pairwise_json = pairwise.to_json()
            .map_err(|err|
                CommonError::InvalidState(format!("Can't serialize Pairwise: {:?}", err)))?;

        self.wallet_service.set(wallet_handle, &format!("pairwise::{}", their_did), &pairwise_json)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pairwise {
    pub my_did: String,
    pub their_did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl Pairwise {
    pub fn new(my_did: String, their_did: String, metadata: Option<String>) -> Pairwise {
        Pairwise {
            my_did: my_did,
            their_did: their_did,
            metadata: metadata
        }
    }
}

impl JsonEncodable for Pairwise {}

impl<'a> JsonDecodable<'a> for Pairwise {}

#[derive(Serialize, Deserialize)]
pub struct PairwiseInfo {
    pub my_did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl JsonEncodable for PairwiseInfo {}

impl<'a> JsonDecodable<'a> for PairwiseInfo {}