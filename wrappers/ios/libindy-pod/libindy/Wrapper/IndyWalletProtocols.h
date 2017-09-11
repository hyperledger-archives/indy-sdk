//
//  IndyWalletProtocol.h
//  libindy
//

@protocol IndyWalletTypeProtocol;
@protocol IndyWalletImplementationProtocol;

@protocol IndyWalletProtocol <IndyWalletTypeProtocol, IndyWalletImplementationProtocol>

@end

// MARK: - Indy Wallet type protocol

@protocol IndyWalletTypeProtocol

@required
+ (id<IndyWalletProtocol>) sharedInstance;

@required
- (NSError *)createWithName:(NSString *)name
                     config:(NSString *)config
                credentials:(NSString *)credentials;

@required
- (NSError *)openWithName:(NSString *)name
                   config:(NSString *)config
            runtimeConfig:(NSString *)runtimeConfig
              credentials:(NSString *)credentials
                   handle:(IndyHandle *)handle;

@required
- (NSError *)deleteWalletWithName:(NSString *)name
                           config:(NSString *)config
                      credentials:(NSString *)credentials;

@end

// MARK: - Indy wallet implementation protocol

@protocol IndyWalletImplementationProtocol <NSObject>

@required
- (NSError *)setValue:(NSString *)value
               forKey:(NSString *)key
           withHandle:(IndyHandle)handle;

@required
- (NSError *)getValue:(NSString **)value
               forKey:(NSString *)key
           withHandle:(IndyHandle)handle;

@required
- (NSError *)getNotExpired:(IndyHandle)walletHandle
                       key:(NSString *)key
                     value:(NSString**)value;

@required
- (NSError *)list:(IndyHandle)handle
              key:(NSString *)key
       valuesJson:(NSString**)valuesJson;

@required
- (NSError *)close:(IndyHandle)handle;

@required
- (NSError *)free:(IndyHandle)handle
              str:(NSString *)str;

@end
