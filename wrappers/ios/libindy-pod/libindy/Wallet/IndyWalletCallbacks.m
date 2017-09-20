//
//  IndyWalletCallbacks.m
//  libindy
//

#import <Foundation/Foundation.h>
#import "IndyWalletCallbacks.h"
#import "IndyKeychainWallet.h"
#import "indy_core.h"

@interface IndyWalletCallbacks ()

@property (strong, readwrite) NSRecursiveLock *globalLock;

@property (strong, readwrite) Class<IndyWalletProtocol> IndyKeychainWalletImplementation;

@property (strong, readwrite) Class<IndyWalletProtocol> customWalletImplementation;

@end

@implementation IndyWalletCallbacks

+ (IndyWalletCallbacks *)sharedInstance
{
    static IndyWalletCallbacks *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [IndyWalletCallbacks new];
        instance.globalLock = [NSRecursiveLock new];
        
        instance.IndyKeychainWalletImplementation = [IndyKeychainWallet class];
        
    });
    
    return instance;
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

- (void)retainCString:(char *)cstring
{
    NSValue *value = [NSValue valueWithPointer:cstring];
    
    [self.valuesArray addObject:value];
}

- (void)freeCString:(const char *)cstring
{
    NSValue *value = [NSValue valueWithPointer:cstring];
    [self.valuesArray removeObject:value];
    
    free((void*)cstring);
}

@end

// MARK: - C IndyKeychainWallet callbacks

indy_error_t IndyKeychainWalletCreateCallback(const char* name,
                                          const char* config,
                                          const char* credentials)
{
    NSString *walletName = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *walletConfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *walletCredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] IndyKeychainWalletImplementation];
    
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

indy_error_t IndyKeychainWalletOpenCallback(const char* name,
                                        const char* config,
                                        const char* runtime_config,
                                        const char* credentials,
                                        indy_handle_t* handle)
{
    NSString *walletName = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *walletConfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *walletRuntimeConfig = (runtime_config != NULL) ? [NSString stringWithUTF8String:runtime_config] : nil;
    NSString *walletCredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] IndyKeychainWalletImplementation];
    
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

indy_error_t IndyKeychainWalletSetCallback(indy_handle_t handle,
                                       const char* key,
                                       const char* value)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    NSString *xvalue = (value != NULL) ? [NSString stringWithUTF8String:value] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] IndyKeychainWalletImplementation];
    
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

indy_error_t IndyKeychainWalletGetCallback(indy_handle_t handle,
                                       const char* key,
                                       const char ** const value_ptr)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] IndyKeychainWalletImplementation];
    
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
 
    const char * cstring = [valueString UTF8String];
    
    char * copied = malloc(sizeof(char)*strlen(cstring));
    strcpy(copied, cstring);
    
    [[IndyWalletCallbacks sharedInstance]  retainCString:copied];
    
    *value_ptr = copied;
    
    return Success;
}

indy_error_t IndyKeychainWalletGetNotExpiredCallback(indy_handle_t handle,
                                                 const char* key,
                                                 const char ** const value_ptr)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] IndyKeychainWalletImplementation];
    
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
    
    const char * cstring = [valueString UTF8String];
    
    char * copied = malloc(sizeof(char)*strlen(cstring));
    strcpy(copied, cstring);
    [[IndyWalletCallbacks sharedInstance]  retainCString:copied];
    
    *value_ptr = copied;
    
    return Success;
}

indy_error_t IndyKeychainWalletListCallback(indy_handle_t handle,
                                        const char* key,
                                        const char ** const values_json_ptr)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] IndyKeychainWalletImplementation];
    
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
    
    const char * cstring = [valuesJsonString UTF8String];
    
    char * copied = malloc(sizeof(char)*strlen(cstring));
    strcpy(copied, cstring);
    
    [[IndyWalletCallbacks sharedInstance]  retainCString:copied];
    
    *values_json_ptr = copied;
    
    return Success;
}

indy_error_t IndyKeychainWalletCloseCallback(indy_handle_t handle)
{
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] IndyKeychainWalletImplementation];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    res = [[implementation sharedInstance] close:handle];

    return (indy_error_t)res.code;
}

indy_error_t IndyKeychainWalletDeleteCallback(const char* name,
                                          const char* config,
                                          const char* credentials)
{
    NSString *xname = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *xconfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *xcredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;

    
    Class<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] IndyKeychainWalletImplementation];
    
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

indy_error_t IndyKeychainWalletFreeCallback(indy_handle_t handle, const char* str)
{
    [[IndyWalletCallbacks sharedInstance] freeCString:str];
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
                                     const char ** const value_ptr)
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
    
    const char * cstring = [valueString UTF8String];
    
    char * copied = malloc(sizeof(char)*strlen(cstring));
    strcpy(copied, cstring);
    
    [[IndyWalletCallbacks sharedInstance]  retainCString:copied];
    
    *value_ptr = copied;

    
    return Success;
}

indy_error_t CustomWalletGetNotExpiredCallback(indy_handle_t handle,
                                               const char* key,
                                               const char ** const value_ptr)
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
    
    const char * cstring = [valueString UTF8String];
    
    char * copied = malloc(sizeof(char)*strlen(cstring));
    strcpy(copied, cstring);
    
    [[IndyWalletCallbacks sharedInstance]  retainCString:copied];
    
    *value_ptr = copied;

    
    return Success;
}

indy_error_t CustomWalletListCallback(indy_handle_t handle,
                                      const char* key,
                                      const char ** const values_json_ptr)
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
    
    const char * cstring = [valuesJsonString UTF8String];
    
    char * copied = malloc(sizeof(char)*strlen(cstring));
    strcpy(copied, cstring);
    
    [[IndyWalletCallbacks sharedInstance]  retainCString:copied];
    
    *values_json_ptr = copied;
    
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
    [[IndyWalletCallbacks sharedInstance]  freeCString:str];
    return Success;
}

