extern crate indy;
extern crate indy_crypto;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate log;

#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::authz::AuthzUtils;
use utils::test::TestUtils;
use utils::pool::PoolUtils;
use utils::ledger::LedgerUtils;
use utils::crypto::CryptoUtils;
use utils::constants::*;

use indy::api::ErrorCode;

use serde_json::Value;
use self::indy_crypto::bn::BigNumber;


#[cfg(feature = "local_nodes_pool")]
use std::thread;

mod high_cases {
    use super::*;

    mod policy_creation {
        use super::*;

        // TODO: Tests contain duplicated setup code, fix it

        #[test]
        fn indy_policy_creation_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let policy_json = AuthzUtils::create_and_store_policy_address(wallet_handle).unwrap();
            println!("{:?}", policy_json);

            let policy: Value = serde_json::from_str(&policy_json).unwrap();
            println!("{:?}", policy);

            let policy_json1 = AuthzUtils::get_policy_from_wallet(wallet_handle,
                                                                  policy["address"].as_str().unwrap()).unwrap();
            println!("{:?}", policy_json1);

            assert_eq!(policy_json, policy_json1);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_adding_new_agent_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let policy_json = AuthzUtils::create_and_store_policy_address(wallet_handle).unwrap();
            println!("{:?}", policy_json);

            let policy: Value = serde_json::from_str(&policy_json).unwrap();
            println!("{:?}", policy);

            let policy_address = policy["address"].as_str().unwrap();
            println!("{:?}", &policy_address);

            // Add new agent but not commitment
            let vk1 = CryptoUtils::create_key(wallet_handle, None).unwrap();
            let verkey1 = AuthzUtils::add_agent_to_policy_in_wallet(wallet_handle, &policy_address, &vk1, false).unwrap();

            let policy_json1 = AuthzUtils::get_policy_from_wallet(wallet_handle,
                                                                  &policy_address).unwrap();
            println!("{:?}", policy_json1);

            let policy1: Value = serde_json::from_str(&policy_json1).unwrap();
            println!("{:?}", policy1);

            let agents = &policy1["agents"];
            println!("{:?}", agents);

            let agent1 = &agents[verkey1];
            println!("{:?}", agent1);

            assert_eq!(agent1["secret"], Value::Null);
            assert_eq!(agent1["blinding_factor"], Value::Null);
            assert_eq!(agent1["double_commitment"], Value::Null);
            assert_eq!(agent1["witness"], Value::Null);

            // Add new agent with commitment
            let vk2 = CryptoUtils::create_key(wallet_handle, None).unwrap();
            let verkey2 = AuthzUtils::add_agent_to_policy_in_wallet(wallet_handle, &policy_address,
                                                                    &vk2, true).unwrap();

            let policy_json2 = AuthzUtils::get_policy_from_wallet(wallet_handle,
                                                                  &policy_address).unwrap();
            println!("{:?}", policy_json2);

            let policy2: Value = serde_json::from_str(&policy_json2).unwrap();
            println!("{:?}", policy2);

            let agents = &policy2["agents"];
            println!("{:?}", agents);

            let agent2 = &agents[&verkey2];
            println!("{:?}", agent2);
            assert_ne!(agent2["secret"], Value::Null);
            assert_ne!(agent2["blinding_factor"], Value::Null);
            assert_ne!(agent2["double_commitment"], Value::Null);
            assert_eq!(agent2["witness"], Value::Null);

            // Update agent's witness
            let witness = BigNumber::rand(1024).unwrap().to_dec().unwrap();
            AuthzUtils::update_agent_witness_in_wallet(wallet_handle, &policy_address,
                                                       &vk2, &witness).unwrap();

            let policy_json3 = AuthzUtils::get_policy_from_wallet(wallet_handle,
                                                                  &policy_address).unwrap();
            println!("{:?}", policy_json3);

            let policy3: Value = serde_json::from_str(&policy_json3).unwrap();
            println!("{:?}", policy3);

            let agents = &policy3["agents"];
            let agent3 = &agents[&verkey2];
            println!("{:?}", agent3);
            assert_ne!(agent3["secret"], Value::Null);
            assert_ne!(agent3["blinding_factor"], Value::Null);
            assert_ne!(agent3["double_commitment"], Value::Null);
            assert_eq!(agent3["witness"].as_str().unwrap(), &witness);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_compute_witness() {
            let initial_witness_1 = "1331";
            let witnesses_1 = vec!["5", "7", "11"];
            let witness_json_1 = serde_json::to_string(&witnesses_1).unwrap();

            let new_witness = AuthzUtils::compute_witness(initial_witness_1, witness_json_1.as_str()).unwrap();
            assert_eq!(new_witness, "643504158456495625697294894076122012823298683918918075281316803450003609694415302955768088360165641868248952086646401307127674328326873426029868509392446241988350932860824209977773116899735527747077428647186062608990009130122466580368878631391603341117491674655831802472348604562075282713758145021132139243634753842910941462230351367036257887430027198923817903425020192483124504403534791129994423008788055739254190657959157400556530998210637619603740044322481303443828107362077833499439043516903924320997379304829053399020036137075376472170795183300411126361835013291383615995198701226528531974304804690795599985368304858499546530428258645122741096702771375744939621947053321465719368153530074085189520725327701282333068841766745681764016272802693858642551508467845520796439342281513339625504280086168448710404500812081465251982610186679494087127905208381088126523480415459827991447531595117408682106052372786550912807237506693637852707743522946364123383161276463930965679952139938884103691724591977572379110604241052728163275036347381297560295332317989274288788338706545050127261307630884118596502421600623535597634945502971108299184636179449878844937874684492921837936683726877817715957128234758240051");

            let initial_witness_2 = "514446009361148542614441078305986518393813921111053703923917065715489770734738272043048961545194016085289395850261985480640399989595053868575678026701592745897410618312172022705030916127420696220969368572758003815071065900356284561874651052721966906667104257072592529437233740441438219243155790635024147191756224502079955852373541222823557247831500137612582404038453917151849684051531267141519940356749749998811152040983601058889586276058629592493856186992015428821396339072825327546580576208555731808633933514297876996827844979863999713888154886881171582919179596487673082884055786175376027972297352248031939932477321215031312776228157997051132280436520709986150187403984138183535044718624384316242816655097332617015084097644368815644102095428963619495547402592229893209197544380188891035387623119432496373650434788337273550431493874088617902677550100556453985637391638499573654470129892413607938135321205854899827229472798542865559939196925400614859210174265141105690930940659282307566007024156766588248866635447527565391773235357177715896920506136183217818072026137644667319469003093706168583890290295529656977275303216287302553324022622759868094168153277393671408554881936038466345430452923249754070213661514022522194059793695199";
            let witnesses_2: Vec<&str> = vec![];
            let witness_json_2 = serde_json::to_string(&witnesses_2).unwrap();
            let new_witness_2 = AuthzUtils::compute_witness(initial_witness_2, witness_json_2.as_str()).unwrap();
            println!("{:?}", new_witness_2)
        }
    }
}