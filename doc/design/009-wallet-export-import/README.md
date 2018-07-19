# Wallet Export/Import Design

Currently all encryption of the wallet is performed in libindy. Storage implementations may be plugged in.
This design proposes portable export/import functionality so the "lock-in" of the user to the software
which created the wallet is avoided.

## Goals and ideas

* Alow users to export their wallets so the can do the backup  or move their secret data to different agency or different device.
  * Export file will be encrypted with export key.
  * Export should contain the whole wallet data (including secrets).
  * Export should be done in a streaming way so big wallets may be exported on machines with conservative memory.
* Alow users to import exported wallet.
  * Import is alowed only on empty wallet - Therefore import is done with one create + import operation.
  * User should provide key used for export, so export file may be decrypted.
  * User should provide new master key used for opening newly created wallet from import.
  * Import should be done in a streaming way so big wallets may be imported on machines with conservative memory.
* Export/Import should work for all storage implementations.
* Expose two public API functions, for export and for create + import wallet.

## Public API

```Rust
/// Exports opened wallet's content using key and path provided in export_config_json
///
/// #Params
/// command_handle: command handle to map callback to caller context
/// wallet_handle: wallet handle (created by open_wallet)
/// export_config_json: JSON containing settings for input operation.
///   {
///     "path": path of the file in which the wallet will be exported
///     "key": passphrase used to derive export key
///   }
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Wallet*
extern pub fn indy_error_t indy_export_wallet(command_handle: i32,
                                              wallet_handle: i32,
                                              export_config_json: *const c_char,
                                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {}

/// Creates a new secure wallet with the given unique name and then imports its content
/// according to fields provided in import_config
///
/// #Params
/// command_handle: command handle to map callback to caller context
/// pool_name: Name of the pool that corresponds to this wallet
/// name: Name of the wallet
/// storage_type(optional): Type of the wallet storage. Defaults to 'default'.
///                  Custom storage types can be registered with indy_register_wallet_storage call.
/// config(optional): Wallet configuration json.
///   {
///       "storage": <object>  List of supported keys are defined by wallet type.
///   }
/// credentials: Wallet credentials json
///   {
///       "key": string,
///       "storage": Optional<object>  List of supported keys are defined by wallet type.
///
///   }
/// import_config_json: JSON containing settings for input operation.
///   {
///     "path": path of the file that contains exported wallet content
///     "key": passphrase used to derive export key
///   }
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Wallet*
extern pub fn indy_import_wallet(command_handle: i32,
                                 pool_name: *const c_char,
                                 name: *const c_char,
                                 storage_type: *const c_char,
                                 config: *const c_char,
                                 credentials: *const c_char,
                                 import_config_json: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {}
```

## Deriving the key from passphrase

For deriving keys from passphrase **Argon2** memory-hard function is used with random salt.

## File format

--- plain stream ---

* `header_length`: length of the serialized header as 4b unsigned little endian integer
* `header`: MessagePack serialized header entity

--- encrypted stream ---

* `header_hash`: 32B **SHA-256** hash of the header.
* `record1_length`: length the serialized record as 4b unsigned little endian integer
* `record1`: MessagePack serialized record entity
* ...
* `recordN_length`: length the serialized record as 4b unsigned little endian integer
* `recordN`: MessagePack serialized record entity
* `STOP`: 4 zero bytes. Allows to make sure that there was no truncation of export file

Where:

```Rust
pub struct Header {
    pub encryption_method: EncryptionMethod, // Method of encryption for encrypted stram
    pub time: u64, // Export time in seconds from UNIX Epoch
    pub version: u32, // Version of header
}

pub enum EncryptionMethod {
    ChaCha20Poly1305IETF { // **ChaCha20-Poly1305-IETF** cypher in blocks per chunk_size bytes
        salt: Vec<u8>,  // pwhash_argon2i13::Salt as bytes. Random salt used for deriving of key from passphrase
        nonce: Vec<u8>, // chacha20poly1305_ietf::Nonce as bytes. Random start nonce. We increment nonce for each chunk to be sure in export file consistency
        chunk_size: usize, // size of encrypted chunk
    },
}

// Note that we use externally tagged enum serialization and header will be represented as:
//
// {
//   "encryption_method": {
//     "ChaCha20Poly1305IETF": {
//       "salt": ..,
//       "nonce": ..,
//       "chunk_size": ..,
//     },
//   },
//   "time": ..,
//   "version": ..,
// }

pub struct Record {
    #[serde(rename = "type")]
    pub type_: String, // Wallet record type
    pub id: String, // Wallet record id
    pub value: String, // Wallet record value
    pub tags: HashMap<String, String>, // Wallet record tags
}
```

The only supported from the beginning encryption method is **ChaCha20-Poly1305-IETF** cypher in blocks per 1024 bytes (to allow streaming).
This is similar encryption as recommended in libsodium secretstream but secretstream was not available in Rust wrapper.
Random salt used for deriving of key from passphrase. We increment nonce for each block to be sure in export file consistency.
Also we use STOP message in encrypted stream that allows to make sure that there was no truncation of export file.