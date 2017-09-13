//
//  IndyWalletCallbacks.m
//  libindy
//

#import <Foundation/Foundation.h>
#import "IndyWalletCallbacks.h"
#import "KeychainWallet.h"
#import "indy_core.h"

@interface IndyWalletCallbacks ()

@property (strong, readwrite) NSRecursiveLock *globalLock;

@property (strong, readwrite) Class<IndyWalletProtocol> keychainWalletImplementation;

@property (strong, readwrite) Class<IndyWalletProtocol> customWalletImplementation;

/**
 Dictionary of [pointer: NSString] to prevent system from deallocating values strings from memory
 */
@property (strong, readwrite) NSMutableDictionary *valuesPointers;

@property (strong, readwrite) NSMutableSet *valuesSet;

@end

@implementation IndyWalletCallbacks

+ (IndyWalletCallbacks *)sharedInstance
{
    static IndyWalletCallbacks *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [IndyWalletCallbacks new];
        instance.valuesPointers = [NSMutableDictionary new];
        instance.globalLock = [NSRecursiveLock new];
        
        instance.keychainWalletImplementation = [KeychainWallet class];
        
    });
    
    return instance;
}

/**
 Remove refecence to NSString with pointer
 */
- (void)freeString:(NSString *)string
{
    [self.valuesSet removeObject:string];
    //[self.valuesPointers removeObjectForKey:pointer];
}

- (void)retainString:(NSString *__autoreleasing *)valueString
{
    @synchronized (self.globalLock)
    {
        //const char *const * valuePointer = (const char *const *)[*valueString UTF8String];
        //NSValue *value = [NSValue valueWithPointer:valuePointer];
        //self.valuesPointers[value] = [*valueString copy];
        
        [self.valuesSet addObject:[*valueString copy]];
    }
}

- (BOOL)setupCustomWalletImplementation:(Class<IndyWalletProtocol>)implementation
{
    if (self.customWalletImplementation != nil)
    {
        return NO;
    }
    
    self.customWalletImplementation = implementation;
    return true;
}

- (void)removeCustomWalletImplementation
{
    self.customWalletImplementation = nil;
}

@end

// MARK: - C KeychainWallet callbacks

indy_error_t KeychainWalletCreateCallback(const char* name,
                                          const char* config,
                                          const char* credentials)
{
    NSString *walletName = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *walletConfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *walletCredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] keychainWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Keychain Wallet Implementation not found for name: %@", walletName);
        return WalletUnknownTypeError;
    }
    
    [[implementation sharedInstance] createWithName:walletName
                                             config:walletConfig
                                        credentials:walletCredentials];
    return Success;
}

indy_error_t KeychainWalletOpenCallback(const char* name,
                                        const char* config,
                                        const char* runtime_config,
                                        const char* credentials,
                                        indy_handle_t* handle)
{
    NSString *walletName = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *walletConfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *walletRuntimeConfig = (runtime_config != NULL) ? [NSString stringWithUTF8String:runtime_config] : nil;
    NSString *walletCredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] keychainWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for name: %@", walletName);
        return WalletUnknownTypeError;
    }
    
    IndyHandle walletHandle = 0;
    NSError *res;
    res = [[implementation sharedInstance] openWithName:walletName
                                                 config:walletConfig
                                          runtimeConfig:walletRuntimeConfig
                                            credentials:walletCredentials
                                                 handle:&walletHandle];
    
    if (res.code != Success)
    {
        return (indy_error_t)res.code;
    }
    
    if (handle) { *handle = walletHandle; }
    
    return Success;
}

indy_error_t KeychainWalletSetCallback(indy_handle_t handle,
                                       const char* key,
                                       const char* value)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    NSString *xvalue = (value != NULL) ? [NSString stringWithUTF8String:value] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] keychainWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }

    NSError *res;
    res = [[implementation sharedInstance] setValue:xvalue
                                             forKey:xkey
                                         withHandle:handle];
    
    return (indy_error_t)res.code;
}

indy_error_t KeychainWalletGetCallback(indy_handle_t handle,
                                       const char* key,
                                       const char *const *value_ptr)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] keychainWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    NSString *valueString = [NSString new];
    res = [[implementation sharedInstance] getValue:&valueString
                                             forKey:xkey
                                         withHandle:handle];
    
    if (res.code != Success)
    {
        return (indy_error_t)res.code;
    }
 
    value_ptr = (const char *const *)[valueString UTF8String];
    [[IndyWalletCallbacks sharedInstance] retainString:&valueString];
    
    return Success;
}

indy_error_t KeychainWalletGetNotExpiredCallback(indy_handle_t handle,
                                                 const char* key,
                                                 const char *const *value_ptr)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] keychainWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    NSString *valueString = [NSString new];
    res = [[implementation sharedInstance] getNotExpired:handle
                                                     key:xkey
                                                   value:&valueString];
    
    if (res.code != Success)
    {
        return (indy_error_t)res.code;
    }
    
    [[IndyWalletCallbacks sharedInstance] retainString:&valueString];
    
    value_ptr = (const char * const *)[valueString UTF8String];
    
    return Success;
}

indy_error_t KeychainWalletListCallback(indy_handle_t handle,
                                        const char* key,
                                        const char *const *values_json_ptr)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] keychainWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    NSString *valuesJsonString = [NSString new];
    res = [[implementation sharedInstance] list:handle
                                            key:xkey
                                     valuesJson:&valuesJsonString];
    
    if (res.code != Success)
    {
        return (indy_error_t)res.code;
    }
    
    [[IndyWalletCallbacks sharedInstance] retainString:&valuesJsonString];
    
    values_json_ptr = (const char * const *)[valuesJsonString UTF8String];
    
    return Success;
}

indy_error_t KeychainWalletCloseCallback(indy_handle_t handle)
{
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] keychainWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    res = [[implementation sharedInstance] close:handle];

    return (indy_error_t)res.code;
}

indy_error_t KeychainWalletDeleteCallback(const char* name,
                                          const char* config,
                                          const char* credentials)
{
    NSString *xname = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *xconfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *xcredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;

    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] keychainWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with name: %@", xname);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    res = [[implementation sharedInstance] deleteWalletWithName:xname
                                                         config:xconfig
                                                    credentials:xcredentials];
    
    return (indy_error_t)res.code;

}


indy_error_t KeychainWalletFreeCallback(indy_handle_t handle, const char* str)
{
    [[IndyWalletCallbacks sharedInstance] freeString:[NSString stringWithUTF8String:str]];
    free((void*)str);
    return Success;
}

// MARK: - C Custom Wallet callbacks

indy_error_t CustomWalletCreateCallback(const char* name,
                                        const char* config,
                                        const char* credentials)
{
    NSString *walletName = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *walletConfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *walletCredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] customWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for name: %@", walletName);
        return WalletUnknownTypeError;
    }
    
    [[implementation sharedInstance] createWithName:walletName
                                             config:walletConfig
                                        credentials:walletCredentials];
    return Success;
}

indy_error_t CustomWalletOpenCallback(const char* name,
                                      const char* config,
                                      const char* runtime_config,
                                      const char* credentials,
                                      indy_handle_t* handle)
{
    NSString *walletName = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *walletConfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *walletRuntimeConfig = (runtime_config != NULL) ? [NSString stringWithUTF8String:runtime_config] : nil;
    NSString *walletCredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] customWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for name: %@", walletName);
        return WalletUnknownTypeError;
    }
    
    IndyHandle walletHandle = 0;
    NSError *res;
    res = [[implementation sharedInstance] openWithName:walletName
                                                 config:walletConfig
                                          runtimeConfig:walletRuntimeConfig
                                            credentials:walletCredentials
                                                 handle:&walletHandle];
    
    if (res.code != Success)
    {
        return (indy_error_t)res.code;
    }
    
    if (handle) { *handle = walletHandle; }
    
    return Success;
}

indy_error_t CustomWalletSetCallback(indy_handle_t handle,
                                     const char* key,
                                     const char* value)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    NSString *xvalue = (value != NULL) ? [NSString stringWithUTF8String:value] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] customWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    res = [[implementation sharedInstance] setValue:xvalue
                                             forKey:xkey
                                         withHandle:handle];
    
    return (indy_error_t)res.code;
}

indy_error_t CustomWalletGetCallback(indy_handle_t handle,
                                     const char* key,
                                     const char *const *value_ptr)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] customWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    NSString *valueString = [NSString new];
    res = [[implementation sharedInstance] getValue:&valueString
                                             forKey:xkey
                                         withHandle:handle];
    
    if (res.code != Success)
    {
        return (indy_error_t)res.code;
    }
    
    value_ptr = (const char *const *)[valueString UTF8String];
    [[IndyWalletCallbacks sharedInstance] retainString:&valueString];
    
    return Success;
}

indy_error_t CustomWalletGetNotExpiredCallback(indy_handle_t handle,
                                               const char* key,
                                               const char *const *value_ptr)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] customWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    NSString *valueString = [NSString new];
    res = [[implementation sharedInstance] getNotExpired:handle
                                                     key:xkey
                                                   value:&valueString];
    
    if (res.code != Success)
    {
        return (indy_error_t)res.code;
    }
    
    [[IndyWalletCallbacks sharedInstance] retainString:&valueString];
    
    value_ptr = (const char * const *)[valueString UTF8String];
    
    return Success;
}

indy_error_t CustomWalletListCallback(indy_handle_t handle,
                                      const char* key,
                                      const char *const *values_json_ptr)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] customWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    NSString *valuesJsonString = [NSString new];
    res = [[implementation sharedInstance] list:handle
                                            key:xkey
                                     valuesJson:&valuesJsonString];
    
    if (res.code != Success)
    {
        return (indy_error_t)res.code;
    }
    
    [[IndyWalletCallbacks sharedInstance] retainString:&valuesJsonString];
    
    values_json_ptr = (const char * const *)[valuesJsonString UTF8String];
    
    return Success;
}

indy_error_t CustomWalletCloseCallback(indy_handle_t handle)
{
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] customWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    res = [[implementation sharedInstance] close:handle];
    
    return (indy_error_t)res.code;
}

indy_error_t CustomWalletDeleteCallback(const char* name,
                                        const char* config,
                                        const char* credentials)
{
    NSString *xname = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *xconfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *xcredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;
    
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] customWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with name: %@", xname);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    res = [[implementation sharedInstance] deleteWalletWithName:xname
                                                         config:xconfig
                                                    credentials:xcredentials];
    
    return (indy_error_t)res.code;
    
}


indy_error_t CustomWalletFreeCallback(indy_handle_t handle, const char* str)
{
    [[IndyWalletCallbacks sharedInstance] freeString:[NSString stringWithUTF8String:str]];
    //free((void*)str);
    return Success;
}

