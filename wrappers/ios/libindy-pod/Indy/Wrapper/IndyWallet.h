//
//  IndyWallet.h
//  libindy
//

#import <Foundation/Foundation.h>
#import "IndyTypes.h"
#import "IndyWalletProtocols.h"


@interface IndyWallet : NSObject

+ (IndyWallet *)sharedInstance;

/**
 
 Registers custom wallet implementation.
 
 @warning Only one custom implementation can be registered. Any other will be ignored.
 
 @param type
        Wallet type name associated with provided implementation.
 @param implementation 
        Class which conforms to protocol IndyWalletProtocol.
 @param completion Completion callback with errord code indicating result.
*/
- (void)registerWalletType:(NSString *)type
        withImplementation:(Class <IndyWalletProtocol>)implementation
                completion:(void (^)(NSError *error))completion;

/**
 Register Keychain Wallet type with default implementation

 @param type Wallet type name associated with provided implementation.
 @param completion Completion callback with errord code indicating result.
*/
- (void)registerIndyKeychainWalletType:(NSString *)type
                            completion:(void (^)(NSError *error))completion;

/**
 Creates a new secure wallet with the given unique name.

 @param config Wallet configuration json.
 {
   "id": string, Identifier of the wallet.
         Configured storage uses this identifier to lookup exact wallet data placement.
   "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
                  'Default' storage type allows to store wallet data in the local file.
                  Custom storage types can be registered with indy_register_wallet_storage call.
   "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
                     Can be optional if storage supports default configuration.
                     For 'default' storage type configuration is:
   {
     "path": optional<string>, Path to the directory with wallet files.
             Defaults to $HOME/.indy_client/wallets.
             Wallet will be stored in the file {path}/{id}/sqlite.db
   }
 }

 @param credentials Wallet credentials json
 {
   "key": string, Passphrase used to derive wallet master key
   "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                          Can be optional if storage supports default configuration.
                          For 'default' storage type should be empty.
 }
 @param completion Completion callback that returns error code.
*/
- (void)createWalletWithConfig:(NSString *)config
                   credentials:(NSString *)credentials
                    completion:(void (^)(NSError *error))completion;

/**
 Opens the wallet with specific name.
 
 Wallet with corresponded name must be previously created with IndyWallet::createWalletWithName method.
 
 @warning It is impossible to open wallet with the same name more than once.
 
 @param config Wallet configuration json.
 {
   "id": string, Identifier of the wallet.
         Configured storage uses this identifier to lookup exact wallet data placement.
   "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
                  'Default' storage type allows to store wallet data in the local file.
                  Custom storage types can be registered with indy_register_wallet_storage call.
   "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
                     Can be optional if storage supports default configuration.
                     For 'default' storage type configuration is:
   {
     "path": optional<string>, Path to the directory with wallet files.
             Defaults to $HOME/.indy_client/wallets.
             Wallet will be stored in the file {path}/{id}/sqlite.db
   }
 }

 @param credentials Wallet credentials json
 {
   "key": string, Passphrase used to derive wallet master key
   "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                          Can be optional if storage supports default configuration.
                          For 'default' storage type should be empty.
 }
 
 @param completion Completion callback that returns error code and created handle to opened wallet to use in methods that require wallet access.
 */
- (void)openWalletWithConfig:(NSString *)config
               credentials:(NSString *)credentials
                completion:(void (^)(NSError *error, IndyHandle walletHandle))completion;

/**
 Closes opened wallet and frees allocated resources.
 
 @param walletHandle  wallet handle returned by IndyWallet::openWalletWithName.
 @param completion Completion callback that returns error code.
 */
- (void)closeWalletWithHandle:(IndyHandle)walletHandle
                   completion:(void (^)(NSError *error))completion;

/**
 Deletes created wallet.
 
 @param config Wallet configuration json.
 {
   "id": string, Identifier of the wallet.
         Configured storage uses this identifier to lookup exact wallet data placement.
   "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
                  'Default' storage type allows to store wallet data in the local file.
                  Custom storage types can be registered with indy_register_wallet_storage call.
   "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
                     Can be optional if storage supports default configuration.
                     For 'default' storage type configuration is:
   {
     "path": optional<string>, Path to the directory with wallet files.
             Defaults to $HOME/.indy_client/wallets.
             Wallet will be stored in the file {path}/{id}/sqlite.db
   }
 }

 @param credentials Wallet credentials json
 {
   "key": string, Passphrase used to derive wallet master key
   "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                          Can be optional if storage supports default configuration.
                          For 'default' storage type should be empty.
 }
 @param completion Completion callback that returns error code.
 */
- (void)deleteWalletWithConfig:(NSString *)config
                 credentials:(NSString *)credentials
                  completion:(void (^)(NSError *error))completion;


/**
 Exports opened wallet.

 Note this endpoint is EXPERIMENTAL. Function signature and behavior may change
 in the future releases.

 @param walletHandle  wallet handle returned by IndyWallet::openWalletWithName.
 @param exportConfigJson  JSON containing settings for input operation.
   {
     "path": path of the file that contains exported wallet content
     "key": passphrase used to export key
   }
 @param completion Completion callback that returns error code.
 */
- (void)exportWalletWithHandle:(IndyHandle)walletHandle
              exportConfigJson:(NSString *)exportConfigJson
                    completion:(void (^)(NSError *error))completion;

/**
 Creates a new secure wallet with the given unique name and then imports its content
 according to fields provided in import_config
 This can be seen as an indy_create_wallet call with additional content import

 Note this endpoint is EXPERIMENTAL. Function signature and behavior may change
 in the future releases.

 @param config Wallet configuration json.
 {
   "id": string, Identifier of the wallet.
         Configured storage uses this identifier to lookup exact wallet data placement.
   "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
                  'Default' storage type allows to store wallet data in the local file.
                  Custom storage types can be registered with indy_register_wallet_storage call.
   "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
                     Can be optional if storage supports default configuration.
                     For 'default' storage type configuration is:
   {
     "path": optional<string>, Path to the directory with wallet files.
             Defaults to $HOME/.indy_client/wallets.
             Wallet will be stored in the file {path}/{id}/sqlite.db
   }
 }

 @param credentials Wallet credentials json
 {
   "key": string, Passphrase used to derive wallet master key
   "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                          Can be optional if storage supports default configuration.
                          For 'default' storage type should be empty.
 }
 @param importConfigJson  JSON containing settings for input operation.
   {
     "path": path of the file that contains exported wallet content
     "key": passphrase used to export key
   }
 @param completion Completion callback that returns error code.
*/
- (void)importWalletWithConfig:(NSString *)config
                 credentials:(NSString *)credentials
            importConfigJson:(NSString *)importConfigJson
                  completion:(void (^)(NSError *error))completion;

/**
 Delete all keychain wallets from Keychain.
 */
- (void)cleanupIndyKeychainWallet;

@end


