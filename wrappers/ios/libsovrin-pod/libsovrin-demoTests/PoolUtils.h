//
//  PoolUtils.h
//  libsovrin-demo
//
//  Created by Kirill Neznamov on 15/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>

@interface PoolUtils : XCTestCase

+ (PoolUtils *)sharedInstance;

- (NSError*)createPoolLedgerConfig:(NSString *)poolName;

@end
