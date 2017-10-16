//
//  WalletUtils.m
//  Indy-demo
//

#import "WalletUtils.h"
#import <Indy/Indy.h>
#import "TestUtils.h"

@interface  WalletUtils()

@property (strong, readwrite) NSMutableArray *registeredWalletTypes;
@end

@implementation WalletUtils

+ (WalletUtils *)sharedInstance
{
    static WalletUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [WalletUtils new];
        instance.registeredWalletTypes = [NSMutableArray new];
    });
    
    return instance;
}

- (NSError *)registerWalletType: (NSString *)xtype
{
    NSError *ret;
    
    if ([self.registeredWalletTypes containsObject:xtype])
    {
        return [NSError errorWithDomain:@"IndyErrorDomain" code: WalletTypeAlreadyRegisteredError userInfo:nil];;
    }
    
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    
    ret = [[IndyWallet sharedInstance] registerIndyKeychainWalletType:xtype
                                                           completion:^(NSError* error)
           {
               err = error;
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self.registeredWalletTypes addObject:xtype];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

-(NSError *)createAndOpenWalletWithPoolName:(NSString *) poolName
                                      xtype:(NSString *) xtype
                                     handle:(IndyHandle *) handle
{
    __block NSError *err = nil;
    NSError *ret = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString *walletName = [NSString stringWithFormat:@"default-wallet-name-%lu", (unsigned long)[[SequenceUtils sharedInstance] getNextId]];
    NSString *xTypeStr = (xtype) ? xtype : @"default";
    
    ret = [[IndyWallet sharedInstance] createWalletWithPoolName:  poolName
                                                           name:  walletName
                                                          xType:  xTypeStr
                                                         config:  nil
                                                    credentials:  nil
                                                     completion: ^(NSError* error)
           {
               err = error;
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if( err.code != Success)
    {
        return err;
    }
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block IndyHandle walletHandle = 0;
    
    ret = [[IndyWallet sharedInstance] openWalletWithName:walletName
                                              runtimeConfig:nil
                                                credentials:nil
                                                 completion:^(NSError *error, IndyHandle h)
           {
               err = error;
               walletHandle = h;
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (handle) { *handle = walletHandle; }
    
    return err;
}

- (NSError *)createWalletWithPoolName:(NSString *)poolName
                           walletName:(NSString *)walletName
                                xtype:(NSString *)xtype
                               config:(NSString *)config
{
    __block NSError *err = nil;
    NSError *ret = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[IndyWallet sharedInstance] createWalletWithPoolName:  poolName
                                                             name:  walletName
                                                            xType:  xtype
                                                           config:  config
                                                      credentials:  nil
                                                       completion: ^(NSError *error)
           {
               err = error;
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
}

- (NSError *)deleteWalletWithName:(NSString *)walletName
{
    __block NSError *err;
    NSError *ret = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[IndyWallet sharedInstance] deleteWalletWithName:walletName
                                                credentials:nil
                                                 completion:^(NSError *error)
           {
               err = error;
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)openWalletWithName:(NSString *)walletName
                         config:(NSString *)config
                      outHandle:(IndyHandle *)handle
{
    __block NSError *err;
    __block IndyHandle outHandle = 0;
    NSError *ret = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[IndyWallet sharedInstance] openWalletWithName:walletName
                                              runtimeConfig:config
                                                credentials:nil
                                                 completion:^(NSError *error, IndyHandle h)
           {
               err = error;
               outHandle = h;
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (handle) { *handle = outHandle; }
    return err;
}

- (NSError *)closeWalletWithHandle:(IndyHandle)walletHandle
{
    __block NSError *err;
    NSError *ret = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[IndyWallet sharedInstance] closeWalletWithHandle:walletHandle
                                                    completion:^(NSError *error)
           {
               err = error;
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils shortTimeout]];
    
    return err;
}

@end
