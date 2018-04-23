extern crate serde_json;
extern crate indy_crypto;

use errors::common::CommonError;
use errors::indy::IndyError;
use services::wallet::{WalletService, RecordOptions};
use domain::pairwise::{Pairwise, PairwiseInfo};
use domain::crypto::did::{Did, TheirDid};

use std::rc::Rc;
use std::str;

use self::indy_crypto::utils::json::JsonEncodable;

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
        let res = self.wallet_service.record_exists::<Pairwise>(wallet_handle, their_did)?;
        Ok(res)
    }

    fn create_pairwise(&self,
                       wallet_handle: i32,
                       their_did: &str,
                       my_did: &str,
                       metadata: Option<&str>) -> Result<(), IndyError> {
        self.wallet_service.get_indy_record::<Did>(wallet_handle, &my_did, &RecordOptions::id())?;
        self.wallet_service.get_indy_record::<TheirDid>(wallet_handle, &their_did, &RecordOptions::id())?;

        let pairwise = Pairwise {
            my_did: my_did.to_string(),
            their_did: their_did.to_string(),
            metadata: metadata.map(str::to_string)
        };

        self.wallet_service.add_indy_object(wallet_handle, &their_did, &pairwise, "{}")?;

        Ok(())
    }

    fn list_pairwise(&self,
                     wallet_handle: i32) -> Result<String, IndyError> {
        let mut list_pairwise: Vec<String> = Vec::new();

        let mut pairwise_search =
            self.wallet_service.search_indy_records::<Pairwise>(wallet_handle, "{}", &RecordOptions::id_value())?;

        while let Some(pairwise_record) = pairwise_search.fetch_next_record()? {
            let pairwise_id = pairwise_record.get_id();
            let pairwise_value = pairwise_record.get_value()
                .ok_or(CommonError::InvalidStructure(format!("Pairwise not found for id: {}", pairwise_id)))?;

            list_pairwise.push(pairwise_value.to_string());
        }

        let list_pairwise_json = serde_json::to_string(&list_pairwise)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't serialize {:?}", err)))?;

        Ok(list_pairwise_json)
    }

    fn get_pairwise(&self,
                    wallet_handle: i32,
                    their_did: &str) -> Result<String, IndyError> {
        let pairwise_info = PairwiseInfo::from(self.wallet_service.get_indy_object::<Pairwise>(wallet_handle,
                                                                                               &their_did,
                                                                                               &RecordOptions::id_value(),
                                                                                               &mut String::new())?);

        let pairwise_info_json = pairwise_info.to_json()
            .map_err(|e|
                CommonError::InvalidState(format!("Can't serialize PairwiseInfo: {:?}", e)))?;

        Ok(pairwise_info_json)
    }

    fn set_pairwise_metadata(&self,
                             wallet_handle: i32,
                             their_did: &str,
                             metadata: Option<&str>) -> Result<(), IndyError> {
        let mut pairwise: Pairwise =
            self.wallet_service.get_indy_object(wallet_handle, &their_did, &RecordOptions::id_value(), &mut String::new())?;

        pairwise.metadata = metadata.as_ref().map(|m| m.to_string());

        self.wallet_service.update_indy_object(wallet_handle, &their_did, &pairwise)?;

        Ok(())
    }
}