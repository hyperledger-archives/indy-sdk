//
//  PoolUtils.h
//  libsovrin-demo
//
//  Created by Kirill Neznamov on 15/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>

@interface PoolUtils : XCTestCase

+ (PoolUtils *)sharedInstance;

- (NSNumber *)getRequestId;

- (NSError*)createPoolLedgerConfig:(NSString *)poolName;

- (NSError*)createAndOpenPoolLedgerConfigWithName: (NSString *) poolName
                                       poolHandle: (SovrinHandle *) handle;

- (NSString *)createPoolConfig:(NSString *)poolName;

- (NSError *)openPoolLedger:(NSString*)poolName
                     config:(NSString*)config
                poolHandler:(SovrinHandle*)handle;

- (NSError *)sendRequestWithPoolHandle:(SovrinHandle)poolHandle
                               request:(NSString *)request
                              response:(NSString **)response;

+ (NSString *) nodeIp;

@end
