use std::time::Duration;

pub const DEFAULT_CREDENTIALS: &str = r#"{"key":"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY", "key_derivation_method":"RAW"}"#;
pub const DID: &str = "8wZcEriaNLNKtteJvx7f8i";
pub const DID_1: &str = "VsKV7grR1BUE29mG2Fm2kX";
pub const DID_TRUSTEE: &str = "V4SGRU86Z58d6TV7PBUe6f";
pub const EXPORT_KEY: &str = "export_key";
pub const METADATA: &str = "some_metadata";
pub const MY1_SEED: &str = "00000000000000000000000000000My1";
pub const PROTOCOL_VERSION: i32 = 2;
pub const SEED_1: &str = "00000000000000000000000000000My1";
pub const TRUSTEE_SEED: &str = "000000000000000000000000Trustee1";
pub const VERKEY_1: &str = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";
pub const VERKEY_ABV_1: &str = "~HYwqs2vrTc8Tn4uBV7NBTe";
pub const VERKEY_TRUSTEE: &str = "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";
pub const VALID_TIMEOUT: Duration = Duration::from_secs(5);
pub const INVALID_TIMEOUT: Duration = Duration::from_micros(1);
