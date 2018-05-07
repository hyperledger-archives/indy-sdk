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
                cb(self.pairwise_exists(wallet_handle, &their_did));
            }
            PairwiseCommand::CreatePairwise(wallet_handle, their_did, my_did, metadata, cb) => {
                info!(target: "pairwise_command_executor", "CreatePairwise command received");
                cb(self.create_pairwise(wallet_handle, &their_did, &my_did, metadata.as_ref().map(String::as_str)));
            }
            PairwiseCommand::ListPairwise(wallet_handle, cb) => {
                info!(target: "pairwise_command_executor", "ListPairwise command received");
                cb(self.list_pairwise(wallet_handle));
            }
            PairwiseCommand::GetPairwise(wallet_handle, their_did, cb) => {
                info!(target: "pairwise_command_executor", "GetPairwise command received");
                cb(self.get_pairwise(wallet_handle, &their_did));
            }
            PairwiseCommand::SetPairwiseMetadata(wallet_handle, their_did, metadata, cb) => {
                info!(target: "pairwise_command_executor", "SetPairwiseMetadata command received");
                cb(self.set_pairwise_metadata(wallet_handle, &their_did, metadata.as_ref().map(String::as_str)));
            }
        };
    }

    fn pairwise_exists(&self,
                       wallet_handle: i32,
                       their_did: &str) -> Result<bool, IndyError> {
        debug!("pairwise_exists >>> wallet_handle: {:?}, their_did: {:?}", wallet_handle, their_did);

        let res = match self.wallet_service.get(wallet_handle, &format!("pairwise::{}", their_did)) {
            Ok(_) => Ok(true),
            Err(WalletError::NotFound(_)) => Ok(false),
            Err(err) => Err(IndyError::WalletError(err)),
        }?;

        debug!("pairwise_exists << res: {:?}", res);

        Ok(res)
    }

    fn create_pairwise(&self,
                        wallet_handle: i32,
                        their_did: &str,
                        my_did: &str,
                        metadata: Option<&str>) -> Result<(), IndyError> {
        debug!("create_pairwise >>> wallet_handle: {:?}, their_did: {:?}, my_did: {:?}, metadata: {:?}", wallet_handle, their_did, my_did, metadata);

        self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did))?;
        self.wallet_service.get(wallet_handle, &format!("their_did::{}", their_did))?;

        let pairwise_json = Pairwise::new(my_did.to_string(), their_did.to_string(),
                                          metadata.map(str::to_string)).to_json()
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Pairwise: {}", err.description())))?;

        let res = self.wallet_service.set(wallet_handle, &format!("pairwise::{}", their_did), &pairwise_json)?;

        debug!("create_pairwise <<< res: {:?}", res);

        Ok(res)
    }

    fn list_pairwise(&self,
                      wallet_handle: i32) -> Result<String, IndyError> {
        debug!("list_pairwise >>> wallet_handle: {:?}", wallet_handle);

        let list_pairwise: Vec<String> =
            self.wallet_service.list(wallet_handle, &format!("pairwise::"))?
                .iter()
                .map(|&(_, ref pair)| pair.clone())
                .collect::<Vec<String>>();

        let res = serde_json::to_string(&list_pairwise)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't serialize {}", err)))?;

        debug!("list_pairwise <<< res: {:?}", res);

        Ok(res)
    }

    fn get_pairwise(&self,
                     wallet_handle: i32,
                     their_did: &str) -> Result<String, IndyError> {
        debug!("get_pairwise >>> wallet_handle: {:?}, their_did: {:?}", wallet_handle, their_did);

        let pairwise = self.wallet_service.get(wallet_handle, &format!("pairwise::{}", their_did))?;

        let pairwise_info: PairwiseInfo = PairwiseInfo::from_json(&pairwise)
            .map_err(|e|
                CommonError::InvalidState(format!("Can't deserialize PairwiseInfo: {:?}", e)))?;

        let res = pairwise_info.to_json()
            .map_err(|e|
                CommonError::InvalidState(format!("Can't serialize PairwiseInfo: {:?}", e)))?;

        debug!("get_pairwise <<< res: {:?}", res);

        Ok(res)
    }

    fn set_pairwise_metadata(&self,
                              wallet_handle: i32,
                              their_did: &str,
                              metadata: Option<&str>) -> Result<(), IndyError> {
        debug!("set_pairwise_metadata >>> wallet_handle: {:?}, their_did: {:?}, metadata: {:?}", wallet_handle, their_did, metadata);

        let pairwise_json = self.wallet_service.get(wallet_handle, &format!("pairwise::{}", their_did))?;

        let mut pairwise: Pairwise = Pairwise::from_json(&pairwise_json)
            .map_err(|err|
                CommonError::InvalidState(format!("Can't deserialize Pairwise: {:?}", err)))?;

        pairwise.metadata = metadata.as_ref().map(|s| s.to_string());

        let pairwise_json = pairwise.to_json()
            .map_err(|err|
                CommonError::InvalidState(format!("Can't serialize Pairwise: {:?}", err)))?;

        let res = self.wallet_service.set(wallet_handle, &format!("pairwise::{}", their_did), &pairwise_json)?;

        debug!("set_pairwise_metadata <<< res: {:?}", res);

        Ok(res)
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