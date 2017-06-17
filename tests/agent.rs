extern crate sovrin;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
extern crate zmq;

use std::sync::mpsc::channel;
use std::thread;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use utils::agent::AgentUtils;
use utils::signus::SignusUtils;
use utils::test::TestUtils;
use utils::timeout::TimeoutUtils;
use utils::wallet::WalletUtils;

mod high_cases {
    use super::*;

    #[test]
    fn sovrin_agent_listen_works_with_sovrin_agent_connect() {
        TestUtils::cleanup_storage();
        let wallet_handle = WalletUtils::create_and_open_wallet("pool3", "wallet3", "default").unwrap();
        let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
        let endpoint = "tcp://127.0.0.1:9701";

        let _ = AgentUtils::listen(wallet_handle, endpoint, None, None).unwrap();

        SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();

        AgentUtils::connect(wallet_handle, did.as_str(), did.as_str(), None).unwrap();

        TestUtils::cleanup_storage();
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
            let endpoint = "tcp://127.0.0.1:9702";

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

            AgentUtils::connect(wallet_handle, did.as_str(), did.as_str(), None).unwrap();

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
            let endpoint = "tcp://127.0.0.1:9703";
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();

            AgentUtils::listen(wallet_handle, endpoint, None, None).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod sovrin_agent_send {
        use super::*;

        #[test]
        fn sovrin_agent_send_works_for_all_data_in_wallet_present() {
            TestUtils::cleanup_storage();

            let (wait_conn_send, wait_conn_recv) = channel();
            let (wait_msg_from_srv_send, wait_msg_from_srv_recv) = channel();
            let (wait_msg_from_cli_send, wait_msg_from_cli_recv) = channel();
            let wallet_handle = WalletUtils::create_and_open_wallet("pool4", "wallet4", "default").unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let endpoint = "tcp://127.0.0.1:9704";
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();
            AgentUtils::listen(wallet_handle, endpoint,
                               Some(Box::new(move |_, conn_handle| {
                                   wait_conn_send.send(conn_handle).unwrap();
                               })),
                               Some(Box::new(move |_, msg| {
                                   wait_msg_from_cli_send.send(msg).unwrap();
                               }))
            ).unwrap();
            let cli_to_srv_connect_id = AgentUtils::connect(wallet_handle, did.as_str(), did.as_str(),
                                                            Some(Box::new(move |_, msg| {
                                                                wait_msg_from_srv_send.send(msg).unwrap();
                                                            }))).unwrap();
            let srv_to_cli_connect_id = wait_conn_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
            let client_msg = "msg_from_client";
            let server_msg = "msg_from_server";

            AgentUtils::send(cli_to_srv_connect_id, client_msg).unwrap();
            assert_eq!(wait_msg_from_cli_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap(), client_msg);
            AgentUtils::send(srv_to_cli_connect_id, server_msg).unwrap();
            assert_eq!(wait_msg_from_srv_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap(), server_msg);

            TestUtils::cleanup_storage();
        }
    }

    mod sovrin_agent_close_connection {
        use super::*;
        use sovrin::api::ErrorCode;

        #[test]
        fn sovrin_agent_close_connection_works_for_outgoing() {
            TestUtils::cleanup_storage();
            let wallet_handle = WalletUtils::create_and_open_wallet("pool3", "wallet3", "default").unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let endpoint = "tcp://127.0.0.1:9705";

            let _ = AgentUtils::listen(wallet_handle, endpoint, None, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();

            let conn_handle = AgentUtils::connect(wallet_handle, did.as_str(), did.as_str(), None).unwrap();

            AgentUtils::close(conn_handle).unwrap();
            assert_eq!(AgentUtils::send(conn_handle, "").unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }
}
