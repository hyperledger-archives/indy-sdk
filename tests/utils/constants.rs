pub const TRUSTEE_SEED: &str = "000000000000000000000000Trustee1";
pub const MY1_SEED: &str = "00000000000000000000000000000My1";
pub const MY2_SEED: &str = "00000000000000000000000000000My2";
pub const VERKEY: &str = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
pub const VERKEY_MY1: &str = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";
pub const VERKEY_MY2: &str = "kqa2HyagzfMAq42H5f9u3UMwnSBPQx2QfrSyXbUPxMn";
pub const VERKEY_TRUSTEE: &str = "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";
pub const INVALID_VERKEY: &str = "CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW";
pub const DID: &str = "8wZcEriaNLNKtteJvx7f8i";
pub const DID_MY1: &str = "VsKV7grR1BUE29mG2Fm2kX";
pub const DID_MY2: &str = "2PRyVHmkXQnQzJQKxHxnXC";
pub const DID_TRUSTEE: &str = "V4SGRU86Z58d6TV7PBUe6f";
pub const INVALID_DID: &str = "invalid_base58string";
pub const IDENTITY_JSON_TEMPLATE: &str = r#"{"did":"{}","verkey":"{}"}"#;
pub const MESSAGE: &[u8] = b"{\"reqId\":1496822211362017764}";
pub const SCHEMA_DATA: &str = r#"{"id":"id","name":"gvt","version":"1.0","attrNames":["name"],"ver":"1.0"}"#;
pub const POOL: &str = "Pool1";
pub const WALLET: &str = "Wallet1";
pub const TYPE: &str = "default";
pub const METADATA: &str = "some metadata";
pub const ENDPOINT: &str = "127.0.0.1:9700";
pub const CRYPTO_TYPE: &str = "ed25519";
pub const SIGNATURE: [i8; 64] = [20, -65, 100, -43, 101, 12, -59, -58, -53, 49, 89, -36, -51, -64, -32, -35, 97, 77, -36, -66, 90, 60, -114, 23, 16, -16, -67, -127, 45, -108, -11, 8, 102, 95, 95, -7, 100, 89, 41, -29, -43, 25, 100, 1, -24, -68, -11, -21, -70, 21, 52, -80, -20, 11, 99, 70, -101, -97, 89, -41, -59, -17, -118, 5];
pub const ENCRYPTED_MESSAGE: [i8; 45] = [-105, 30, 89, 75, 76, 28, -59, -45, 105, -46, 20, 124, -85, -13, 109, 29, -88, -82, -8, -6, -50, -84, -53, -48, -49, 56, 124, 114, 82, 126, 74, 99, -72, -78, -117, 96, 60, 119, 50, -40, 121, 21, 57, -68, 89];
pub const NONCE: [i8; 24] = [-14, 102, -41, -57, 1, 4, 75, -46, -91, 87, 14, 41, -39, 48, 42, -126, -121, 84, -58, 59, -27, 51, -32, -23];
pub const DEFAULT_CRED_DEF_CONFIG: &str = r#"{"support_revocation":false}"#;
pub const TAG: &str = "tag1";
pub const GVT_SCHEMA_NAME: &str = "gvt";
pub const XYZ_SCHEMA_NAME: &str = "xyz";
pub const SCHEMA_VERSION: &str = "1.0";
pub const GVT_SCHEMA_ATTRIBUTES: &str = r#"["name", "age", "sex", "height"]"#;
pub const XYZ_SCHEMA_ATTRIBUTES: &str = r#"["status", "period"]"#;
pub const REVOC_REG_TYPE: &str = "CL_ACCUM";
pub const SIGNATURE_TYPE: &str = "CL";
pub const REV_CRED_DEF_CONFIG: &str = r#"{"support_revocation":true}"#;
pub const GVT_CRED_VALUES: &str = r#"{
			"        "sex": {"raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050"},
			"        "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
			"        "height": {"raw": "175", "encoded": "175"},
			"        "age": {"raw": "28", "encoded": "28"}
			"    }"#;
pub const CREDENTIALS: &str = r#"{"key": "key"}"#;
pub const PROTOCOL_VERSION: i32 = 2;
pub const EXPORT_KEY: &str = "export_key";
