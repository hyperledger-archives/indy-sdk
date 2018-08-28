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
extern crate indy_crypto;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[path = "../tests/utils/mod.rs"]
#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::test::TestUtils;
use utils::constants::*;

use criterion::{Criterion, Benchmark};

mod create {
    use super::*;

    fn setup() {
        TestUtils::cleanup_storage();
    }

    fn create_argon2i_mod((): ()) {
        WalletUtils::create_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS_ARAGON2I_MOD).unwrap();
    }

    fn create_argon2i_int((): ()) {
        WalletUtils::create_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS_ARAGON2I_INT).unwrap();
    }

    fn create_raw((): ()) {
        WalletUtils::create_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS_RAW).unwrap();
    }

    pub fn bench(c: &mut Criterion) {
        c.bench(
            "wallet_create",
            Benchmark::new("create_argon2i_mod", |b| b.iter_with_setup(setup, create_argon2i_mod))
                .sample_size(10));

        c.bench(
            "wallet_create",
            Benchmark::new("create_argon2i_int", |b| b.iter_with_setup(setup, create_argon2i_int))
                .sample_size(20));

        c.bench(
            "wallet_create",
            Benchmark::new("create_raw", |b| b.iter_with_setup(setup, create_raw))
                .sample_size(50));
    }
}

criterion_group!(benches, create::bench);
criterion_main!(benches);