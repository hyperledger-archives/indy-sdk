//
//  Base58Test.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 23.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import <Indy/Indy.h>
#import "NSDictionary+JSON.h"
#import <CoreBitcoin+Categories.h>


@interface Base58Test : XCTestCase

@end

@implementation Base58Test

- (void)testDecode
{
    XCTAssertTrue([[@"" dataFromBase58] isEqualToData:[@"" dataUsingEncoding:NSUTF8StringEncoding allowLossyConversion:false]], @"");
    
    XCTAssertTrue([[@"3mJr7AoUXx2Wqd" dataFromBase58] isEqualToData:[@"1234598760" dataUsingEncoding:NSUTF8StringEncoding allowLossyConversion:false]], @"");
    
     XCTAssertTrue([[@"3yxU3u1igY8WkgtjK92fbJQCd4BZiiT1v25f" dataFromBase58] isEqualToData:[@"abcdefghijklmnopqrstuvwxyz" dataUsingEncoding:NSUTF8StringEncoding allowLossyConversion:false]], @"");
}



@end
