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

@end

@implementation IndyWalletCallbacks

+ (IndyWalletCallbacks *)sharedInstance
{
    static IndyWalletCallbacks *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [IndyWalletCallbacks new];
    });
    
    return instance;
}

- (IndyWalletCallbacks *)init
{
    self = [super init];
    if (self)
    {
        self.typesAndImplementation = [[NSMutableDictionary alloc] init];
        self.handlesAndTypes = [[NSMutableDictionary alloc] init];
        self.namesAndTypes = [[NSMutableDictionary alloc] init];
        self.globalLock = [NSRecursiveLock new];
    }
    return self;
}

// MARK: - Wallet Type
- (void)addWalletType:(NSString *)type
        withImplementation:(id<IndyWalletProtocol>)implementation
{
    @synchronized (self.globalLock)
    {
        self.typesAndImplementation[type] = implementation;
    }
}

- (void)removeWalletType:(NSString *)type
{
    @synchronized (self.globalLock)
    {
        self.typesAndImplementation[type] = nil;
    }
}

// Wallet name

- (void)addWalletName:(NSString *)name
   forRegisteredWalletType:(NSString *)type
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

- (id<IndyWalletProtocol>)getWalletByName:(NSString *)name
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

- (id<IndyWalletProtocol>)getWalletByHandle:(IndyHandle)handle
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

@end

// MARK: - C wallet callbacks

indy_error_t IndyWalletCreateCallback(const char* name,
                                      const char* config,
                                      const char* credentials)
{
    NSString *walletName = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *walletConfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *walletCredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;
    
    id<IndyWalletProtocol> implementation = [[IndyWalletCallbacks sharedInstance] getWalletByName:walletName];
    
    if (implementation == nil)
    {
        NSLog(@"Wallet Implementation not found for name: %@", walletName);
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
    return Success;
}

indy_error_t IndyWalletSetCallback(indy_handle_t handle,
                                   const char* key,
                                   const char* value)
{
    return Success;
}

indy_error_t IndyWalletGetCallback(indy_handle_t handle,
                                   const char* key,
                                   const char *const *value_ptr)
{
    return Success;
}

indy_error_t IndyWalletGetNotExpiredCallback(indy_handle_t handle,
                                             const char* key,
                                             const char *const *value_ptr)
{
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
    return Success;
}

