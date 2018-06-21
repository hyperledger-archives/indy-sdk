//
//  IndyWalletProtocol.h
//  libindy
//

// MARK: - Indy Wallet type protocol

@protocol IndyWalletProtocol

@required
/**
 Singleton for wallets management.
 */
+ (id<IndyWalletProtocol>)sharedInstance;

@required
/**
 Remove all wallet instances, clean storage.
 */
- (void)cleanup;

@required
/**
 Create wallet.
 */
- (NSError *)createWithName:(NSString *)name
                     config:(NSString *)config
                credentials:(NSString *)credentials;

@required
/**
 Open wallet and return new wallet handle.
 
 @param handle Create and return wallet handle.
 */
- (NSError *)openWithName:(NSString *)name
                   config:(NSString *)config
            runtimeConfig:(NSString *)runtimeConfig
              credentials:(NSString *)credentials
                   handle:(IndyHandle *)handle;

@required
- (NSError *)deleteWalletWithName:(NSString *)name
                           config:(NSString *)config
                      credentials:(NSString *)credentials;

@required
/**
 Set pair key:value to wallet with handle.
 
 @param key Key.
 @param value Value.
 @param handle Wallet handle.
 */
- (NSError *)setValue:(NSString *)value
               forKey:(NSString *)key
           withHandle:(IndyHandle)handle;

@required
/**
 Get value for key from wallet with handle.
 */
- (NSError *)getValue:(NSString **)value
               forKey:(NSString *)key
           withHandle:(IndyHandle)handle;

@required
/**
 Get not expired value for key from wallet with handle.
 
 @handle Wallet handle.
 */
- (NSError *)getNotExpired:(IndyHandle)walletHandle
                       key:(NSString *)key
                     value:(NSString**)value;

@required
/**
 Get values for key from wallet with handle in format:
 
 @code {"values":[{"key":"", "value":""}, {"key":"", "value":""}]}
 @endcode
 
 @warning If no values are found, return {"values":[]}
 */
- (NSError *)list:(IndyHandle)handle
              key:(NSString *)key
       valuesJson:(NSString**)valuesJson;

@required
/**
 Close wallet with handle
 */
- (NSError *)close:(IndyHandle)handle;

@end
