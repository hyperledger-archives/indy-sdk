//
//  IndyWalletCallbacks.h
//  libindy
//
//  Created by Anastasia Tarasova on 01/09/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "indy_core.h"
#import "IndyTypes.h"
#import "IndyWalletProtocols.h"

@interface IndyWalletCallbacks : NSObject

+ (IndyWalletCallbacks*) sharedInstance;

/**
 Add wallet type and calls to dictionary [type: methods]
 */
- (void)addWalletType:(NSString *)type withImplementation:(id<IndyWalletProtocol>)implementation;
- (void)removeWalletType:(NSString *)type;


- (void)addWalletName:(NSString *)name forRegisteredWalletType:(NSString *)walletType;
- (void)removeWalletName:(NSString *)name;


- (void)addWallethandle:(IndyHandle)handle forRegisteredWalletType:(NSString *)walletType;
- (void)removeWalletHandle:(IndyHandle)handle;


- (id<IndyWalletProtocol>)getWalletImplementationByHandle:(IndyHandle)handle;
- (id<IndyWalletProtocol>)getWalletImplementationByName:(NSString *) name;

- (void)retainString:(NSString**)valueString;
- (void)freeStringWithPointer:(NSValue *)pointer;

@end

#if __cplusplus
extern "C" {
#endif

extern indy_error_t IndyWalletCreateCallback(const char* name,
                                             const char* config,
                                             const char* credentials);

extern indy_error_t IndyWalletOpenCallback(const char* name,
                                           const char* config,
                                           const char* runtime_config,
                                           const char* credentials,
                                           indy_handle_t* handle);

extern indy_error_t IndyWalletSetCallback(indy_handle_t handle,
                                          const char* key,
                                          const char* value);

extern indy_error_t IndyWalletGetCallback(indy_handle_t handle,
                                          const char* key,
                                          const char *const *value_ptr);

extern indy_error_t IndyWalletGetNotExpiredCallback(indy_handle_t handle,
                                                    const char* key,
                                                    const char *const *value_ptr);

extern indy_error_t IndyWalletListCallback(indy_handle_t handle,
                                           const char* key,
                                           const char *const *values_json_ptr);

extern indy_error_t IndyWalletCloseCallback(indy_handle_t handle);

extern indy_error_t IndyWalletDeleteCallback(const char* name,
                                             const char* config,
                                             const char* credentials);

extern indy_error_t IndyWalletFreeCallback(indy_handle_t handle, const char* str);
    
#if __cplusplus
}   // Extern C
#endif
