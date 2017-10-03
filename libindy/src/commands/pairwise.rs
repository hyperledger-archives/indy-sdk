extern crate serde_json;

use utils::json::{JsonDecodable, JsonEncodable};
use errors::common::CommonError;
use errors::indy::IndyError;
use services::wallet::WalletService;

use std::error::Error;
use std::rc::Rc;
use std::str;

pub enum PairwiseCommand {
    PairwiseExists(
        i32, // wallet handle
        String, // their_did
        Box<Fn(Result<bool, IndyError>) + Send>),
    CreatePairwise(
        i32, // wallet handle
        String, // their_did
        String, // my_did
        Box<Fn(Result<(), IndyError>) + Send>),
    ListPairwise(
        i32, // wallet handle
        Box<Fn(Result<String, IndyError>) + Send>),
    PairwiseGetMyDid(
        i32, // wallet handle
        String, // their_did
        Box<Fn(Result<String, IndyError>) + Send>),
    SetPairwiseMetadata(
        i32, // wallet handle
        String, // their_did
        String, // metadata
        Box<Fn(Result<(), IndyError>) + Send>),
    GetPairwiseMetadata(
        i32, // wallet handle
        String, // their_did
        Box<Fn(Result<String, IndyError>) + Send>)
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
            PairwiseCommand::CreatePairwise(wallet_handle, their_did, my_did, cb) => {
                info!(target: "pairwise_command_executor", "CreatePairwise command received");
                self.create_pairwise(wallet_handle, &their_did, &my_did, cb);
            }
            PairwiseCommand::ListPairwise(wallet_handle, cb) => {
                info!(target: "pairwise_command_executor", "PairwiseList command received");
                self.pairwise_list(wallet_handle, cb);
            }
            PairwiseCommand::PairwiseGetMyDid(wallet_handle, their_did, cb) => {
                info!(target: "pairwise_command_executor", "PairwiseGetMyDid command received");
                self.pairwise_get_my_did(wallet_handle, &their_did, cb);
            }
            PairwiseCommand::SetPairwiseMetadata(wallet_handle, their_did, metadata, cb) => {
                info!(target: "pairwise_command_executor", "PairwiseSetMetadata command received");
                self.pairwise_set_metadata(wallet_handle, &their_did, &metadata, cb);
            }
            PairwiseCommand::GetPairwiseMetadata(wallet_handle, their_did, cb) => {
                info!(target: "pairwise_command_executor", "PairwiseGetMetadata command received");
                self.pairwise_get_metadata(wallet_handle, &their_did, cb);
            }
        };
    }

    fn pairwise_exists(&self,
                       wallet_handle: i32,
                       their_did: &str,
                       cb: Box<Fn(Result<bool, IndyError>) + Send>) {
        cb(Ok(self.wallet_service.get(wallet_handle, &format!("pairwise::{}", their_did)).is_ok()));
    }

    fn create_pairwise(&self,
                       wallet_handle: i32,
                       their_did: &str,
                       my_did: &str,
                       cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self._create_pairwise(wallet_handle, their_did, my_did));
    }

    fn _create_pairwise(&self,
                        wallet_handle: i32,
                        their_did: &str,
                        my_did: &str) -> Result<(), IndyError> {
        self.wallet_service.get(wallet_handle, &format!("my_did::{}", my_did))?;
        self.wallet_service.get(wallet_handle, &format!("their_did::{}", their_did))?;

        let pairwise_json = Pairwise::new(my_did.to_string(), their_did.to_string(), None).to_json()
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't serialize Pairwise: {}", err.description())))?;

        self.wallet_service.set(wallet_handle, &format!("pairwise::{}", their_did), &pairwise_json)?;

        Ok(())
    }

    fn pairwise_list(&self,
                     wallet_handle: i32,
                     cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._pairwise_list(wallet_handle));
    }

    fn _pairwise_list(&self,
                      wallet_handle: i32) -> Result<String, IndyError> {
        let pairwise_list: Vec<String> =
            self.wallet_service.list(wallet_handle, &format!("pairwise::"))?
                .iter()
                .map(|&(_, ref pair)| pair.clone())
                .collect::<Vec<String>>();

        let pairwise_list_json = serde_json::to_string(&pairwise_list)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't serialize {}", err)))?;

        Ok(pairwise_list_json)
    }

    fn pairwise_get_my_did(&self,
                           wallet_handle: i32,
                           their_did: &str,
                           cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._pairwise_get_my_did(wallet_handle, their_did));
    }

    fn _pairwise_get_my_did(&self,
                            wallet_handle: i32,
                            their_did: &str) -> Result<String, IndyError> {
        let pairwise = self.wallet_service.get(wallet_handle, &format!("pairwise::{}", their_did))?;

        let pair: Pairwise = Pairwise::from_json(&pairwise)
            .map_err(|e|
                CommonError::InvalidStructure(format!("Can't deserialize Pairwise: {:?}", e)))?;

        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", pair.my_did))?;

        let my_public_did: MyPublicDid = MyPublicDid::from_json(&my_did_json)
            .map_err(|e|
                CommonError::InvalidStructure(format!("Can't deserialize MyPublicDid: {:?}", e)))?;

        let my_public_did_json = my_public_did.to_json()
            .map_err(|e|
                CommonError::InvalidStructure(format!("Can't serialize MyPublicDid: {:?}", e)))?;

        Ok(my_public_did_json)
    }

    fn pairwise_set_metadata(&self,
                             wallet_handle: i32,
                             their_did: &str,
                             metadata: &str,
                             cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self._pairwise_set_metadata(wallet_handle, their_did, metadata))
    }

    fn _pairwise_set_metadata(&self,
                              wallet_handle: i32,
                              their_did: &str,
                              metadata: &str) -> Result<(), IndyError> {
        let pairwise_json = self.wallet_service.get(wallet_handle, &format!("pairwise::{}", their_did))?;

        let mut pairwise: Pairwise = Pairwise::from_json(&pairwise_json)
            .map_err(|err|
                CommonError::InvalidStructure(format!("Can't deserialize Pairwise: {:?}", err)))?;

        pairwise.metadata = Some(metadata.to_string());

        let pairwise_json = pairwise.to_json()
            .map_err(|err|
                CommonError::InvalidStructure(format!("Can't serialize Pairwise: {:?}", err)))?;

        self.wallet_service.set(wallet_handle, &format!("pairwise::{}", their_did), &pairwise_json)?;
        Ok(())
    }

    fn pairwise_get_metadata(&self,
                             wallet_handle: i32,
                             their_did: &str,
                             cb: Box<Fn(Result<String, IndyError>) + Send>) {
        cb(self._pairwise_get_metadata(wallet_handle, their_did));
    }

    fn _pairwise_get_metadata(&self,
                              wallet_handle: i32,
                              their_did: &str) -> Result<String, IndyError> {
        let pairwise_json = self.wallet_service.get(wallet_handle, &format!("pairwise::{}", their_did))?;

        let pairwise: Pairwise = Pairwise::from_json(&pairwise_json)
            .map_err(|err|
                CommonError::InvalidStructure(format!("Can't deserialize Pairwise: {:?}", err)))?;

        pairwise.metadata.ok_or(
            IndyError::CommonError(
                CommonError::InvalidStructure(format!("Metadata not found for: {:?}", their_did))))
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
pub struct MyPublicDid {
    pub did: String,
    pub crypto_type: String,
    pub pk: String,
    pub verkey: String
}

impl MyPublicDid {
    pub fn new(did: String, crypto_type: String, pk: String, verkey: String) -> MyPublicDid {
        MyPublicDid {
            did: did,
            crypto_type: crypto_type,
            pk: pk,
            verkey: verkey
        }
    }
}

impl JsonEncodable for MyPublicDid {}

impl<'a> JsonDecodable<'a> for MyPublicDid {}