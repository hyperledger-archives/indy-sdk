extern crate serde_json;
extern crate indy_crypto;

use errors::common::CommonError;
use errors::indy::IndyError;
use errors::wallet::WalletError;
use services::wallet::{WalletService, WalletRecordRetrieveOptions, WalletSearch};

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
            wallet_service
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
        match self._wallet_get_pairwise(wallet_handle, their_did) {
            Ok(_) => Ok(true),
            Err(IndyError::WalletError(WalletError::NotFound(_))) => Ok(false),
            Err(err) => Err(err),
        }
    }

    fn create_pairwise(&self,
                       wallet_handle: i32,
                       their_did: &str,
                       my_did: &str,
                       metadata: Option<&str>) -> Result<(), IndyError> {
        self.wallet_service.get_record(wallet_handle, "Did", &my_did, WalletRecordRetrieveOptions::RETRIEVE_ID)?;
        self.wallet_service.get_record(wallet_handle, "TheirDid", &their_did, WalletRecordRetrieveOptions::RETRIEVE_ID)?;

        let pairwise = Pairwise {
            my_did: my_did.to_string(),
            their_did: their_did.to_string(),
            metadata: metadata.map(str::to_string)
        };

        self._wallet_set_pairwise(wallet_handle, &their_did, &pairwise)?;

        Ok(())
    }

    fn list_pairwise(&self,
                     wallet_handle: i32) -> Result<String, IndyError> {
        let mut list_pairwise: Vec<String> = Vec::new();

        let pairwise_search = self._wallet_pairwise_search(wallet_handle, "{}")?;

        while let Some(pairwise_record) = pairwise_search.fetch_next_record()? {
            let pairwise_value = pairwise_record.get_value()?;
            list_pairwise.push(pairwise_value.to_string());
        }

        let list_pairwise_json = serde_json::to_string(&list_pairwise)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't serialize {:?}", err)))?;

        Ok(list_pairwise_json)
    }

    fn get_pairwise(&self,
                    wallet_handle: i32,
                    their_did: &str) -> Result<String, IndyError> {
        let pairwise_info: PairwiseInfo = self._wallet_get_pairwise_info(wallet_handle, &their_did)?;

        let pairwise_info_json = pairwise_info.to_json()
            .map_err(|e|
                CommonError::InvalidState(format!("Can't serialize PairwiseInfo: {:?}", e)))?;

        Ok(pairwise_info_json)
    }

    fn set_pairwise_metadata(&self,
                             wallet_handle: i32,
                             their_did: &str,
                             metadata: Option<&str>) -> Result<(), IndyError> {
        let mut pairwise: Pairwise = self._wallet_get_pairwise(wallet_handle, &their_did)?;

        pairwise.metadata = metadata.as_ref().map(|m| m.to_string());

        self._wallet_update_pairwise(wallet_handle, &their_did, &pairwise)?;

        Ok(())
    }

    fn _wallet_set_pairwise(&self, wallet_handle: i32, id: &str, pairwise: &Pairwise) -> Result<String, IndyError> {
        self.wallet_service.set_object(wallet_handle, "Pairwise", id, pairwise, "{}")
    }

    fn _wallet_update_pairwise(&self, wallet_handle: i32, id: &str, pairwise: &Pairwise) -> Result<(), IndyError> {
        let pairwise_json = pairwise.to_json()
            .map_err(|err|
                CommonError::InvalidState(format!("Can't serialize Pairwise: {:?}", err)))?;

        self.wallet_service.update_record_value(wallet_handle, "Pairwise", id, &pairwise_json)
            .map_err(|err| IndyError::from(err))
    }

    fn _wallet_get_pairwise(&self, wallet_handle: i32, key: &str) -> Result<Pairwise, IndyError> {
        self.wallet_service.get_object::<Pairwise>(wallet_handle, "Pairwise", &key,
                                                   WalletRecordRetrieveOptions::RETRIEVE_ID_VALUE, &mut String::new())
    }

    fn _wallet_get_pairwise_info(&self, wallet_handle: i32, key: &str) -> Result<PairwiseInfo, IndyError> {
        self.wallet_service.get_object::<PairwiseInfo>(wallet_handle, "Pairwise", &key,
                                                       WalletRecordRetrieveOptions::RETRIEVE_ID_VALUE, &mut String::new())
    }

    fn _wallet_pairwise_search(&self, wallet_handle: i32, query: &str) -> Result<WalletSearch, WalletError> {
        self.wallet_service.search_records(wallet_handle, "Pairwise",
                                           query, WalletRecordRetrieveOptions::RETRIEVE_ID_VALUE)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pairwise {
    my_did: String,
    their_did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<String>,
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