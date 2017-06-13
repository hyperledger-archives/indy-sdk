//
//  SignusUtils.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 02.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "SignusUtils.h"
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"
#import "WalletUtils.h"

@implementation SignusUtils

+ (SignusUtils *)sharedInstance
{
    static SignusUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^{
        instance = [SignusUtils new];
    });
    
    return instance;
}
- (NSError *) sign:(SovrinHandle)walletHandle
          theirDid:(NSString*)theirDid
           message:(NSString*)message
         signature:(NSString**)signature
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *signSignature = nil;
    NSError *ret;

    
    ret = [SovrinSignus sign:walletHandle
                         did:theirDid
                         msg:message
                  completion:^(NSError *error, NSString *blockSignature)
    {
        err = error;
        signSignature =blockSignature;
        
        [completionExpectation fulfill];
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    *signature = signSignature;
    
    return err;
}


- (NSError *)createMyDidWithWalletHandle:(SovrinHandle)walletHandle
                               myDidJson:(NSString *)myDidJson
                                outMyDid:(NSString **)myDid
                             outMyVerkey:(NSString **)myVerkey
                                 outMyPk:(NSString **)myPk
{
   
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *did = nil;
    __block NSString *verKey = nil;
    __block NSString *pk = nil;
    NSError *ret;

    ret = [SovrinSignus createAndStoreMyDid:walletHandle
                                    didJSON:myDidJson
                                 completion:^(NSError *error, NSString *blockDid, NSString *blockVerKey, NSString *blockPk)
    {
        err = error;
        did = blockDid;
        verKey = blockVerKey;
        pk = blockPk;
        
        [completionExpectation fulfill];
    }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (myDid) { *myDid = did; }
    if (myVerkey){ *myVerkey = verKey; }
    if (myPk) { *myPk = pk; }
    
    return err;
}

- (NSError *)storeTheirDid: (SovrinHandle) walletHandle
              identityJson: (NSString *)identityJson
{
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    NSError *ret;
    
    ret = [SovrinSignus storeTheirDid:walletHandle
                         identityJSON:identityJson
                           completion:^(NSError *error)
    {
        err = error;
        [completionExpectation fulfill];
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    return err;
}

@end
