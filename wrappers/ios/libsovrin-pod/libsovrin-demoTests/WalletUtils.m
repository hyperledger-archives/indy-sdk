//
//  WalletUtils.m
//  libsovrin-demo
//

#import "WalletUtils.h"
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"

@implementation WalletUtils

+ (WalletUtils *)sharedInstance
{
    static WalletUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [WalletUtils new];
    });
    
    return instance;
}

-(NSError*) createWallet:(NSString*) poolName
              walletName:(NSString*) walletName
                   xtype:(NSString*) xtype
                  handle:(SovrinHandle*) handle
{
    __block NSError *err = nil;
    NSError *ret = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[SovrinWallet sharedInstance] createWallet:  poolName
                                                 name:  walletName
                                                xType:  xtype
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
    
    ret = [[SovrinWallet sharedInstance] openWallet:  walletName
                                      runtimeConfig:  nil
                                        credentials:  nil
                                         completion: ^(NSError* error, SovrinHandle h)
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
    
    *handle = walletHandle;
    return err;
}

-(NSError*) walletSetSeqNoForValue:(SovrinHandle) walletHandle
                      claimDefUUID:(NSString*) uuid
                     claimDefSeqNo:(NSNumber*) seqNo
{
    __block NSError *err = nil;
    NSError *ret = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [[SovrinWallet sharedInstance] walletSetSeqNo:  seqNo
                                              forHandle:  walletHandle
                                                 andKey:  uuid
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

@end
