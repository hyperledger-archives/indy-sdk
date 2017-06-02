//
//  Ledger.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 02.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import <libsovrin/libsovrin.h>
#import "NSDictionary+JSON.h"

@interface Ledger : XCTestCase

@end

@implementation Ledger

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void) testAttributeRequest
{
    NSLog(@"Ledger: testAttributeRequest() started...");
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"pool1";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *res = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    res = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfig:&poolHandle poolName:poolName];
    XCTAssertEqual(res.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    
    SovrinHandle walletHandle = 0;
    res = [[WalletUtils sharedInstance] createAndOpenWallet:poolName
                                                 walletName:walletName
                                                      xtype:xtype
                                                     handle:&walletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils:createAndOpenWallet failed");

    // todo: what is this?
    //let (trustee_did, trustee_verkey, trustee_pk) = get_trustee_keys(wallet_handle);
    
    
}
@end
