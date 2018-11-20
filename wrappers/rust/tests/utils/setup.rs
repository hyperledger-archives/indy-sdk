extern crate futures;

use self::futures::future::Future;

use super::indy;

use std::ops::{Index, IndexMut};
use std::iter::FromIterator;
use utils::constants::PROTOCOL_VERSION;
use utils::wallet::Wallet;
use utils::pool;
use utils::did;


pub struct SetupConfig
{
    pub connect_to_pool: bool,
    pub num_trustees: u8,
    pub num_users: u8,
    pub num_nodes: u8,
}


pub struct Setup<'a>
{
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

    Configures the pool, generates trustees and users, abd generate addresses
    according to the [`SetupConfig`].

    [`SetupConfig`]: SetupConfig
    */
    pub fn new(wallet: &Wallet, config: SetupConfig) -> Setup
    {
        let (pool_name, pool_handle) = Setup::setup_pool(config.connect_to_pool);
        let mut trustees = None;
        let mut users = None;

        match pool_handle {
            Some(pool_handle) => {
                if config.num_trustees > 0 {
                    trustees = Some(Setup::create_trustees(wallet, pool_handle, config.num_trustees));

                    {
                        let trustee_dids = trustees.as_ref().unwrap().dids();
                        users = Some(Setup::create_users(wallet, pool_handle, trustee_dids[0], config.num_users));
                    };
                }
            }
            _ => ()
        }

        Setup {
            node_count: config.num_nodes,
            pool_handle,
            pool_name,
            trustees,
            users,
            wallet,
        }
    }

    fn setup_pool(connect_to_pool: bool) -> (String, Option<i32>)
    {
        indy::pool::set_protocol_version(PROTOCOL_VERSION as usize).wait().unwrap();

        let pool_name = pool::create_default_pool();

        if connect_to_pool {
            let pool_handle = indy::pool::open_pool_ledger(&pool_name, None).wait().unwrap();
            (pool_name, Some(pool_handle))
        } else {
            (pool_name, None)
        }
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
}

impl<'a> Drop for Setup<'a> {
    fn drop(&mut self) {
        self.pool_handle.map(|handle| {
            indy::pool::close_pool_ledger(handle).wait().unwrap()
        });
        indy::pool::delete_pool_ledger(self.pool_name.as_str()).wait().unwrap();
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
