//
//  KeychainWallet.m
//  libindy-demo
//

#import "IndyWalletCallbacks.h"
#import "IndyKeychainWallet.h"
#import "NSError+IndyError.h"
#import "libindy-Swift.h"

@interface IndyKeychainWallet ()

@property (strong, readwrite) NSRecursiveLock *globalLock;

@property (strong, readwrite) NSMutableDictionary *handlesAndNames; // dictionary of active [walletHandle: walletName]
@property (strong, readwrite) NSMutableArray *openedWalletItems; // array of IndyKeychainWalletItem


@end


@implementation IndyKeychainWallet

+ (IndyKeychainWallet *)sharedInstance
{
    static IndyKeychainWallet *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [IndyKeychainWallet new];
        instance.handlesAndNames = [NSMutableDictionary new];
        instance.openedWalletItems = [NSMutableArray new];
    });
    
    return instance;
}

- (NSString *)walletTypeName
{
    return @"keychainWallet";
}

- (NSError *)createWithName:(NSString *)name config:(NSString *)config credentials:(NSString *)credentials
{
    @synchronized (self.globalLock)
    {
        NSArray *walletNames = [IndyKeychainWalletItem allStoredWalletNames];
        
        if ([walletNames containsObject:name])
        {
            return [NSError errorFromIndyError:WalletAlreadyExistsError];
        }
        
        IndyKeychainWalletItem *walletItem = [[IndyKeychainWalletItem alloc] initWithName:name config:config credentials:credentials];
        
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
       
        return [NSError errorFromIndyError:Success];
}

- (NSError *)setValue:(NSString *)value forKey:(NSString *)key withHandle:(IndyHandle)handle
{
    return nil;
}

- (NSError *)getValue:(NSString *__autoreleasing *)value forKey:(NSString *)key withHandle:(IndyHandle)handle
{
    return nil;
}

- (NSError *)getNotExpired:(IndyHandle)walletHandle key:(NSString *)key value:(NSString *)value
{
    return nil;
}

- (NSError *)list:(IndyHandle)handle key:(NSString *)key valuesJson:(NSString *)valuesJson
{
    return nil;
}

- (NSError *)close:(IndyHandle)handle
{
    return nil;
}

- (NSError *)deleteWalletWithName:(NSString *)name config:(NSString *)config credentials:(NSString *)credentials
{
    return nil;
}

- (NSError *)free:(IndyHandle)handle str:(NSString *)str
{
    return nil;
}

@end
