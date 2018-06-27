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

File contains encrypted header and encrypted list of records using **ChaCha20-Poly1305-IETF** cypher in blocks per 1024 bytes (to allow streaming).
This is similar encyption as recomended in libsodium secretstream but secretstream was not available in Rust wrapper.

#### Header
Header contains version of file format, time of creation, encryption method, nonce and salt used for encryption.

 * `header_length`: 2B unsigned big endian integer (length of encrypted header)
 * `version`: 4B unsigned big endian integer
 * `time`: 8B unsigned big endian integer
 * `encryption_method_length`: 2B unsigned big endian integer
 * `encryption_method`: UTF-8 string of `encryption_method_length` length
 * `nonce_length`: 2B unsigned big endian integer
 * `nonce`: bytes of length `nonce_length`, containing nonce for encryption of body. Nonce is then incremented for every block.
 * `salt_length`: 2B unsigned big endian integer
 * `salt`: bytes of length `salt_length`, containing salt used for deriving export key
 * `header_hash`: 32B **SHA-256** hash of the header.

#### Body

Body format is list of records in falowing format:

 * `record_length`: 4B unsigned big endian integer
 * `type_length`: 4B unsigned big endian integer
 * `type`: UTF-8 string of `type_length` bytes
 * `name_length`: 4B unsigned big endian integer
 * `name`: UTF-8 string of `name_length` bytes
 * `value_length`: 4B unsigned big endian integer
 * `value`: UTF-8 string of `value_length` bytes
 * `tags_json_length`: 4B unsigned big endian integer
 * `tags_json`: UTF-8 string of `tags_json_length` bytes
 