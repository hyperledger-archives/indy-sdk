//
//  WalletUtils.m
//  libsovrin-demo
//

#import "WalletUtils.h"
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"

@interface  WalletUtils()

@property (nonatomic, strong) NSMutableArray *registeredWallets;
@end

@implementation WalletUtils

+ (WalletUtils *)sharedInstance
{
    static WalletUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [WalletUtils new];
        instance.registeredWallets = [NSMutableArray new];
    });
    
    return instance;
}

// TODO: Implement when architecture is discussed
//- (NSError *)registerWalletType: (NSString *)xtype
//{
//    NSMutableArray *wallets = self.registeredWallets;
//    
//    NSError *ret;
//    if ([wallets containsObject:xtype])
//    {
//        return [NSError new];
//    }
//    
//    
//}

-(NSError*) createAndOpenWalletWithPoolName:(NSString*) poolName
                                      xtype:(NSString*) xtype
                                     handle:(SovrinHandle*) handle
{
    __block NSError *err = nil;
    NSError *ret = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString *walletName = [NSString stringWithFormat:@"default-wallet-name-%lu", (unsigned long)[[SequenceUtils sharedInstance] getNextId]];
    NSString *xTypeStr = (xtype) ? xtype : @"default";
    
    ret = [[SovrinWallet sharedInstance] createWalletWithPoolName:  poolName
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
    __block SovrinHandle walletHandle = 0;
    
    ret = [[SovrinWallet sharedInstance] openWalletWithName:walletName
                                              runtimeConfig:nil
                                                credentials:nil
                                                 completion:^(NSError *error, SovrinHandle h)
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
    
    ret = [[SovrinWallet sharedInstance] createWalletWithPoolName:  poolName
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
    
    ret = [[SovrinWallet sharedInstance] deleteWalletWithName:walletName
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
                      outHandle:(SovrinHandle *)handle
{
    __block NSError *err;
    __block SovrinHandle outHandle = 0;
    NSError *ret = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[SovrinWallet sharedInstance] openWalletWithName:walletName
                                              runtimeConfig:config
                                                credentials:nil
                                                 completion:^(NSError *error, SovrinHandle h)
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

- (NSError *)closeWalletWithHandle:(SovrinHandle)walletHandle
{
    __block NSError *err;
    NSError *ret = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[SovrinWallet sharedInstance] closeWalletWithHandle:walletHandle
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

- (NSError*) walletSetSeqNo:(NSNumber *)seqNo
                   forValue:(NSString *)value
               walletHandle:(SovrinHandle) walletHandle
{
    __block NSError *err = nil;
    NSError *ret = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[SovrinWallet sharedInstance] walletSetSeqNo:seqNo
                                               forValue:value
                                           walletHandle:walletHandle
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


@end
