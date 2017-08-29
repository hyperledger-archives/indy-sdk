extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

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

use indy::api::ErrorCode;

#[macro_use]
mod utils;

use utils::agent::AgentUtils;
use utils::ledger::LedgerUtils;
use utils::pool::PoolUtils;
use utils::signus::SignusUtils;
use utils::test::TestUtils;
use utils::timeout::TimeoutUtils;
use utils::wallet::WalletUtils;

use std::sync::mpsc::RecvTimeoutError;

static POOL: &'static str = "pool_1";
static ENDPOINT: &'static str = "127.0.0.1:9700";
static TRUSTEE_SEED: &'static str = "000000000000000000000000Trustee1";
static CLIENT_MESSAGE: &'static str = "msg_from_client";
static SERVER_MESSAGE: &'static str = "msg_from_server";

mod high_cases {
    use super::*;

    #[test]
    fn indy_agent_listen_works_with_indy_agent_connect() {
        TestUtils::cleanup_storage();

        let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
        let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

        let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
        AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();

        SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), ENDPOINT).unwrap();

        AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

        AgentUtils::close_listener(listener_handle).unwrap();
        WalletUtils::close_wallet(wallet_handle).unwrap();

        TestUtils::cleanup_storage();
    }

    mod indy_agent_connect {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_agent_connect_works_for_remote_data() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let trustee_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (listener_did, listener_ver_key, listener_pub_key) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(trustee_wallet, Some(TRUSTEE_SEED)).unwrap();
            let sender_did = trustee_did.clone();
            let sender_wallet = trustee_wallet;

            let listener_nym_json = LedgerUtils::build_nym_request(trustee_did.as_str(), listener_did.as_str(), Some(listener_ver_key.as_str()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, trustee_wallet, trustee_did.as_str(), listener_nym_json.as_str()).unwrap();

            let listener_attrib_json =
                LedgerUtils::build_attrib_request(listener_did.as_str(), listener_did.as_str(), None,
                                                  Some(format!("{{\"endpoint\":{{\"ha\":\"{}\", \"verkey\":\"{}\"}}}}", ENDPOINT, listener_pub_key).as_str()),
                                                  None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, listener_wallet, listener_did.as_str(), listener_attrib_json.as_str()).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            AgentUtils::connect(pool_handle, sender_wallet, sender_did.as_str(), listener_did.as_str(), None).unwrap();

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(trustee_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_all_data_in_wallet_present() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).expect("create wallet");

            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, Some("sovrin_agent_connect_works_for_a")).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), "127.0.0.1:9700").unwrap();

            //FIXME temporary code: replace by indy_agent_listen
            thread::spawn(move || {
                let secret_key = "6wBM7yEYWD7wGd3ZtNQX5r31uWuC8NoZS2Lr6HZvRTY4".from_base58().unwrap();
                let public_key = "2vTqP9QfNdvPr397QaFKtbVUPbhgqmAum2oDVkYsk4p9".from_base58().unwrap();
                let socket: zmq::Socket = zmq::Context::new().socket(zmq::SocketType::ROUTER).unwrap();
                socket.set_curve_server(true).unwrap();
                socket.add_curve_keypair([public_key, secret_key].concat().as_slice()).unwrap();
                socket.bind(format!("tcp://{}", "127.0.0.1:9700").as_str()).unwrap();
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

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_listen {
        use super::*;

        #[test]
        fn indy_agent_listen_works_for_all_data_in_wallet_present() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let sender_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (listener_did, listener_verkey, listener_pk) = SignusUtils::create_and_store_my_did(listener_wallet, Some(TRUSTEE_SEED)).unwrap();
            let (sender_did, sender_ver_key, sender_pub_key) = SignusUtils::create_and_store_my_did(sender_wallet, None).unwrap();

            SignusUtils::store_their_did_from_parts(listener_wallet, sender_did.as_str(), sender_pub_key.as_str(), sender_ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            SignusUtils::store_their_did_from_parts(sender_wallet, listener_did.as_str(), listener_pk.as_str(), listener_verkey.as_str(), ENDPOINT).unwrap();

            AgentUtils::connect(pool_handle, sender_wallet, sender_did.as_str(), listener_did.as_str(), None).unwrap();

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_listen_works_for_get_sender_data_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let trustee_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let sender_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) = SignusUtils::create_and_store_my_did(trustee_wallet, Some(TRUSTEE_SEED)).unwrap();
            let (sender_did, sender_ver_key, sender_pub_key) = SignusUtils::create_and_store_my_did(sender_wallet, None).unwrap();
            let (listener_did, listener_verkey, listener_pk) = (trustee_did.clone(), trustee_verkey.clone(), trustee_pk.clone());
            let listener_wallet = trustee_wallet;

            let sender_nym_json = LedgerUtils::build_nym_request(trustee_did.as_str(), sender_did.as_str(),
                                                                 Some(sender_ver_key.as_str()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, trustee_wallet, trustee_did.as_str(), sender_nym_json.as_str()).unwrap();

            let sender_attrib_json =
                LedgerUtils::build_attrib_request(sender_did.as_str(), sender_did.as_str(), None,
                                                  Some(format!("{{\"endpoint\":{{\"ha\":\"{}\", \"verkey\":\"{}\"}}}}", ENDPOINT, sender_pub_key).as_str()),
                                                  None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, sender_wallet, sender_did.as_str(), sender_attrib_json.as_str()).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            SignusUtils::store_their_did_from_parts(sender_wallet, listener_did.as_str(), listener_pk.as_str(), listener_verkey.as_str(), ENDPOINT).unwrap();

            AgentUtils::connect(pool_handle, sender_wallet, sender_did.as_str(), listener_did.as_str(), None).unwrap();

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_listen_works_for_passed_on_connect_callback() {
            TestUtils::cleanup_storage();

            let msg = "New Connection";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).expect("create wallet");

            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), ENDPOINT).unwrap();

            let (wait_conn_send, wait_conn_recv) = channel();

            let listener_handle = AgentUtils::listen(ENDPOINT,
                                                     Some(Box::new(move |_, _| {
                                                         wait_conn_send.send(msg).unwrap();
                                                     })),
                                                     None).unwrap();

            AgentUtils::add_identity(listener_handle, pool_handle, wallet_handle, did.as_str()).unwrap();

            AgentUtils::connect(pool_handle, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

            assert_eq!(wait_conn_recv.recv_timeout(TimeoutUtils::medium_timeout()).unwrap(), msg);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_listen_works_for_passed_on_msg_callback() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), ENDPOINT).unwrap();

            let (wait_msg_send, wait_msg_recv) = channel();
            
            let listener_handle = AgentUtils::listen(ENDPOINT,
                                                     None,
                                                     Some(Box::new(move |_, msg| {
                                                         wait_msg_send.send(msg).unwrap();
                                                     }))).unwrap();

            AgentUtils::add_identity(listener_handle, pool_handle, wallet_handle, did.as_str()).unwrap();

            let conn_handle = AgentUtils::connect(pool_handle, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

            AgentUtils::send(conn_handle, CLIENT_MESSAGE).unwrap();

            assert_eq!(wait_msg_recv.recv_timeout(TimeoutUtils::medium_timeout()).unwrap(), CLIENT_MESSAGE);

            AgentUtils::close_listener(listener_handle).unwrap();
            AgentUtils::close_connection(conn_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_add_identity {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_agent_add_identity_works() {
            TestUtils::cleanup_storage();

            let receiver_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let (receiver_did, _, receiver_pk) = SignusUtils::create_and_store_my_did(receiver_wallet, None).unwrap();
            AgentUtils::add_identity(listener_handle, -1, receiver_wallet, receiver_did.as_str()).unwrap();

            let sock = zmq::Context::new().socket(zmq::SocketType::DEALER).unwrap();
            let kp = zmq::CurveKeyPair::new().unwrap();
            sock.set_identity(zmq::z85_encode(&kp.public_key).unwrap().as_bytes()).unwrap();
            sock.set_curve_publickey(&kp.public_key).unwrap();
            sock.set_curve_secretkey(&kp.secret_key).unwrap();
            sock.set_curve_serverkey(receiver_pk.from_base58().unwrap().as_slice()).unwrap();
            sock.set_protocol_version(zmq::make_proto_version(1, 1)).unwrap();
            sock.connect(format!("tcp://{}", ENDPOINT).as_str()).unwrap();
            sock.send("test", zmq::DONTWAIT).unwrap();
            sock.poll(zmq::POLLIN, 100).unwrap();
            assert_eq!(sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(), "NOT_CONNECTED");

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(receiver_wallet).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_add_identity_works_for_multiply_keys() {
            TestUtils::cleanup_storage();

            let receiver_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

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
                sock.connect(format!("tcp://{}", ENDPOINT).as_str()).unwrap();
                sock.send("test", zmq::DONTWAIT).unwrap();
                sock.poll(zmq::POLLIN, 100).unwrap();
                assert_eq!(sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(), "NOT_CONNECTED");
            }

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(receiver_wallet).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_rm_identity {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_agent_rm_identity_works() {
            TestUtils::cleanup_storage();

            let receiver_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

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
            sock.connect(format!("tcp://{}", ENDPOINT).as_str()).unwrap();
            sock.send("test", zmq::DONTWAIT).unwrap();
            sock.poll(zmq::POLLIN, 1000).unwrap();
            assert_eq!(sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(), "NOT_CONNECTED");
            sock.disconnect(format!("tcp://{}", ENDPOINT).as_str()).unwrap();

            AgentUtils::rm_identity(listener_handle, receiver_wallet, receiver_did.as_str()).unwrap();
            sock.connect(format!("tcp://{}", ENDPOINT).as_str()).unwrap();
            sock.send("test", zmq::DONTWAIT).unwrap();
            sock.poll(zmq::POLLIN, 1000).unwrap();
            assert_eq!(sock.recv_string(zmq::DONTWAIT).unwrap_err(), zmq::Error::EAGAIN);
            sock.disconnect(format!("tcp://{}", ENDPOINT).as_str()).unwrap();

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(receiver_wallet).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_send {
        use super::*;

        #[test]
        fn indy_agent_send_works_for_all_data_in_wallet_present() {
            TestUtils::cleanup_storage();

            let (wait_conn_send, wait_conn_recv) = channel();
            let (wait_msg_from_srv_send, wait_msg_from_srv_recv) = channel();
            let (wait_msg_from_cli_send, wait_msg_from_cli_recv) = channel();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), ENDPOINT).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT,
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

            AgentUtils::send(cli_to_srv_connect_id, CLIENT_MESSAGE).unwrap();
            assert_eq!(wait_msg_from_cli_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap(), CLIENT_MESSAGE);

            AgentUtils::send(srv_to_cli_connect_id, SERVER_MESSAGE).unwrap();
            assert_eq!(wait_msg_from_srv_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap(), SERVER_MESSAGE);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_close_connection {
        use super::*;

        #[test]
        fn indy_agent_close_connection_works_for_outgoing() {
            TestUtils::cleanup_storage();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), ENDPOINT).unwrap();

            let conn_handle = AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

            AgentUtils::close_connection(conn_handle).unwrap();
            assert_eq!(AgentUtils::send(conn_handle, "").unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_close_connection_works_for_incoming_conn() {
            TestUtils::cleanup_storage();

            let (wait_conn_send, wait_conn_recv) = channel();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), ENDPOINT).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT,
                                                     Some(Box::new(move |_, conn_handle| {
                                                         wait_conn_send.send(conn_handle).unwrap();
                                                     })),
                                                     None).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();
            AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();
            let srv_to_cli_connect_id = wait_conn_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

            AgentUtils::close_connection(srv_to_cli_connect_id).unwrap();

            assert_eq!(AgentUtils::send(srv_to_cli_connect_id, SERVER_MESSAGE).unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_close_listener {
        use super::*;

        #[test]
        fn indy_agent_close_listener_works() {
            TestUtils::cleanup_storage();

            let (wait_conn_send, wait_conn_recv) = channel();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), ENDPOINT).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT,
                                                     Some(Box::new(move |_, conn_handle| {
                                                         wait_conn_send.send(conn_handle).unwrap();
                                                     })),
                                                     None).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();
            AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();
            let srv_to_cli_connect_id = wait_conn_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

            AgentUtils::close_listener(listener_handle).unwrap();

            assert_eq!(AgentUtils::send(srv_to_cli_connect_id, SERVER_MESSAGE).unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}

mod medium_cases {
    use super::*;

    mod indy_agent_connect {
        use super::*;

        #[test]
        fn indy_agent_connect_works_for_unknow_listener() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let sender_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (listener_did, _, _) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (sender_did, _, _) = SignusUtils::create_and_store_my_did(sender_wallet, None).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            let res = AgentUtils::connect(pool_handle, sender_wallet, sender_did.as_str(), listener_did.as_str(), None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_invalid_remote_data() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let trustee_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (listener_did, listener_ver_key, listener_pub_key) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(trustee_wallet, Some(TRUSTEE_SEED)).unwrap();
            let sender_did = trustee_did.clone();
            let sender_wallet = trustee_wallet;

            let listener_nym_json = LedgerUtils::build_nym_request(trustee_did.as_str(), listener_did.as_str(),
                                                                   Some(listener_ver_key.as_str()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, trustee_wallet, trustee_did.as_str(),
                                                 listener_nym_json.as_str()).unwrap();

            let invalid_attrib_data = format!("{{\"endpoint\":{{\"verkey\":\"{}\"}}}}", listener_pub_key);
            let listener_attrib_json = LedgerUtils::build_attrib_request(listener_did.as_str(), listener_did.as_str(),
                                                                         None, Some(invalid_attrib_data.as_str()), None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, listener_wallet, listener_did.as_str(),
                                                 listener_attrib_json.as_str()).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            let res = AgentUtils::connect(pool_handle, sender_wallet,
                                          sender_did.as_str(), listener_did.as_str(), None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();


            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_expired_key_in_wallet() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let sender_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (listener_did, listener_ver_key, listener_pub_key) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (sender_did, _, _) = SignusUtils::create_and_store_my_did(sender_wallet, Some(TRUSTEE_SEED)).unwrap();

            SignusUtils::store_their_did_from_parts(sender_wallet, listener_did.as_str(),
                                                    listener_pub_key.as_str(), listener_ver_key.as_str(), ENDPOINT).unwrap();

            let (listener_new_ver_key, listener_new_pk) =
                SignusUtils::replace_keys(listener_wallet, listener_did.as_str(), "{}").unwrap();

            assert!(listener_ver_key != listener_new_ver_key);
            assert!(listener_pub_key != listener_new_pk);

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            let res = AgentUtils::connect_hang_up_expected(pool_handle, sender_wallet,
                                                           sender_did.as_str(), listener_did.as_str());
            assert_eq!(res.unwrap_err(), RecvTimeoutError::Timeout);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_expired_key_in_ledger() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let trustee_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (listener_did, listener_ver_key, listener_pub_key) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(trustee_wallet, Some(TRUSTEE_SEED)).unwrap();
            let sender_did = trustee_did.clone();
            let sender_wallet = trustee_wallet;

            let listener_nym_json = LedgerUtils::build_nym_request(trustee_did.as_str(), listener_did.as_str(),
                                                                   Some(listener_ver_key.as_str()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, trustee_wallet, trustee_did.as_str(),
                                                 listener_nym_json.as_str()).unwrap();

            let listener_attrib_json = LedgerUtils::build_attrib_request(listener_did.as_str(), listener_did.as_str(), None,
                                                                         Some(format!("{{\"endpoint\":{{\"ha\":\"{}\", \"verkey\":\"{}\"}}}}", ENDPOINT, listener_pub_key).as_str()),
                                                                         None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, listener_wallet, &listener_did, &listener_attrib_json).unwrap();

            let (listener_new_ver_key, listener_new_pk) =
                SignusUtils::replace_keys(listener_wallet, &listener_did, "{}").unwrap();

            assert!(listener_ver_key != listener_new_ver_key);
            assert!(listener_pub_key != listener_new_pk);

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            let res = AgentUtils::connect_hang_up_expected(pool_handle, sender_wallet,
                                                           sender_did.as_str(), listener_did.as_str());
            assert_eq!(res.unwrap_err(), RecvTimeoutError::Timeout);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_incompatible_wallet_and_pool() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let wallet_handle = WalletUtils::create_and_open_wallet("other_pool", None).unwrap();
            let (did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, wallet_handle, did.as_str()).unwrap();

            let res = AgentUtils::connect(pool_handle, wallet_handle, did.as_str(),
                                          did.as_str(), None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletIncompatiblePoolError);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_unknown_sender_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(),
                                                    ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, wallet_handle, did.as_str()).unwrap();

            let res = AgentUtils::connect(pool_handle, wallet_handle, "unknownDid",
                                          did.as_str(), None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_invalid_listener() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(),
                                                    ver_key.as_str(), ENDPOINT).unwrap();

            let res = AgentUtils::connect_hang_up_expected(0, wallet_handle,
                                                           did.as_str(), did.as_str());
            assert_eq!(res.unwrap_err(), RecvTimeoutError::Timeout);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_closed_listener() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(),
                                                    ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();

            AgentUtils::close_listener(listener_handle).unwrap();

            let res = AgentUtils::connect_hang_up_expected(0, wallet_handle,
                                                           did.as_str(), did.as_str());
            assert_eq!(res.unwrap_err(), RecvTimeoutError::Timeout);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_connect_without_add_identity() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(),
                                                    ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let res = AgentUtils::connect_hang_up_expected(0, wallet_handle,
                                                           did.as_str(), did.as_str());
            assert_eq!(res.unwrap_err(), RecvTimeoutError::Timeout);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            AgentUtils::close_listener(listener_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_connect_after_remove_identity() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).expect("create wallet");

            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(),
                                                    ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();

            AgentUtils::rm_identity(listener_handle, wallet_handle, did.as_str()).unwrap();

            let res = AgentUtils::connect_hang_up_expected(0, wallet_handle,
                                                           did.as_str(), did.as_str());
            assert_eq!(res.unwrap_err(), RecvTimeoutError::Timeout);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            AgentUtils::close_listener(listener_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_passed_callback() {
            TestUtils::cleanup_storage();

            let (wait_msg_send, wait_msg_recv) = channel();
            let (wait_conn_send, wait_conn_recv) = channel();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT,
                                                     Some(Box::new(move |_, conn_handle| {
                                                         wait_conn_send.send(conn_handle).unwrap();
                                                     })),
                                                     None).unwrap();

            AgentUtils::add_identity(listener_handle, pool_handle, wallet_handle, did.as_str()).unwrap();

            AgentUtils::connect(0, wallet_handle,
                                did.as_str(), did.as_str(),
                                Some(Box::new(move |_, msg| {
                                    wait_msg_send.send(msg).unwrap();
                                }))).unwrap();

            let conn_handle = wait_conn_recv.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();

            AgentUtils::send(conn_handle, CLIENT_MESSAGE).unwrap();
            assert_eq!(wait_msg_recv.recv_timeout(TimeoutUtils::medium_timeout()).unwrap(), CLIENT_MESSAGE);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_connect_works_for_twice() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let sender_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (listener_did, listener_ver_key, listener_pub_key) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (sender_did, _, _) = SignusUtils::create_and_store_my_did(sender_wallet, Some(TRUSTEE_SEED)).unwrap();

            SignusUtils::store_their_did_from_parts(sender_wallet, listener_did.as_str(),
                                                    listener_pub_key.as_str(), listener_ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            AgentUtils::connect(pool_handle, sender_wallet, sender_did.as_str(), listener_did.as_str(), None).unwrap();

            let res = AgentUtils::connect_hang_up_expected(pool_handle, sender_wallet,
                                                           sender_did.as_str(), listener_did.as_str());
            assert_eq!(res.unwrap_err(), RecvTimeoutError::Timeout);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_listen {
        use super::*;

        #[test]
        fn indy_agent_listen_works_for_endpoint_already_in_use() {
            TestUtils::cleanup_storage();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let res = AgentUtils::listen(ENDPOINT, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            AgentUtils::close_listener(listener_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_listen_works_for_invalid_endpoint() {
            TestUtils::cleanup_storage();

            let res = AgentUtils::listen("127.0.0", None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_listen_works_for_reject_unknow_sender() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let sender_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (listener_did, listener_ver_key, listener_pub_key) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (sender_did, _, _) = SignusUtils::create_and_store_my_did(sender_wallet, None).unwrap();

            SignusUtils::store_their_did_from_parts(sender_wallet, listener_did.as_str(),
                                                    listener_pub_key.as_str(), listener_ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            let res = AgentUtils::connect(pool_handle, sender_wallet,
                                          sender_did.as_str(), listener_did.as_str(), None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidState);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_listen_works_for_reject_expired_saved_sender_data() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let sender_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (listener_did, listener_verkey, listener_pk) = SignusUtils::create_and_store_my_did(listener_wallet, Some(TRUSTEE_SEED)).unwrap();
            let (sender_did, sender_ver_key, sender_pub_key) = SignusUtils::create_and_store_my_did(sender_wallet, None).unwrap();

            SignusUtils::store_their_did_from_parts(listener_wallet, sender_did.as_str(), sender_pub_key.as_str(), sender_ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            SignusUtils::replace_keys(sender_wallet, sender_did.as_str(), "{}").unwrap();

            SignusUtils::store_their_did_from_parts(sender_wallet, listener_did.as_str(), listener_pk.as_str(), listener_verkey.as_str(), ENDPOINT).unwrap();

            let res = AgentUtils::connect(pool_handle, sender_wallet, sender_did.as_str(), listener_did.as_str(), None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidState);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_add_identity {
        use super::*;

        #[test]
        fn indy_agent_add_identity_works_for_incoming_connection_require_ledger_request_but_pool_handle_is_invalid() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger("indy_agent_add_identity_works_for_incoming_connection_require_ledger_request_but_pool_handle_is_invalid").unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet("indy_agent_add_identity_works_for_incoming_connection_require_ledger_request_but_pool_handle_is_invalid", None).unwrap();
            let trustee_wallet = WalletUtils::create_and_open_wallet("indy_agent_add_identity_works_for_incoming_connection_require_ledger_request_but_pool_handle_is_invalid", None).unwrap();
            let (listener_did, listener_ver_key, listener_pub_key) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(trustee_wallet, Some(TRUSTEE_SEED)).unwrap();
            let sender_did = trustee_did.clone();
            let sender_wallet = trustee_wallet;

            let listener_nym_json = LedgerUtils::build_nym_request(trustee_did.as_str(), listener_did.as_str(), Some(listener_ver_key.as_str()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, trustee_wallet, trustee_did.as_str(), listener_nym_json.as_str()).unwrap();

            let listener_attrib_json =
                LedgerUtils::build_attrib_request(listener_did.as_str(), listener_did.as_str(), None,
                                                  Some(format!("{{\"endpoint\":{{\"ha\":\"{}\", \"verkey\":\"{}\"}}}}", ENDPOINT, listener_pub_key).as_str()),
                                                  None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, listener_wallet, listener_did.as_str(), listener_attrib_json.as_str()).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
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

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_add_identity_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let res = AgentUtils::add_identity(listener_handle, 1, wallet_handle, "unknownDid");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_add_identity_works_for_invalid_listener_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (receiver_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            let invalid_listener_handle = listener_handle + 1;

            let res = AgentUtils::add_identity(invalid_listener_handle, 1, wallet_handle, receiver_did.as_str());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_add_identity_works_for_twice() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let (did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();
            let res = AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_add_identity_works_for_two_did_on_same_listener() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let (did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (did2, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();
            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did2.as_str()).unwrap();

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_add_identity_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let (receiver_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let invalid_wallet_handle = wallet_handle + 1;

            let res = AgentUtils::add_identity(listener_handle, 1, invalid_wallet_handle, receiver_did.as_str());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_add_identity_works_after_remove() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let (did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();

            AgentUtils::rm_identity(listener_handle, wallet_handle, did.as_str()).unwrap();

            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_rm_identity {
        use super::*;

        #[test]
        fn indy_agent_rm_identity_works_for_invalid_listener_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let (did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();

            let invalid_listener_handle = listener_handle + 1;
            let res = AgentUtils::rm_identity(invalid_listener_handle, wallet_handle, did.as_str());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_rm_identity_works_for_twice() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let (receiver_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            AgentUtils::add_identity(listener_handle, 0, wallet_handle, receiver_did.as_str()).unwrap();

            AgentUtils::rm_identity(listener_handle, wallet_handle, receiver_did.as_str()).unwrap();

            let res = AgentUtils::rm_identity(listener_handle, wallet_handle, receiver_did.as_str());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_rm_identity_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let (did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = AgentUtils::rm_identity(listener_handle, invalid_wallet_handle, did.as_str());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_rm_identity_works_for_unknown_receiver_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();

            let (did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();

            let res = AgentUtils::rm_identity(listener_handle, wallet_handle, "unknownDid");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_send {
        use super::*;

        #[test]
        fn indy_agent_send_works_for_invalid_connection_handle() {
            TestUtils::cleanup_storage();
            
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(),
                                                    ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();

            let connection_handle = AgentUtils::connect(0, wallet_handle, did.as_str(),
                                                        did.as_str(), None).unwrap();
            let invalid_connection_handle = connection_handle + 100;

            let res = AgentUtils::send(invalid_connection_handle, CLIENT_MESSAGE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_send_works_for_closed_connection() {
            TestUtils::cleanup_storage();
            
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(),
                                                    ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();

            let connection_handle = AgentUtils::connect(0, wallet_handle, did.as_str(),
                                                        did.as_str(), None).unwrap();

            AgentUtils::close_connection(connection_handle).unwrap();

            let res = AgentUtils::send(connection_handle, CLIENT_MESSAGE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_send_works_for_closed_listener_incoming_connection() {
            TestUtils::cleanup_storage();

            let (wait_conn_send, wait_conn_recv) = channel();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let sender_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (listener_did, listener_verkey, listener_pub_key) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (sender_did, _, _) = SignusUtils::create_and_store_my_did(sender_wallet, Some(TRUSTEE_SEED)).unwrap();//TODO QUESTION

            SignusUtils::store_their_did_from_parts(sender_wallet, listener_did.as_str(),
                                                    listener_pub_key.as_str(), listener_verkey.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT,
                                                     Some(Box::new(move |_, conn_handle| {
                                                         wait_conn_send.send(conn_handle).unwrap();
                                                     })),
                                                     None).unwrap();

            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            AgentUtils::connect(pool_handle, sender_wallet, sender_did.as_str(), listener_did.as_str(), None).unwrap();
            let srv_to_cli_connect_id = wait_conn_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

            AgentUtils::close_listener(listener_handle).unwrap();

            let res = AgentUtils::send(srv_to_cli_connect_id, SERVER_MESSAGE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[ignore] //[IS-282] Close all incoming connections associated with DID after call remove_identity
        fn indy_agent_send_works_for_removed_identity() {
            TestUtils::cleanup_storage();

            let (wait_conn_send, wait_conn_recv) = channel();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let listener_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let sender_wallet = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (listener_did, listener_verkey, listener_pub_key) = SignusUtils::create_and_store_my_did(listener_wallet, None).unwrap();
            let (sender_did, _, _) = SignusUtils::create_and_store_my_did(sender_wallet, Some(TRUSTEE_SEED)).unwrap();

            SignusUtils::store_their_did_from_parts(sender_wallet, listener_did.as_str(),
                                                    listener_pub_key.as_str(), listener_verkey.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT,
                                                     Some(Box::new(move |_, conn_handle| {
                                                         wait_conn_send.send(conn_handle).unwrap();
                                                     })),
                                                     None).unwrap();

            AgentUtils::add_identity(listener_handle, pool_handle, listener_wallet, listener_did.as_str()).unwrap();

            AgentUtils::connect(pool_handle, sender_wallet, sender_did.as_str(), listener_did.as_str(), None).unwrap();
            let srv_to_cli_connect_id = wait_conn_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

            AgentUtils::rm_identity(listener_handle, listener_wallet, listener_did.as_str()).unwrap();

            let res = AgentUtils::send(srv_to_cli_connect_id, SERVER_MESSAGE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(listener_wallet).unwrap();
            WalletUtils::close_wallet(sender_wallet).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_close_connection {
        use super::*;

        #[test]
        fn indy_agent_close_connection_works_for_incorrect_conn_handle() {
            TestUtils::cleanup_storage();

            let (wait_msg_from_cli_send, wait_msg_from_cli_recv) = channel();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None,
                                                     Some(Box::new(move |_, msg| {
                                                         wait_msg_from_cli_send.send(msg).unwrap();
                                                     }))).unwrap();
            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();

            let conn_handle = AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

            assert_eq!(AgentUtils::close_connection(conn_handle + 100).unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::send(conn_handle, CLIENT_MESSAGE).unwrap();
            assert_eq!(wait_msg_from_cli_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap(), CLIENT_MESSAGE);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_close_connection_works_for_twice() {
            TestUtils::cleanup_storage();
            
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, 0, wallet_handle, did.as_str()).unwrap();

            let conn_handle = AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

            AgentUtils::close_connection(conn_handle).unwrap();

            let res = AgentUtils::close_connection(conn_handle);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod indy_agent_close_listener {
        use super::*;

        #[test]
        fn indy_agent_close_listener_works_for_incorrect_handle() {
            TestUtils::cleanup_storage();

            let (wait_msg_from_cli_send, wait_msg_from_cli_recv) = channel();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (did, ver_key, pub_key): (String, String, String) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None,
                                                     Some(Box::new(move |_, msg| {
                                                         wait_msg_from_cli_send.send(msg).unwrap();
                                                     }))).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();
            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(), ver_key.as_str(), ENDPOINT).unwrap();
            let conn_handle = AgentUtils::connect(0, wallet_handle, did.as_str(), did.as_str(), None).unwrap();

            let incorrect_listener_handle = conn_handle;
            assert_eq!(AgentUtils::close_listener(incorrect_listener_handle).unwrap_err(), ErrorCode::CommonInvalidStructure);

            AgentUtils::send(conn_handle, CLIENT_MESSAGE).unwrap();
            assert_eq!(wait_msg_from_cli_recv.recv_timeout(TimeoutUtils::short_timeout()).unwrap(), CLIENT_MESSAGE);

            AgentUtils::close_listener(listener_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_agent_close_listener_works_for_twice() {
            TestUtils::cleanup_storage();
            
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            SignusUtils::store_their_did_from_parts(wallet_handle, did.as_str(), pub_key.as_str(),
                                                    ver_key.as_str(), ENDPOINT).unwrap();

            let listener_handle = AgentUtils::listen(ENDPOINT, None, None).unwrap();
            AgentUtils::add_identity(listener_handle, -1, wallet_handle, did.as_str()).unwrap();

            AgentUtils::close_listener(listener_handle).unwrap();

            let res = AgentUtils::close_listener(listener_handle);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}
