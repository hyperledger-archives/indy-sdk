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

//@property (strong, readwrite) NSMutableDictionary *valuesPointers;

@property (strong, readwrite) NSMutableSet *valuesSet;

- (void)freeString:(NSString *)string;
- (void)retainString:(NSString**)valueString;

- (BOOL)setupCustomWalletImplementation:(Class<IndyWalletProtocol>)implementation;
- (void)removeCustomWalletImplementation;

@end

#if __cplusplus
extern "C" {
#endif
    
// MARK: - IndyKeychainWallet C-callbacks

extern indy_error_t IndyKeychainWalletCreateCallback(const char* name,
                                                 const char* config,
                                                 const char* credentials);

extern indy_error_t IndyKeychainWalletOpenCallback(const char* name,
                                               const char* config,
                                               const char* runtime_config,
                                               const char* credentials,
                                               indy_handle_t* handle);

extern indy_error_t IndyKeychainWalletSetCallback(indy_handle_t handle,
                                              const char* key,
                                              const char* value);

extern indy_error_t IndyKeychainWalletGetCallback(indy_handle_t handle,
                                              const char* key,
                                              const char ** const value_ptr);
    
extern indy_error_t IndyKeychainWalletGetNotExpiredCallback(indy_handle_t handle,
                                                        const char* key,
                                                        const char ** const value_ptr);

extern indy_error_t IndyKeychainWalletListCallback(indy_handle_t handle,
                                               const char* key,
                                               const char ** const values_json_ptr);

extern indy_error_t IndyKeychainWalletCloseCallback(indy_handle_t handle);

extern indy_error_t IndyKeychainWalletDeleteCallback(const char* name,
                                                 const char* config,
                                                 const char* credentials);

extern indy_error_t IndyKeychainWalletFreeCallback(indy_handle_t handle, const char* str);
    
// MARK: - Custom Wallet c-callbacks
    
    
extern indy_error_t CustomWalletCreateCallback(const char* name,
                                               const char* config,
                                               const char* credentials);
    
extern indy_error_t CustomWalletOpenCallback(const char* name,
                                             const char* config,
                                             const char* runtime_config,
                                             const char* credentials,
                                             indy_handle_t* handle);
    
extern indy_error_t CustomWalletSetCallback(indy_handle_t handle,
                                            const char* key,
                                            const char* value);
    
extern indy_error_t CustomWalletGetCallback(indy_handle_t handle,
                                            const char* key,
                                            const char ** const value_ptr);
    
extern indy_error_t CustomWalletGetNotExpiredCallback(indy_handle_t handle,
                                                      const char* key,
                                                      const char ** const value_ptr);
    
extern indy_error_t CustomWalletListCallback(indy_handle_t handle,
                                             const char* key,
                                             const char ** const values_json_ptr);
    
extern indy_error_t CustomWalletCloseCallback(indy_handle_t handle);
    
extern indy_error_t CustomWalletDeleteCallback(const char* name,
                                               const char* config,
                                               const char* credentials);
    
extern indy_error_t CustomWalletFreeCallback(indy_handle_t handle, const char* str);
    
#if __cplusplus
}   // Extern C
#endif
