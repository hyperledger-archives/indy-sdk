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

- (NSError*)createPoolLedgerConfig:(NSString *)poolName;

- (NSError*)createAndOpenPoolLedgerConfig: (SovrinHandle*) poolHandle
                                 poolName: (NSString *)poolName;

- (NSString *)createPoolConfig:(NSString *)poolName;

- (NSError *)openPoolLedger:(NSString*)poolName
                     config:(NSString*)config
                poolHandler:(SovrinHandle**)handle;

- (NSError *)sendRequest:(SovrinHandle)poolHandle
                 request:(NSString *)request
                response:(NSString **)response;

+ (NSString *) nodeIp;

@end
