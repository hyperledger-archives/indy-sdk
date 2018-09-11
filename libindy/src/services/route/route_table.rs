use services::wallet::{WalletService};
use errors::wallet::WalletError;
use std::collections::HashMap;

pub struct RouteTable {
    wallet_service : WalletService
}


impl RouteTable {
    pub fn new (wallet_service: Option<WalletService>) -> RouteTable {
        let ws : WalletService = match wallet_service {
            Some(w) => w,
            None => WalletService::new(),
        };

        RouteTable { wallet_service: ws }

    }

    pub fn add_route(&mut self, did_with_key_frag : &str,
                    endpoint : &str, wallet_handle:i32) -> Result<(), WalletError> {
        self.wallet_service.add_record(wallet_handle, "route_table", did_with_key_frag, endpoint, &HashMap::new())
    }

    pub fn lookup_route(&mut self, did_with_key_frag : &str, wallet_handle : i32) -> Option<String> {
        let options_json = json!({"retrieveType": false,"retrieveValue": true,"retrieveTags": false}).to_string();
        let endpoint_result = self.wallet_service.get_record(wallet_handle,
                                                      "route_table",
                                                      did_with_key_frag,
                                                            &options_json);
        match endpoint_result {
            Ok(wr) => match wr.get_value() {
                Some(value) =>  Some(value.to_string()),
                None => None
            },
            Err(_) => None
        }
    }

    pub fn remove_route(&mut self, did_with_key_frag : &str, wallet_handle : i32) -> () {
        self.wallet_service.delete_record(wallet_handle, "route_table", did_with_key_frag);
    }

    pub fn update_route(&mut self, did_with_key_frag : &str, new_endpoint : &str, wallet_handle : i32) -> () {
        self.wallet_service.update_record_value(wallet_handle, "route_table", did_with_key_frag, new_endpoint);
    }
}


#[cfg(test)]
mod tests {
    use super::{RouteTable};
    use utils::inmem_wallet::InmemWallet;
    use utils::test::TestUtils;


    //TODO fix this test so that Error is returned not String
    #[test]
    fn test_add_and_lookup_route_fail() {
        _cleanup();
        let did_with_key_frag : &str = &"did:sov:NCjtLejiBg18RAV9mefAQT#1";
        let endpoint : &str = &"http://localhost:8080";
        let mut route_table : RouteTable = RouteTable::new(None);
        route_table.wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = route_table.wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        //let result = route_table.add_route(did_with_key_frag, endpoint, wallet_handle);
        let endpoint_lookup = route_table.lookup_route(did_with_key_frag, wallet_handle);
        assert!(endpoint_lookup.is_none());
    }

    //TODO fix this test and test above so it can identify if add or lookup is failing
    #[test]
    fn test_add_and_lookup_route_success() {
        _cleanup();
        let did_with_key_frag : &str = &"did:sov:NCjtLejiBg18RAV9mefAQT#1";
        let endpoint : &str = &"http://localhost:8080";
        let mut route_table : RouteTable = RouteTable::new(None);
        route_table.wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = route_table.wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        let result = route_table.add_route(did_with_key_frag, endpoint, wallet_handle);
        let endpoint_lookup = route_table.lookup_route(did_with_key_frag, wallet_handle).unwrap();
        assert_eq!(&endpoint_lookup, endpoint);
    }

    //TODO fix this test so it's more unit test style test for remove_route()
    #[test]
    fn test_add_remove_then_lookup_route_success() {
        _cleanup();
        let did_with_key_frag : &str = &"did:sov:NCjtLejiBg18RAV9mefAQT#1";
        let endpoint : &str = &"http://localhost:8080";
        let mut route_table : RouteTable = RouteTable::new(None);
        route_table.wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = route_table.wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        let result = route_table.add_route(did_with_key_frag, endpoint, wallet_handle);
        route_table.remove_route(did_with_key_frag, wallet_handle);
        let endpoint_lookup = route_table.lookup_route(did_with_key_frag, wallet_handle);
        assert!(endpoint_lookup.is_none());
    }

    //TODO fix this test so it's more unit test style test for update_route() aka doesn't rely on other route_table functions
    #[test]
    fn test_update_route_success() {
        _cleanup();
        let did_with_key_frag : &str = &"did:sov:NCjtLejiBg18RAV9mefAQT#1";
        let endpoint : &str = &"http://localhost:8080";
        let mut route_table : RouteTable = RouteTable::new(None);
        route_table.wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = route_table.wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        let result = route_table.add_route(did_with_key_frag, endpoint, wallet_handle);
        route_table.update_route(did_with_key_frag, &"http://localhost:8081", wallet_handle);
        let endpoint_lookup = route_table.lookup_route(did_with_key_frag, wallet_handle).unwrap();
        assert_eq!(&endpoint_lookup, "http://localhost:8081");
    }



    fn _config() -> String {
        json!({"id": "w1"}).to_string()
    }

    fn _credentials() -> String {
        json!({"key": "my_key"}).to_string()
    }

    fn _cleanup() {
        TestUtils::cleanup_storage();
        InmemWallet::cleanup();
    }

    fn _fetch_options(type_: bool, value: bool, tags: bool) -> String {
        json!({
          "retrieveType": type_,
          "retrieveValue": value,
          "retrieveTags": tags,
        }).to_string()
    }
}