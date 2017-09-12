//
//  KeychainWallet.m
//  libindy-demo
//

#import "IndyWalletCallbacks.h"
#import "KeychainWallet.h"
#import "NSError+IndyError.h"
#import "KeychainWalletConfig.h"
#import "SequenceUtils.h"
#import "NSString+JSON.h"
#import "libindy-Swift.h"


// MARK: - Indy Keychain Wallet

@interface KeychainWallet ()


// Properties for singleton
@property (strong, readwrite) NSRecursiveLock *globalLock;
@property (strong, readwrite) NSMutableDictionary *handlesDictionary; // dictionary of active [walletHandle: walletItem]
@property (strong, readwrite) NSMutableDictionary *namesAndDictionary; // dictionary of active [walletName: handle]

@end


@implementation KeychainWallet

+ (KeychainWallet *)sharedInstance
{
    static KeychainWallet *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [KeychainWallet new];
        instance.handlesDictionary = [NSMutableDictionary new];
    });
    
    return instance;
}

- (instancetype)initWithName:(NSString *)name
                    poolName:(NSString *)poolName
                      config:(NSString *)config
                 credentials:(NSString *)credentials
{
    self = [super init];
    if (self){
    }
    
    return self;
}

- (NSError *)createWithName:(NSString *)name config:(NSString *)config credentials:(NSString *)credentials
{
    @synchronized (self.globalLock)
    {
        // 1. Fetch all stored wallet names from keychain
        NSArray *walletNames = [KeychainWalletItem allStoredWalletNames];
        
        if ([walletNames containsObject:name])
        {
            return [NSError errorFromIndyError:WalletAlreadyExistsError];
        }
        
        // Create walletItem for wallet to interact with keychain.
        KeychainWalletItem *walletItem = [[KeychainWalletItem alloc] initWithName:name
                                                                           config:config
                                                                      credentials:credentials];
        
        NSError *res;
        [walletItem updateInKeychainAndReturnError:&res];
        
        if ( res.code != Success)
        {
            return res;
        }
    }
    
    return [NSError errorFromIndyError:Success];
}

- (NSError *)openWithName:(NSString *)name
                   config:(NSString *)config
            runtimeConfig:(NSString *)runtimeConfig
              credentials:(NSString *)credentials
                   handle:(IndyHandle *)handle
                outWallet:(__autoreleasing id<IndyWalletProtocol> *)wallet
{
    // 1. Process runtime config
    
    KeychainWalletConfig *parcedRuntimeConfig;;
    if ([runtimeConfig isEqualToString:config])
    {
        parcedRuntimeConfig = [[KeychainWalletConfig alloc] initWithJson:[config toDictionary]];
    }
    else
    {
        parcedRuntimeConfig = [KeychainWalletConfig defaultConfig];
    }
    
    NSArray *walletNames = [KeychainWalletItem allStoredWalletNames];
    if ([walletNames containsObject:name] == false)
    {
        return [NSError errorFromIndyError:CommonInvalidState];
    }
    
    // 2. create & add handle to dictionary. create KeychainWalletItem for this handle
    
    KeychainWalletItem *walletItem = [[KeychainWalletItem alloc] initWithName:name config:config credentials:credentials];
    walletItem.freshnessTime = parcedRuntimeConfig.freshnessTime;
    
    IndyHandle xhandle = (IndyHandle)[[SequenceUtils sharedInstance] getNextId];
    
    self.handlesDictionary[@(xhandle)] = walletItem;
    self.namesAndDictionary[name] = @(xhandle);
    
    if (handle) { *handle = xhandle;}
    
    return [NSError errorFromIndyError:Success];
}


- (NSError *)setValue:(NSString *)value forKey:(NSString *)key withHandle:(IndyHandle)handle
{
    @synchronized (self.globalLock)
    {
        if (self.handlesDictionary[@(handle)] == nil)
        {
            return [NSError errorFromIndyError:CommonInvalidState];
        }
        
        // fetch wallet item to interact with keychain for that wallet
        KeychainWalletItem *walletItem = self.handlesDictionary[@(handle)];

        NSError *res;
        [walletItem setWalletValue:value forKey:key error:&res];
        
        if (res.code != Success)
        {
            return [NSError errorFromIndyError:CommonInvalidState];
        }
    }
    
    return [NSError errorFromIndyError:Success];
}

- (NSError *)getValue:(NSString *__autoreleasing *)value forKey:(NSString *)key withHandle:(IndyHandle)handle
{
    @synchronized (self.globalLock)
    {
        if (self.handlesDictionary[@(handle)] == nil)
        {
            return [NSError errorFromIndyError:CommonInvalidState];
        }
        
        // fetch wallet item to interact with keychain for that wallet
        KeychainWalletItem *walletItem = self.handlesDictionary[@(handle)];
        
        NSString *value = [walletItem getValueForKey:key];
        
        if (value == nil)
        {
            return [NSError errorFromIndyError:WalletNotFoundError];
        }
    }
    
    return [NSError errorFromIndyError:Success];
}

- (NSError *)getNotExpired:(IndyHandle)walletHandle key:(NSString *)key value:(NSString**)value
{
    @synchronized (self.globalLock)
    {
        if (self.handlesDictionary[@(walletHandle)] == nil)
        {
            return [NSError errorFromIndyError:CommonInvalidState];
        }
        
        // fetch wallet item to interact with keychain for that wallet
        KeychainWalletItem *walletItem = self.handlesDictionary[@(walletHandle)];
        
        NSString *valueString = [walletItem getNotExpiredValueForKey:key];
        
        if (valueString == nil)
        {
            return [NSError errorFromIndyError:WalletNotFoundError];
        }
        
        if (value)
        {
            *value = valueString;
        }
    }
    
    return [NSError errorFromIndyError:Success];
}

- (NSError *)list:(IndyHandle)handle key:(NSString *)key valuesJson:(NSString**)valuesJson
{
    @synchronized (self.globalLock)
    {
        if (self.handlesDictionary[@(handle)] == nil)
        {
            return [NSError errorFromIndyError:CommonInvalidState];
        }
        
        KeychainWalletItem *walletItem = self.handlesDictionary[@(handle)];
        
        NSString *valuesJsonList = [walletItem listValuesJsonForKeyPrefix:key];
        
        if (valuesJson)
        {
            *valuesJson = valuesJsonList;
        }
    }
    
    return [NSError errorFromIndyError:Success];
}

- (NSError *)close:(IndyHandle)handle
{
    @synchronized (self.globalLock)
    {
        if (self.handlesDictionary[@(handle)] == nil)
        {
            return [NSError errorFromIndyError:CommonInvalidState];
        }
        
        [self.handlesDictionary removeObjectForKey:@(handle)];
    }
    
    return [NSError errorFromIndyError:Success];
}

- (NSError *)deleteWalletWithName:(NSString *)name config:(NSString *)config credentials:(NSString *)credentials
{
    NSArray *walletNames = [KeychainWalletItem allStoredWalletNames];
    
    if ([walletNames containsObject:name])
    {
        return [NSError errorFromIndyError:WalletAlreadyExistsError];
    }
    return nil;
}

- (NSError *)free:(IndyHandle)handle str:(NSString *)str
{
    return [NSError errorFromIndyError:Success];
}

+ (NSString *)walletTypeName
{
    return @"keychainWallet";
}

- (void) cleanup
{
    [self.handlesDictionary removeAllObjects];
    [self.namesAndDictionary removeAllObjects];

    @synchronized (self.globalLock)
    {
        // 1. Fetch all stored wallet names from keychain
        NSArray *walletNames = [KeychainWalletItem allStoredWalletNames];
        
        for (NSString *name in walletNames)
        {
            KeychainWalletItem *walletItem = [[KeychainWalletItem alloc] initWithName:name
                                                                               config:nil
                                                                          credentials:nil];
            [walletItem deleteFromKeychainAndReturnError:nil];
        }
    }
}

@end
