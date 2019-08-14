#[macro_use]
extern crate criterion;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate named_type_derive;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate byteorder;
extern crate indy;
extern crate ursa;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;
extern crate rand;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[path = "../tests/utils/mod.rs"]
#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::non_secrets::NonSecretsUtils;
use utils::test::TestUtils;
use utils::constants::*;

use criterion::{Criterion, Benchmark};

use utils::sequence::SequenceUtils;
use rand::Rng;


mod create {
    use super::*;

    fn setup() {
        TestUtils::cleanup_storage();
    }

    fn create_wallet(credentials: &str) {
        WalletUtils::create_wallet(DEFAULT_WALLET_CONFIG, credentials).unwrap();
    }

    pub fn bench(c: &mut Criterion) {
        c.bench(
            "wallet_create",
            Benchmark::new(
                "wallet_create_argon2i_mod",
                |b| b.iter_with_setup(setup, |()| create_wallet(WALLET_CREDENTIALS_ARGON2I_MOD))
            ).sample_size(10),
        );

        c.bench(
            "wallet_create",
            Benchmark::new(
                "wallet_create_argon2i_int",
                |b| b.iter_with_setup(setup, |()| create_wallet(WALLET_CREDENTIALS_ARGON2I_INT))
            ).sample_size(20),
        );

        c.bench(
            "wallet_create",
            Benchmark::new(
                "wallet_create_raw",
                |b| b.iter_with_setup(setup, |()| create_wallet(WALLET_CREDENTIALS_RAW))
            ).sample_size(30),
        );
    }
}

mod open {
    use super::*;

    pub static mut WALLET_HANDLE: i32 = 0;

    pub const WALLET_CONFIG_ARGON2I_MOD: &'static str = r#"{"id":"wallet_open_ARGON2I_MOD"}"#;
    pub const WALLET_CONFIG_ARGON2I_INT: &'static str = r#"{"id":"wallet_open_ARGON2I_INT"}"#;
    pub const WALLET_CONFIG_RAW: &'static str = r#"{"id":"wallet_open_RAW"}"#;

    fn pre_setup() {
        TestUtils::cleanup_storage();

        WalletUtils::create_wallet(WALLET_CONFIG_ARGON2I_MOD, WALLET_CREDENTIALS_ARGON2I_MOD).unwrap();
        WalletUtils::create_wallet(WALLET_CONFIG_ARGON2I_INT, WALLET_CREDENTIALS_ARGON2I_INT).unwrap();
        WalletUtils::create_wallet(WALLET_CONFIG_RAW, WALLET_CREDENTIALS_RAW).unwrap();
    }

    fn setup() {
        unsafe { if WALLET_HANDLE != 0 { WalletUtils::close_wallet(WALLET_HANDLE).unwrap(); } }
    }

    fn open_wallet(config: &str, credentials: &str) {
        unsafe { WALLET_HANDLE = WalletUtils::open_wallet(config, credentials).unwrap(); }
    }

    pub fn bench(c: &mut Criterion) {
        pre_setup();

        c.bench(
            "wallet_open",
            Benchmark::new(
                "wallet_open_argon2i_mod",
                |b| b.iter_with_setup(|| setup(), |()| open_wallet(WALLET_CONFIG_ARGON2I_MOD, WALLET_CREDENTIALS_ARGON2I_MOD))
            ).sample_size(10),
        );

        c.bench(
            "wallet_open",
            Benchmark::new(
                "wallet_open_argon2i_int",
                |b| b.iter_with_setup(|| setup(), |()| open_wallet(WALLET_CONFIG_ARGON2I_INT, WALLET_CREDENTIALS_ARGON2I_INT))
            ).sample_size(20),
        );

        c.bench(
            "wallet_open",
            Benchmark::new(
                "wallet_open_raw",
                |b| b.iter_with_setup(|| setup(), |()| open_wallet(WALLET_CONFIG_RAW, WALLET_CREDENTIALS_RAW))
            ).sample_size(30),
        );
    }
}

mod close {
    use super::*;

    pub const WALLET_CONFIG_ARGON2I_MOD: &'static str = r#"{"id":"wallet_close_ARGON2I_MOD"}"#;
    pub const WALLET_CONFIG_ARGON2I_INT: &'static str = r#"{"id":"wallet_close_ARGON2I_INT"}"#;
    pub const WALLET_CONFIG_RAW: &'static str = r#"{"id":"wallet_close_RAW"}"#;

    fn pre_setup() {
        TestUtils::cleanup_storage();

        WalletUtils::create_wallet(WALLET_CONFIG_ARGON2I_MOD, WALLET_CREDENTIALS_ARGON2I_MOD).unwrap();
        WalletUtils::create_wallet(WALLET_CONFIG_ARGON2I_INT, WALLET_CREDENTIALS_ARGON2I_INT).unwrap();
        WalletUtils::create_wallet(WALLET_CONFIG_RAW, WALLET_CREDENTIALS_RAW).unwrap();
    }

    fn setup(config: &str, credentials: &str) -> i32 {
        WalletUtils::open_wallet(config, credentials).unwrap()
    }

    fn close_wallet(handle: i32) {
        WalletUtils::close_wallet(handle).unwrap();
    }

    pub fn bench(c: &mut Criterion) {
        pre_setup();

        c.bench(
            "wallet_close",
            Benchmark::new(
                "wallet_close_argon2i_mod",
                |b| b.iter_with_setup(|| setup(WALLET_CONFIG_ARGON2I_MOD, WALLET_CREDENTIALS_ARGON2I_MOD), |handle| close_wallet(handle)),
            ).sample_size(10),
        );

        c.bench(
            "wallet_close",
            Benchmark::new(
                "wallet_close_argon2i_int",
                |b| b.iter_with_setup(|| setup(WALLET_CONFIG_ARGON2I_INT, WALLET_CREDENTIALS_ARGON2I_INT), |handle| close_wallet(handle)),
            ).sample_size(20),
        );

        c.bench(
            "wallet_close",
            Benchmark::new(
                "wallet_close_raw",
                |b| b.iter_with_setup(|| setup(WALLET_CONFIG_RAW, WALLET_CREDENTIALS_RAW), |handle| close_wallet(handle)),
            ).sample_size(30),
        );
    }
}

mod delete {
    use super::*;

    fn pre_setup() {
        TestUtils::cleanup_storage();
    }

    fn setup(credentials: &str) {
        WalletUtils::create_wallet(DEFAULT_WALLET_CONFIG, credentials).unwrap();
    }

    fn delete_wallet(credentials: &str) {
        WalletUtils::delete_wallet(DEFAULT_WALLET_CONFIG, credentials).unwrap();
    }

    pub fn bench(c: &mut Criterion) {
        pre_setup();

        c.bench(
            "wallet_delete",
            Benchmark::new(
                "wallet_delete_argon2i_mod",
                |b| b.iter_with_setup(|| setup(WALLET_CREDENTIALS_ARGON2I_MOD), |()| delete_wallet(WALLET_CREDENTIALS_ARGON2I_MOD)),
            ).sample_size(10),
        );

        c.bench(
            "wallet_delete",
            Benchmark::new(
                "wallet_delete_argon2i_int",
                |b| b.iter_with_setup(|| setup(WALLET_CREDENTIALS_ARGON2I_INT), |()| delete_wallet(WALLET_CREDENTIALS_ARGON2I_INT)),
            ).sample_size(20),
        );

        c.bench(
            "wallet_delete",
            Benchmark::new(
                "wallet_delete_raw",
                |b| b.iter_with_setup(|| setup(WALLET_CREDENTIALS_RAW), |()| delete_wallet(WALLET_CREDENTIALS_RAW)),
            ).sample_size(30),
        );
    }
}

mod get_record {
    use super::*;

    fn get_record(wallet_handle: WalletHandle, type_: &str, id: &str) {
        NonSecretsUtils::get_wallet_record(wallet_handle, type_, id, "{}").unwrap();
    }

    pub fn bench(c: &mut Criterion) {
        let wallet_handle = init_wallet();

        c.bench(
            "wallet_get_record",
            Benchmark::new("wallet_get_record", move |b|
                b.iter_with_setup(get_rand_key, |(type_, id): (String, String)| get_record(wallet_handle, &type_, &id)))
                .sample_size(50));
    }
}

mod delete_record {
    use super::*;

    static mut INDEX: usize = 0;

    fn setup() -> (String, String) {
        unsafe {
            INDEX = INDEX + 1;
            (_type(INDEX), _id(INDEX))
        }
    }

    fn delete_record(wallet_handle: WalletHandle, type_: &str, id: &str) {
        NonSecretsUtils::delete_wallet_record(wallet_handle, type_, id).unwrap();
    }

    pub fn bench(c: &mut Criterion) {
        let wallet_handle = init_wallet();

        c.bench(
            "wallet_delete_record",
            Benchmark::new("wallet_delete_record", move |b|
                b.iter_with_setup(setup, |(type_, id): (String, String)| delete_record(wallet_handle, &type_, &id)))
                .sample_size(10));
    }
}

mod add_record {
    use super::*;

    static mut INDEX: usize = COUNT;

    fn setup() -> (String, String, String, String) {
        unsafe {
            INDEX = INDEX + 1;
            (_type(INDEX), _id(INDEX), _value(INDEX), _tags(INDEX))
        }
    }

    fn add_record(wallet_handle: WalletHandle, type_: &str, id: &str, value: &str, tags: &str) {
        NonSecretsUtils::add_wallet_record(wallet_handle, type_, id, value, Some(tags)).unwrap();
    }

    pub fn bench(c: &mut Criterion) {
        let wallet_handle = init_wallet();

        c.bench(
            "wallet_add_record",
            Benchmark::new("wallet_add_record", move |b|
                b.iter_with_setup(
                    setup, |(type_, id, value, tags): (String, String, String, String)|
                        add_record(wallet_handle, &type_, &id, &value, &tags)))
                .sample_size(10));
    }
}

mod add_record_tags {
    use super::*;

    fn setup() -> (String, String, String) {
        let (type_, id) = get_rand_key();
        (type_, id, r#"{"tag_1": "value_1", "~tag_2": "value_2"}"#.to_string())
    }

    fn add_record_tags(wallet_handle: WalletHandle, type_: &str, id: &str, tags: &str) {
        NonSecretsUtils::add_wallet_record_tags(wallet_handle, type_, id, tags).unwrap();
    }

    pub fn bench(c: &mut Criterion) {
        let wallet_handle = init_wallet();

        c.bench(
            "wallet_add_record_tags",
            Benchmark::new("wallet_add_record_tags", move |b|
                b.iter_with_setup(
                    setup,
                    |(type_, id, tags): (String, String, String)| add_record_tags(wallet_handle, &type_, &id, &tags)))
                .sample_size(10));
    }
}

mod delete_record_tags {
    use super::*;

    fn setup() -> (String, String, String) {
        let (type_, id) = get_rand_key();
        (type_, id, r#"["tag_id_1"]"#.to_string())
    }

    fn delete_record_tags(wallet_handle: WalletHandle, type_: &str, id: &str, tag_names: &str) {
        NonSecretsUtils::delete_wallet_record_tags(wallet_handle, type_, id, tag_names).unwrap();
    }

    pub fn bench(c: &mut Criterion) {
        let wallet_handle = init_wallet();

        c.bench(
            "wallet_delete_record_tags",
            Benchmark::new("wallet_delete_record_tags", move |b|
                b.iter_with_setup(
                    setup,
                    |(type_, id, tag_names): (String, String, String)| delete_record_tags(wallet_handle, &type_, &id, &tag_names)))
                .sample_size(10));
    }
}

mod search_records {
    use super::*;

    fn open_search(wallet_handle: WalletHandle, query: &str) {
        NonSecretsUtils::open_wallet_search(wallet_handle, TYPE_1, query, "{}").unwrap();
    }

    pub fn bench(c: &mut Criterion) {
        let wallet_handle = init_wallet();

        let query = r#"{}"#;

        c.bench(
            "wallet_search",
            Benchmark::new(
                "wallet_search_empty",
                move |b| b.iter(|| open_search(wallet_handle, query)),
            ).sample_size(20),
        );

        let query = r#"{
            "tag_id_1": "tag_value_10_1"
        }"#;

        c.bench(
            "wallet_search",
            Benchmark::new(
                "wallet_search_eq",
                move |b| b.iter(|| open_search(wallet_handle, query)),
            ).sample_size(20),
        );

        let query = r#"{
            "~tag_id_3": {
                "$gt": "30"
            }
        }"#;

        c.bench(
            "wallet_search",
            Benchmark::new(
                "wallet_search_gt",
                move |b| b.iter(|| open_search(wallet_handle, query)),
            ).sample_size(20),
        );

        let query = r#"{
                    "tag_id_1": {
                        "$in": ["tag_value_10_1", "tag_value_11_1", "tag_value_70_1", "tag_value_71_1"]
                    }
                }"#;

        c.bench(
            "wallet_search",
            Benchmark::new(
                "wallet_search_in",
                move |b| b.iter(|| open_search(wallet_handle, query)),
            ).sample_size(20),
        );

        let query = r#"{
            "tag_id_1": "tag_value_11_1",
            "~tag_id_3": "10"
        }"#;

        c.bench(
            "wallet_search",
            Benchmark::new(
                "wallet_search_and",
                move |b| b.iter(|| open_search(wallet_handle, query)),
            ).sample_size(20),
        );

        let query = r#"{
            "$or": [
                {"tag_id_1": "tag_value_11_1"},
                {"tag_id_3": "90"}
            ]
        }"#;

        c.bench(
            "wallet_search",
            Benchmark::new(
                "wallet_search_or",
                move |b| b.iter(|| open_search(wallet_handle, query)),
            ).sample_size(20),
        );
    }
}

pub const COUNT: usize = 1000;
pub const TYPE_1: &'static str = "type_1";
pub const TYPE_2: &'static str = "type_2";

fn _type(suffix: usize) -> String {
    if suffix % 2 != 0 { TYPE_1.to_string() } else { TYPE_2.to_string() }
}

fn _id(suffix: usize) -> String { format!("id_{}", suffix) }

fn _value(suffix: usize) -> String {
    format!("value_{}", suffix)
}

fn _tags(suffix: usize) -> String {
    json!({
            "tag_id_1":  format!("tag_value_{}_1", suffix),
            "tag_id_2":  format!("tag_value_{}_2", suffix),
            "~tag_id_3":  format!("{}", suffix),
        }).to_string()
}

fn add_records(wallet_handle: WalletHandle) {
    for i in 0..COUNT {
        NonSecretsUtils::add_wallet_record(wallet_handle,
                                           &_type(i),
                                           &_id(i),
                                           &_value(i),
                                           Some(&_tags(i))).unwrap();
    }
}

fn init_wallet() -> i32 {
    TestUtils::cleanup_storage();

    let config = json!({
            "id": format!("default-wallet_id-{}", SequenceUtils::get_next_id())
        }).to_string();

    WalletUtils::create_wallet(&config, WALLET_CREDENTIALS_RAW).unwrap();
    let wallet_handle = WalletUtils::open_wallet(&config, WALLET_CREDENTIALS_RAW).unwrap();

    add_records(wallet_handle);

    wallet_handle
}

fn get_rand_key() -> (String, String) {
    let i = rand::thread_rng().gen_range(0, COUNT);
    (_type(i), _id(i))
}

criterion_group!(benches, create::bench,
                          open::bench,
                          close::bench,
                          delete::bench,
                          get_record::bench,
                          delete_record::bench,
                          add_record::bench,
                          add_record_tags::bench,
                          delete_record_tags::bench,
                          search_records::bench);
criterion_main!(benches);
