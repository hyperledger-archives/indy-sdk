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

/**
 Dictionary of wallet types and objective-c callbacks [type: implementation]
 */

@property (strong, readwrite) NSMutableDictionary *typesAndImplementation;

/**
 Dictionary of wallet types and created wallet handles [type: handle]
 */
@property (strong, readwrite) NSMutableDictionary *handlesAndTypes;

/**
 Dictionary of wallet types and names os created wallets [type: name]
 */
@property (strong, readwrite) NSMutableDictionary *namesAndTypes;

/**
 Dictionary of [pointer: NSString] to prevent system from deallocating values strings from memory
 */
@property (strong, readwrite) NSMutableDictionary *valuesPointers;

@end

@implementation IndyWalletCallbacks

+ (IndyWalletCallbacks *)sharedInstance
{
    static IndyWalletCallbacks *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [IndyWalletCallbacks new];
        instance.handlesAndTypes = [NSMutableDictionary new];
        instance.namesAndTypes = [NSMutableDictionary new];
        instance.valuesPointers = [NSMutableDictionary new];
        instance.globalLock = [NSRecursiveLock new];
        
        instance.typesAndImplementation = [NSMutableDictionary new];
        
    });
    
    return instance;
}


// MARK: - Wallet Type
- (void)addWalletType:(NSString *)type
   withImplementation:(id<IndyWalletProtocol>)implementation
{
    // TODO: - Can user re-register with another implementation?
    @synchronized (self.globalLock)
    {
        if (self.typesAndImplementation[type] != nil)
        {
            self.typesAndImplementation[type] = implementation;
        }
    }
}

- (void)removeWalletType:(NSString *)type
{
    @synchronized (self.globalLock)
    {
        [self.typesAndImplementation removeObjectForKey:type];
    }
}

// Wallet name

- (void)addWalletName:(NSString *)name forRegisteredWalletType:(NSString *)type
{
    @synchronized (self.globalLock)
    {
        self.namesAndTypes[name] = type;
    }

}
- (void)removeWalletName:(NSString *)name
{
    @synchronized (self.globalLock)
    {
        [self.namesAndTypes removeObjectForKey:name];
    }
}

// MARK: - Wallet handle

- (void)addWallethandle:(IndyHandle)handle
     forRegisteredWalletType:(NSString *)type
{
    @synchronized (self.globalLock)
    {
        self.handlesAndTypes[@(handle)] = type;
    }
}

- (void)removeWalletHandle:(IndyHandle)handle
{
    @synchronized (self.globalLock)
    {
        [self.handlesAndTypes removeObjectForKey:@(handle)];
    }
}

// MARK: - Get wallet implementation

- (id<IndyWalletProtocol>)getWalletImplementationByName:(NSString *)name
{
    NSString *type = nil;
    @synchronized (self.globalLock)
    {
        type = self.namesAndTypes[name];
    }
    
    if (type == nil)
    {
        return nil;
    }
    
    id<IndyWalletProtocol> implementation;
    
    @synchronized (self.globalLock)
    {
        implementation = self.typesAndImplementation[type];
    }
    
    return implementation;
}

- (id<IndyWalletProtocol>)getWalletImplementationByHandle:(IndyHandle)handle
{
    NSString *type = nil;
    @synchronized (self.globalLock)
    {
        type = self.handlesAndTypes[@(handle)];
    }
    
    if (type == nil)
    {
        return nil;
    }
    
    id<IndyWalletProtocol> implementation;
    
    @synchronized (self.globalLock)
    {
        implementation = self.typesAndImplementation[type];
    }
    return implementation;
}

/**
 Remove refecence to NSString with pointer
 */
- (void)freeStringWithPointer:(NSValue *)pointer
{
    [self.valuesPointers removeObjectForKey:pointer];
}

- (void)retainString:(NSString *__autoreleasing *)valueString
{
    const char *const * valuePointer = (const char *const *)[valueString UTF8String];
    (const char *const *)[valueString UTF8String]
    NSValue *value = [NSValue valueWithPointer:[stri]];
}


@end

// MARK: - C wallet callbacks

indy_error_t IndyWalletCreateCallback(const char* name,
                                      const char* config,
                                      const char* credentials)
{
    NSString *walletName = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *walletConfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *walletCredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;
    
    id<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] getWalletImplementationByName:walletName];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for name: %@", walletName);
        return WalletUnknownTypeError;
    }
    
    [implementation createWithName:walletName
                            config:walletConfig
                       credentials:walletCredentials];
    return Success;
}

indy_error_t IndyWalletOpenCallback(const char* name,
                                    const char* config,
                                    const char* runtime_config,
                                    const char* credentials,
                                    indy_handle_t* handle)
{
    NSString *walletName = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *walletConfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *walletRuntimeConfig = (runtime_config != NULL) ? [NSString stringWithUTF8String:runtime_config] : nil;
    NSString *walletCredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;
    
    id<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] getWalletImplementationByName:walletName];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for name: %@", walletName);
        return WalletUnknownTypeError;
    }
    
    IndyHandle walletHandle = 0;
    NSError *res;
    res = [implementation openWithName:walletName
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

indy_error_t IndyWalletSetCallback(indy_handle_t handle,
                                   const char* key,
                                   const char* value)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    NSString *xvalue = (value != NULL) ? [NSString stringWithUTF8String:value] : nil;
    
    id<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] getWalletImplementationByHandle:handle];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }

    NSError *res;
    res = [implementation setValue:xvalue
                            forKey:xkey
                        withHandle:handle];
    
    return (indy_error_t)res.code;
}

indy_error_t IndyWalletGetCallback(indy_handle_t handle,
                                   const char* key,
                                   const char *const *value_ptr)
{
    NSString *xkey = (key != NULL) ? [NSString stringWithUTF8String: key] : nil;
    
    id<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] getWalletImplementationByHandle:handle];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for wallet with handle: %i", handle);
        return WalletUnknownTypeError;
    }
    
    NSError *res;
    NSString *valueString = [NSString new];
    res = [implementation getValue:&valueString
                            forKey:xkey
                        withHandle:handle];
 
    value_ptr = (const char *const *)[valueString UTF8String];
    [[IndyWalletCallbacks sharedInstance] retainString:&valueString];
    
    return (indy_error_t)res.code;
}

indy_error_t IndyWalletGetNotExpiredCallback(indy_handle_t handle,
                                             const char* key,
                                             const char *const *value_ptr)
{
    id<IndyWalletProtocol> wallet = [[IndyWalletCallbacks sharedInstance] getWalletImplementationByHandle:handle];
    
    const char *const * my_ptr = NSString s
    
    NSString *str = @"dgdgd";
    
    value_ptr = (const char * const *)[str UTF8String];
    return Success;
}

indy_error_t IndyWalletListCallback(indy_handle_t handle,
                                    const char* key,
                                    const char *const *values_json_ptr)
{
    return Success;
}

indy_error_t IndyWalletCloseCallback(indy_handle_t handle)
{
    return Success;
}

indy_error_t IndyWalletDeleteCallback(const char* name,
                                      const char* config,
                                      const char* credentials)
{
    return Success;
}


indy_error_t IndyWalletFreeCallback(indy_handle_t handle, const char* str)
{
    free((void*)str);
    return Success;
}

