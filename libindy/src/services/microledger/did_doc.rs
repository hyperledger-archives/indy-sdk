use services::microledger::view::View;
use std::collections::HashMap;
use errors::common::CommonError;
use services::wallet::storage::WalletStorage;

pub struct DidDoc {
    pub did: String,
//    storage: Box<WalletStorage>,
}


impl View for DidDoc where Self: Sized {
    // initialize
    fn new(name: &str, options: HashMap<String, String>) -> Result<Self, CommonError> {

        Ok(DidDoc {did: name.to_string()})
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::test::TestUtils;
    use services::microledger::helpers::tests::valid_did_doc_storage_options;

    #[test]
    fn test_setup_did_doc() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let options = valid_did_doc_storage_options();
        let doc = DidDoc::new(did, options).unwrap();
    }

    #[test]
    fn test_add_key_txns_did_doc() {

    }

    #[test]
    fn test_add_endpoint_txns_did_doc() {

    }
}