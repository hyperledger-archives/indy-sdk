//
//  AgentHighCases.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 22.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"
#import "AgentUtils.h"
#import "WalletUtils.h"
#import "SignusUtils.h"
#import "AnoncredsUtils.h"
#import "NSDictionary+JSON.h"
#import "NSString+Validation.h"
#import "NSArray+JSON.h"

@interface AgentHignCases : XCTestCase

@end

@implementation AgentHignCases

- (void)setUp
{
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown
{
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testAgentListerWorksWithSovrinAgentConnect
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool3"
                                                             walletName:@"wallet3"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed ");

    // 2. create DID
    NSString *did;
    NSString *verKey;
    NSString *pubKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verKey
                                                                    outMyPk:&pubKey];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndStoreMyDidWithWalletHandle() failed ");
    
    // 3. listen
    NSString *endpoint = @"tcp://127.0.0.1:9701";
    
    SovrinHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenWithWalletHandle:walletHandle
                                                     endpoint:endpoint
                                           connectionCallback:nil
                                              messageCallback:nil
                                            outListenerHandle:&listenerHandle];
     XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed ");
    
    [TestUtils cleanupStorage];
}

@end
