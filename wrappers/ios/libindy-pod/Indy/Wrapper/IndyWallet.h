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
        withImplementation:(Class<IndyWalletProtocol>)implementation
                completion:(void (^)(NSError *error)) completion;

/**
 Register Keychain Wallet type with default implementation

 @param type Wallet type name associated with provided implementation.
 @param completion Completion callback with errord code indicating result.
*/
- (void)registerIndyKeychainWalletType:(NSString *)type
                            completion:(void (^)(NSError *error)) completion;
/**
 Creates a new secure wallet with the given unique name.
 
 @param poolName   Name of the pool that corresponds to this wallet.
 @param name Name of the wallet.
 @param type Type of the wallet. Defaults to 'default'.
             Custom types can be registered with IndyWallet:registerWalletType:withImplementation:completion.
 @param config Wallet configuration json. List of supported keys are defined by wallet type.
               If NULL, then default config will be used.

 @param credentials Wallet credentials json: {
     "key": <string>
 }
 @param completion Completion callback that returns error code.
*/
- (void)createWalletWithName:(NSString *)name
                    poolName:(NSString *)poolName
                        type:(NSString *)type
                      config:(NSString *)config
                 credentials:(NSString *)credentials
                  completion:(void (^)(NSError *error)) completion;

/**
 Opens the wallet with specific name.
 
 Wallet with corresponded name must be previously created with IndyWallet::createWalletWithName method.
 
 @warning It is impossible to open wallet with the same name more than once.
 
 @param name Name of the wallet.
 @param config Runtime wallet configuration json. if NULL, then default runtime_config will be used.
 Example:
 
 @code
 {
 "freshness_time": string (optional), Amount of minutes to consider wallet value as fresh. Defaults to 24*60.
 ... List of additional supported keys are defined by wallet type.
 }
 @endcode
 
 @param credentials Wallet credentials json: {
     "key": <string>
 }
 
 @param completion Completion callback that returns error code and created handle to opened wallet to use in methods that require wallet access.
 */
- (void)openWalletWithName:(NSString *)name
             runtimeConfig:(NSString *)config
               credentials:(NSString *)credentials
                completion:(void (^)(NSError *error, IndyHandle walletHandle )) completion;

/**
 Closes opened wallet and frees allocated resources.
 
 @param walletHandle  wallet handle returned by IndyWallet::openWalletWithName.
 @param completion Completion callback that returns error code.
 */
- (void)closeWalletWithHandle:(IndyHandle)walletHandle
                   completion:(void (^)(NSError *error ))completion;

/**
 Deletes created wallet.
 
 @param walletName of the wallet to delete.
 
 @param credentials Wallet credentials json: {
     "key": <string>
 }
 @param completion Completion callback that returns error code.
 */
- (void)deleteWalletWithName:(NSString *)walletName
                 credentials:(NSString *)credentials
                  completion:(void (^)(NSError *error ))completion;

/**
 Delete all keychain wallets from Keychain.
 */
- (void)cleanupIndyKeychainWallet;

@end


