//
//  KeychainWallet.m
//  libindy-demo
//

#import "IndyWalletCallbacks.h"
#import "KeychainWallet.h"
#import "NSError+IndyError.h"


@interface KeychainWallet ()

@property (strong, readwrite) NSRecursiveLock *globalLock;
@property (strong, readwrite) NSMutableArray *walletNames;
@property (strong, readwrite) NSMutableArray *walletHandles;

@end


@implementation KeychainWallet

+ (KeychainWallet *)sharedInstance
{
    static KeychainWallet *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [KeychainWallet new];
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
        if ([self.walletNames containsObject:name])
        {
            return [NSError errorFromIndyError:CommonInvalidState];
        }
        
        [self.walletNames addObject:name];
    }
    
    return [NSError errorFromIndyError:Success];
}


- (NSError *)openWithName:(NSString *)name
                   config:(NSString *)config
            runtimeConfig:(NSString *)runtimeConfig
              credentials:(NSString *)credentials
                   handle:(IndyHandle *)handle
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
