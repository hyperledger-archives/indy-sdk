extern crate sovrin;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
extern crate zmq_pw as zmq;

use std::sync::mpsc::channel;
use std::thread;

use sovrin::api::ErrorCode;

#[macro_use]
mod utils;

use utils::agent::AgentUtils;
use utils::ledger::LedgerUtils;
use utils::pool::PoolUtils;
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
        let endpoint = "127.0.0.1:9701";

        let listener_handle = AgentUtils::listen(endpoint, None, None).unwrap();
        AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();

        SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();

        AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

        TestUtils::cleanup_storage();
    }

    mod sovrin_agent_connect {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn sovrin_agent_connect_works_for_remote_data() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config("sovrin_agent_connect_works_for_remote_data").unwrap();

            let endpoint = "127.0.0.1:9710";
            let listener_wallet = WalletUtils::create_and_open_wallet("sovrin_agent_connect_works_for_remote_data", "wallet10.1", "default").unwrap();
            let trustee_wallet = WalletUtils::create_and_open_wallet("sovrin_agent_connect_works_for_remote_data", "wallet10.2", "default").unwrap();
            let (listener_did, listener_ver_key, listener_pub_key) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (trustee_did, _, _) = SignusUtils::create_my_did(trustee_wallet, r#"{"seed":"000000000000000000000000Trustee1","cid":true}"#).unwrap();
            let sender_did = trustee_did.clone();
            let sender_wallet = trustee_wallet;

            let listener_nym_json = LedgerUtils::build_nym_request(trustee_did.as_str(), listener_did.as_str(), Some(listener_ver_key.as_str()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, trustee_wallet, trustee_did.as_str(), listener_nym_json.as_str()).unwrap();

            let listener_attrib_json =
                LedgerUtils::build_attrib_request(listener_did.as_str(), listener_did.as_str(), None,
                                                  Some(format!("{{\"endpoint\":{{\"ha\":\"{}\", \"verkey\":\"{}\"}}}}", endpoint, listener_pub_key).as_str()),
                                                  None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, listener_wallet, listener_did.as_str(), listener_attrib_json.as_str()).unwrap();

            let listener_handle = AgentUtils::listen(endpoint, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            AgentUtils::connect(pool_handle, sender_wallet, sender_did.as_str(), listener_did.as_str(), None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_agent_connect_works_for_all_data_in_wallet_present() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").expect("create wallet");

            let seed: Option<String> = Some("sovrin_agent_connect_works_for_a".to_string());
            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, seed).unwrap();
            let endpoint = "127.0.0.1:9702";

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();

            //FIXME temporary code: replace by sovrin_agent_listen
            thread::spawn(move || {
                let secret_key = "6wBM7yEYWD7wGd3ZtNQX5r31uWuC8NoZS2Lr6HZvRTY4".from_base58().unwrap();
                let public_key = "2vTqP9QfNdvPr397QaFKtbVUPbhgqmAum2oDVkYsk4p9".from_base58().unwrap();
                let socket: zmq::Socket = zmq::Context::new().socket(zmq::SocketType::ROUTER).unwrap();
                socket.set_curve_server(true).unwrap();
                socket.add_curve_keypair([public_key, secret_key].concat().as_slice()).unwrap();
                socket.bind(format!("tcp://{}", endpoint).as_str()).unwrap();
                socket.poll(zmq::POLLIN, -1).unwrap();
                let identity = socket.recv_string(zmq::DONTWAIT).unwrap().unwrap();
                let msg = socket.recv_string(zmq::DONTWAIT).unwrap().unwrap();
                info!("Fake agent socket - recv - from {}, msg {}", identity, msg);
                if msg.eq(r#"{"did":{"sender_did":"L1Xk2qCV6uxEEsYhP7B4EP","receiver_did":"L1Xk2qCV6uxEEsYhP7B4EP"}}"#) {
                    info!("Fake agent socket send ACK");
                    socket.send_multipart(&[identity.as_bytes(), "DID_ACK".as_bytes()], zmq::DONTWAIT).unwrap();
                }
            });
            //FIXME /temporary code

            AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

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
            let endpoint = "127.0.0.1:9703";
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();

            AgentUtils::listen(endpoint, None, None).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod sovrin_agent_add_identity {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn sovrin_agent_add_identity_works() {
            TestUtils::cleanup_storage();

            let endpoint = "127.0.0.1:9711";
            let receiver_wallet = WalletUtils::create_and_open_wallet("ignore", "wallet11receiver", "default").unwrap();
            let listener_handle = AgentUtils::listen(endpoint, None, None).unwrap();

            let (receiver_did, _, receiver_pk) = SignusUtils::create_and_store_my_did(receiver_wallet, None).unwrap();
            AgentUtils::add_identity(listener_handle, -1, receiver_wallet, receiver_did.as_str()).unwrap();

            let sock = zmq::Context::new().socket(zmq::SocketType::DEALER).unwrap();
            let kp = zmq::CurveKeyPair::new().unwrap();
            sock.set_identity(zmq::z85_encode(&kp.public_key).unwrap().as_bytes()).unwrap();
            sock.set_curve_publickey(&kp.public_key).unwrap();
            sock.set_curve_secretkey(&kp.secret_key).unwrap();
            sock.set_curve_serverkey(receiver_pk.from_base58().unwrap().as_slice()).unwrap();
            sock.set_protocol_version(zmq::make_proto_version(1, 1)).unwrap();
            sock.connect(format!("tcp://{}", endpoint).as_str()).unwrap();
            sock.send("test", zmq::DONTWAIT).unwrap();
            sock.poll(zmq::POLLIN, 100).unwrap();
            assert_eq!(sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(), "NOT_CONNECTED");

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_agent_add_identity_works_for_multiply_keys() {
            TestUtils::cleanup_storage();

            let endpoint = "127.0.0.1:9714";
            let receiver_wallet = WalletUtils::create_and_open_wallet("ignore", "wallet14receiver", "default").unwrap();
            let listener_handle = AgentUtils::listen(endpoint, None, None).unwrap();

            let (receiver_did1, _, receiver_pk1) = SignusUtils::create_and_store_my_did(receiver_wallet, None).unwrap();
            let (receiver_did2, _, receiver_pk2) = SignusUtils::create_and_store_my_did(receiver_wallet, None).unwrap();
            for receiver_did in [receiver_did1, receiver_did2].iter() {
                AgentUtils::add_identity(listener_handle, -1, receiver_wallet, receiver_did.as_str()).unwrap();
            }

            for receiver_pk in [receiver_pk1, receiver_pk2].iter() {
                let sock = zmq::Context::new().socket(zmq::SocketType::DEALER).unwrap();
                let kp = zmq::CurveKeyPair::new().unwrap();
                sock.set_identity(zmq::z85_encode(&kp.public_key).unwrap().as_bytes()).unwrap();
                sock.set_curve_publickey(&kp.public_key).unwrap();
                sock.set_curve_secretkey(&kp.secret_key).unwrap();
                sock.set_curve_serverkey(receiver_pk.from_base58().unwrap().as_slice()).unwrap();
                sock.set_protocol_version(zmq::make_proto_version(1, 1)).unwrap();
                sock.connect(format!("tcp://{}", endpoint).as_str()).unwrap();
                sock.send("test", zmq::DONTWAIT).unwrap();
                sock.poll(zmq::POLLIN, 100).unwrap();
                assert_eq!(sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(), "NOT_CONNECTED");
            }

            TestUtils::cleanup_storage();
        }
    }

    mod sovrin_agent_rm_identity {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn sovrin_agent_rm_identity_works() {
            TestUtils::cleanup_storage();

            let endpoint = "127.0.0.1:9713";
            let receiver_wallet = WalletUtils::create_and_open_wallet("ignore", "wallet13receiver", "default").unwrap();
            let listener_handle = AgentUtils::listen(endpoint, None, None).unwrap();

            let (receiver_did, _, receiver_pk) = SignusUtils::create_and_store_my_did(receiver_wallet, None).unwrap();
            AgentUtils::add_identity(listener_handle, -1, receiver_wallet, receiver_did.as_str()).unwrap();

            let sock = zmq::Context::new().socket(zmq::SocketType::DEALER).unwrap();
            let kp = zmq::CurveKeyPair::new().unwrap();
            sock.set_linger(0).unwrap();
            sock.set_identity(zmq::z85_encode(&kp.public_key).unwrap().as_bytes()).unwrap();
            sock.set_curve_publickey(&kp.public_key).unwrap();
            sock.set_curve_secretkey(&kp.secret_key).unwrap();
            sock.set_curve_serverkey(receiver_pk.from_base58().unwrap().as_slice()).unwrap();
            sock.set_protocol_version(zmq::make_proto_version(1, 1)).unwrap();
            sock.connect(format!("tcp://{}", endpoint).as_str()).unwrap();
            sock.send("test", zmq::DONTWAIT).unwrap();
            sock.poll(zmq::POLLIN, 1000).unwrap();
            assert_eq!(sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(), "NOT_CONNECTED");
            sock.disconnect(format!("tcp://{}", endpoint).as_str()).unwrap();

            AgentUtils::rm_identity(listener_handle, receiver_wallet, receiver_did.as_str()).unwrap();
            sock.connect(format!("tcp://{}", endpoint).as_str()).unwrap();
            sock.send("test", zmq::DONTWAIT).unwrap();
            sock.poll(zmq::POLLIN, 1000).unwrap();
            assert_eq!(sock.recv_string(zmq::DONTWAIT).unwrap_err(), zmq::Error::EAGAIN);
            sock.disconnect(format!("tcp://{}", endpoint).as_str()).unwrap();

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
            let endpoint = "127.0.0.1:9704";
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();
            let listener_handle = AgentUtils::listen(endpoint,
                                                     Some(Box::new(move |_, conn_handle| {
                                                         wait_conn_send.send(conn_handle).unwrap();
                                                     })),
                                                     Some(Box::new(move |_, msg| {
                                                         wait_msg_from_cli_send.send(msg).unwrap();
                                                     }))).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();
            let cli_to_srv_connect_id = AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(),
                                                            Some(Box::new(move |_, msg| {
                                                                wait_msg_from_srv_send.send(msg).unwrap();
                                                            }))).unwrap();
            let srv_to_cli_connect_id = wait_conn_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
            let client_msg = "msg_from_client";
            let server_msg = "msg_from_server";

            info!("Sending message from client to server");
            AgentUtils::send(cli_to_srv_connect_id, client_msg).unwrap();
            assert_eq!(wait_msg_from_cli_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap(), client_msg);
            info!("Sending message from server to client");
            AgentUtils::send(srv_to_cli_connect_id, server_msg).unwrap();
            assert_eq!(wait_msg_from_srv_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap(), server_msg);

            TestUtils::cleanup_storage();
        }
    }

    mod sovrin_agent_close_connection {
        use super::*;

        #[test]
        fn sovrin_agent_close_connection_works_for_outgoing() {
            TestUtils::cleanup_storage();
            let wallet_handle = WalletUtils::create_and_open_wallet("pool3", "wallet3", "default").unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let endpoint = "127.0.0.1:9705";

            let listener_handle = AgentUtils::listen(endpoint, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();

            let conn_handle = AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

            AgentUtils::close_connection(conn_handle).unwrap();
            assert_eq!(AgentUtils::send(conn_handle, "").unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_agent_close_connection_works_for_incoming_conn() {
            TestUtils::cleanup_storage();

            let (wait_conn_send, wait_conn_recv) = channel();
            let wallet_handle = WalletUtils::create_and_open_wallet("pool4", "wallet4", "default").unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let endpoint = "127.0.0.1:9706";
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();
            let listener_handle = AgentUtils::listen(endpoint,
                                                     Some(Box::new(move |_, conn_handle| {
                                                         wait_conn_send.send(conn_handle).unwrap();
                                                     })),
                                                     None).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();
            AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();
            let srv_to_cli_connect_id = wait_conn_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

            AgentUtils::close_connection(srv_to_cli_connect_id).unwrap();

            let server_msg = "msg_from_server";
            assert_eq!(AgentUtils::send(srv_to_cli_connect_id, server_msg).unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }

    mod sovrin_agent_close_listener {
        use super::*;

        #[test]
        fn sovrin_agent_close_listener_works() {
            TestUtils::cleanup_storage();

            let (wait_conn_send, wait_conn_recv) = channel();
            let wallet_handle = WalletUtils::create_and_open_wallet("pool8", "wallet8", "default").unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let endpoint = "127.0.0.1:9708";
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();
            let listener_handle = AgentUtils::listen(endpoint,
                                                     Some(Box::new(move |_, conn_handle| {
                                                         wait_conn_send.send(conn_handle).unwrap();
                                                     })),
                                                     None).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();
            AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();
            let srv_to_cli_connect_id = wait_conn_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

            AgentUtils::close_listener(listener_handle).unwrap();

            let server_msg = "msg_from_server";
            assert_eq!(AgentUtils::send(srv_to_cli_connect_id, server_msg).unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }
}

mod medium_cases {
    use super::*;

    mod sovrin_agent_add_identity {
        use super::*;

        #[test]
        fn sovrin_agent_add_identity_works_for_incoming_connection_require_ledger_request_but_pool_handle_is_invalid() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config("sovrin_agent_add_identity_works_for_incoming_connection_require_ledger_request_but_pool_handle_is_invalid").unwrap();

            let endpoint = "127.0.0.1:9712";
            let listener_wallet = WalletUtils::create_and_open_wallet("sovrin_agent_add_identity_works_for_incoming_connection_require_ledger_request_but_pool_handle_is_invalid", "wallet12.1", "default").unwrap();
            let trustee_wallet = WalletUtils::create_and_open_wallet("sovrin_agent_add_identity_works_for_incoming_connection_require_ledger_request_but_pool_handle_is_invalid", "wallet12.2", "default").unwrap();
            let (listener_did, listener_ver_key, listener_pub_key) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (trustee_did, _, _) = SignusUtils::create_my_did(trustee_wallet, r#"{"seed":"000000000000000000000000Trustee1","cid":true}"#).unwrap();
            let sender_did = trustee_did.clone();
            let sender_wallet = trustee_wallet;

            let listener_nym_json = LedgerUtils::build_nym_request(trustee_did.as_str(), listener_did.as_str(), Some(listener_ver_key.as_str()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, trustee_wallet, trustee_did.as_str(), listener_nym_json.as_str()).unwrap();

            let listener_attrib_json =
                LedgerUtils::build_attrib_request(listener_did.as_str(), listener_did.as_str(), None,
                                                  Some(format!("{{\"endpoint\":{{\"ha\":\"{}\", \"verkey\":\"{}\"}}}}", endpoint, listener_pub_key).as_str()),
                                                  None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, listener_wallet, listener_did.as_str(), listener_attrib_json.as_str()).unwrap();

            let listener_handle = AgentUtils::listen(endpoint, None, None).unwrap();
            let invalid_pool_handle = listener_handle;
            AgentUtils::add_identity(listener_handle, invalid_pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            /* TODO
             * Currently pool_handle and wallet_handle of add_identity will be checked only at required:
             * when listener will check incoming connection and go to ledger for info.
             * As result, add_identity will be successful but next connect will fail.
             * Possible the test should be split into two:
             * - add_identity_works_for_incompatible_pool_and_wallet
             *    with immediately check in the library
             * - connect_works_for_incorrect_connect_request
             *    actual info in ledger or listener_wallet, wrong public key in sender_wallet
             */

            assert_eq!(AgentUtils::connect(pool_handle, sender_wallet, sender_did.as_str(), listener_did.as_str(), None).unwrap_err(), ErrorCode::CommonInvalidState);

            TestUtils::cleanup_storage();
        }
    }

    mod sovrin_agent_close_connection {
        use super::*;

        #[test]
        fn sovrin_agent_close_connection_works_for_incorrect_conn_handle() {
            TestUtils::cleanup_storage();

            let (wait_msg_from_cli_send, wait_msg_from_cli_recv) = channel();
            let wallet_handle = WalletUtils::create_and_open_wallet("pool6", "wallet6", "default").unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let endpoint = "127.0.0.1:9707";
            let listener_handle = AgentUtils::listen(endpoint, None,
                                                     Some(Box::new(move |_, msg| {
                                                         wait_msg_from_cli_send.send(msg).unwrap();
                                                     }))).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();
            let conn_handle = AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

            assert_eq!(AgentUtils::close_connection(conn_handle + 100).unwrap_err(), ErrorCode::CommonInvalidStructure);

            let client_msg = "msg_from_cli_to_srv";
            AgentUtils::send(conn_handle, client_msg).unwrap();
            assert_eq!(wait_msg_from_cli_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap(), client_msg);

            TestUtils::cleanup_storage();
        }
    }

    mod sovrin_agent_close_listener {
        use super::*;

        #[test]
        fn sovrin_agent_close_listener_works_for_incorrect_handle() {
            TestUtils::cleanup_storage();

            let (wait_msg_from_cli_send, wait_msg_from_cli_recv) = channel();
            let wallet_handle = WalletUtils::create_and_open_wallet("pool9", "wallet9", "default").unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let endpoint = "127.0.0.1:9709";
            let listener_handle = AgentUtils::listen(endpoint, None,
                                                     Some(Box::new(move |_, msg| {
                                                         wait_msg_from_cli_send.send(msg).unwrap();
                                                     }))).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), endpoint).unwrap();
            let conn_handle = AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

            let incorrect_listener_handle = conn_handle;
            assert_eq!(AgentUtils::close_listener(incorrect_listener_handle).unwrap_err(), ErrorCode::CommonInvalidStructure);

            let client_msg = "msg_from_cli_to_srv";
            AgentUtils::send(conn_handle, client_msg).unwrap();
            assert_eq!(wait_msg_from_cli_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap(), client_msg);

            TestUtils::cleanup_storage();
        }
    }
}
