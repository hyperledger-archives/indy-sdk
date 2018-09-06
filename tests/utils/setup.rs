use serde_json;
use super::indy;

use std::ops::{Index, IndexMut};
use std::iter::FromIterator;
use std::collections::HashMap;
use utils::constants::PROTOCOL_VERSION;
use utils::wallet::Wallet;
use utils::pool;
use utils::did;


pub struct SetupConfig
{
    pub num_trustees: u8,
    pub num_users: u8,
    pub num_addresses: usize,
    pub connect_to_pool: bool,
    pub number_of_nodes: u8,
    pub mint_tokens: Option<Vec<u64>>,
    pub fees: Option<serde_json::Value>,
}


pub struct Setup<'a>
{
    pub addresses: Option<Vec<String>>,
    pub fees: Option<serde_json::Value>,
    pub node_count: u8,
    pub pool_handle: Option<i32>,
    pub pool_name: String,
    pub trustees: Option<Entities>,
    pub users: Option<Entities>,
    wallet: &'a Wallet,
}

impl<'a> Setup<'a>
{

    /**
    Create a new Setup.

    Configures the pool, generates trustees and users, generate addresses, sets
    fees and mints tokens according to the [`SetupConfig`].

    [`SetupConfig`]: SetupConfig
    */
    pub fn new(wallet: &Wallet, config: SetupConfig) -> Setup
    {
//        sovtoken::api::sovtoken_init();
        let (pool_name, pool_handle) = Setup::setup_pool(config.connect_to_pool);
        let mut addresses = None;
        let mut fees = None;
        let mut trustees = None;
        let mut users = None;
        match pool_handle {
            Some(pool_handle) => {
//                addresses = Setup::create_addresses(wallet, config.num_addresses);
                trustees = Some(Setup::create_trustees(wallet, pool_handle, config.num_trustees));

                {
                    let trustee_dids = trustees.as_ref().unwrap().dids();

            //    if let Some(token_vec) = config.mint_tokens {
            //        assert!(token_vec.len() <= config.num_addresses, "You are minting to more addresses than are available.");
            //        Setup::mint(wallet, pool_handle, &trustee_dids, &addresses, token_vec);
            //    }

            //    if let Some(f) = config.fees {
            //        fees_utils::set_fees(pool_handle, wallet.handle, PAYMENT_METHOD_NAME, &f.to_string(), &trustee_dids);
            //        fees = Some(f);
            //    }

                    users = Some(Setup::create_users(wallet, pool_handle, trustee_dids[0], config.num_users));
                };
            }
            _ => ()
        }

        Setup {
            addresses,
            fees,
            node_count: config.number_of_nodes,
            pool_handle,
            pool_name,
            trustees,
            users,
            wallet,
        }
    }

    fn setup_pool(connect_to_pool: bool) -> (String, Option<i32>)
    {
        let pc_string = pool::create_pool_config();
        let pool_config = pc_string.as_str();
        indy::pool::Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let pool_name = pool::create_pool_ledger(pool_config);
        (
            pool_name.clone(),
            if connect_to_pool {
                let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();
                Some(pool_handle)
            } else {
                None
            }
        )
    }

    fn create_users(wallet: &Wallet, pool_handle: i32, did_trustee: &str, num_users: u8) -> Entities
    {
        did::create_multiple_nym(wallet.handle, pool_handle, did_trustee, num_users, did::NymRole::User)
            .unwrap()
            .into_iter()
            .map(Entity::new)
            .collect()
    }

    fn create_trustees(wallet: &Wallet, pool_handle: i32, num_trustees: u8) -> Entities
    {
        did::initial_trustees(num_trustees, wallet.handle, pool_handle)
            .unwrap()
            .into_iter()
            .map(Entity::new)
            .collect()
    }

//    fn create_addresses(wallet: &Wallet, num_addresses: usize) -> Vec<String>
//    {
//        gen_address::generate_n(wallet, num_addresses)
//    }

//    fn mint(wallet: &Wallet, pool_handle: i32, dids: &Vec<&str>, addresses: &Vec<String>, token_vec: Vec<u64>)
//    {
//        let map: HashMap<String, u64> = addresses
//            .clone()
//            .into_iter()
//            .zip(token_vec.into_iter())
//            .collect();
//
//        let mint_rep = mint::mint_tokens(map, pool_handle, wallet.handle, dids).unwrap();
//        assert_eq!(mint_rep.op, ResponseOperations::REPLY);
//    }

    fn fees_reset_json(fees: Option<serde_json::Value>) -> Option<String>
    {
        if fees.is_some() {
            type FeesMap = HashMap<String, u64>;
            let fees: FeesMap = serde_json::from_value(fees.unwrap()).unwrap();
            let mut map = HashMap::new();

            for k in fees.keys() {
                map.insert(k, 0);
            }

            Some(serde_json::to_string(&map).unwrap())
        } else {
            None
        }
    }
}

impl<'a> Drop for Setup<'a> {
    fn drop(&mut self) {
        indy::pool::Pool::delete(self.pool_name.as_str()).unwrap();
        if let Some(reset_fees) = Setup::fees_reset_json(self.fees.take()) {
//            let dids = self.trustees.dids();
//            fees_utils::set_fees(
//                self.pool_handle,
//                self.wallet.handle,
//                PAYMENT_METHOD_NAME,
//                &reset_fees,
//                &dids
//            );
        }
    }
}

/**
An entity with a did and a verkey.
*/
pub struct Entity
{
    pub did: String,
    pub verkey: String,
}

impl Entity {
    fn new((did, verkey): (String, String)) -> Self
    {
        Entity {
            did,
            verkey
        }
    }
}


/**
Contain a vector of [`Entity`].

You can access elements like an array.
```
use utils::setup::{Entities, Entity};
let entities = Entities(vec![
    Entity::new((String::from("V4SGRU86Z58d6TV7PBUe6f"), String::from("4TFcJS5FBo42EModbbaeYXHFoQAnmZKWrWKt8yWTB6Bq")))
    Entity::new((String::from("7LQt1bEbk5zB6gaFbEPDzB"), String::from("GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL")))
    Entity::new((String::from("Ln7kZXHFxZg5689JZciJMJ"), String::from("BnDcRVr6ZUkrNmxB2pmUbKVeZSuSnBecLFJNteS9iiM4")))
]);

assert_eq!(entities[1].did, "7LQt1bEbk5zB6gaFbEPDzB")
```

[`Entity`]: Entity
*/
pub struct Entities(Vec<Entity>);

impl Entities {

    /**
    The dids of the entities without the verkey.
    */
    pub fn dids(&self) -> Vec<&str> {
        self.0
            .iter()
            .map(|trust| trust.did.as_str())
            .collect()
    }
}

impl Index<usize> for Entities
{
    type Output = Entity;

    fn index(&self, i: usize) -> &Entity {
        &self.0[i]
    }
}

impl IndexMut<usize> for Entities
{
    fn index_mut(&mut self, i: usize) -> & mut Entity {
        &mut self.0[i]
    }
}

impl FromIterator<Entity> for Entities
{
    fn from_iter<I: IntoIterator<Item=Entity>>(iter: I) -> Entities {
        let mut v = Vec::new();
        for entity in iter {
            v.push(entity);
        }

        Entities(v)
    }
}
