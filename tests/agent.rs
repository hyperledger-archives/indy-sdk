extern crate sovrin;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
extern crate zmq;

use std::thread;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use utils::agent::AgentUtils;
use utils::signus::SignusUtils;
use utils::test::TestUtils;
use utils::wallet::WalletUtils;

mod high_cases {
    use super::*;

    #[test]
    fn sovrin_agent_listen_works_with_sovrin_agent_connect() {
        TestUtils::cleanup_storage();
        let wallet_handle = WalletUtils::create_and_open_wallet("pool3", "wallet3", "default").unwrap();
        let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

        let (_, endpoint): (i32, String) = AgentUtils::listen(wallet_handle).unwrap();

        let endpoint = endpoint.replace("0.0.0.0", "127.0.0.1");
        SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint.as_str()).unwrap();

        AgentUtils::connect(wallet_handle, did.as_str(), did.as_str()).unwrap();
    }

    mod sovrin_agent_connect {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn sovrin_agent_connect_works_for_all_data_in_wallet_present() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").expect("create wallet");

            let seed: Option<String> = Some("sovrin_agent_connect_works_for_a".to_string());
            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, seed).unwrap();
            let endpoint = "tcp://127.0.0.1:9700";

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();

            //FIXME temporary code: replace by sovrin_agent_listen
            thread::spawn(move || {
                let secret_key = zmq::z85_encode("6wBM7yEYWD7wGd3ZtNQX5r31uWuC8NoZS2Lr6HZvRTY4".from_base58().unwrap().as_slice()).unwrap();
                let public_key = zmq::z85_encode("2vTqP9QfNdvPr397QaFKtbVUPbhgqmAum2oDVkYsk4p9".from_base58().unwrap().as_slice()).unwrap();
                let socket: zmq::Socket = zmq::Context::new().socket(zmq::SocketType::ROUTER).unwrap();
                socket.set_curve_publickey(public_key.as_str()).unwrap();
                socket.set_curve_secretkey(secret_key.as_str()).unwrap();
                socket.set_curve_server(true).unwrap();
                socket.bind(endpoint).unwrap();
                socket.poll(zmq::POLLIN, -1).unwrap();
                let identity = socket.recv_string(zmq::DONTWAIT).unwrap().unwrap();
                let msg = socket.recv_string(zmq::DONTWAIT).unwrap().unwrap();
                info!("Fake agent socket - recv - from {}, msg {}", identity, msg);
                if msg.eq("DID") {
                    info!("Fake agent socket send ACK");
                    socket.send_multipart(&[identity.as_bytes(), "DID_ACK".as_bytes()], zmq::DONTWAIT).unwrap();
                }
            });
            //FIXME /temporary code

            AgentUtils::connect(wallet_handle, did.as_str(), did.as_str()).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod sovrin_agent_listen {
        use super::*;

        #[test]
        fn sovrin_agent_listen_works_for_all_data_in_wallet_present() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool2", "wallet2", "default").expect("create wallet");

            let seed: Option<String> = Some("sovrin_agent_listen_works_for_al".to_string());
            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, seed).unwrap();
            let endpoint = "tcp://127.0.0.1:9700";
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();

            AgentUtils::listen(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}
